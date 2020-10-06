use std::io::{BufRead, self, Read, LineWriter, Write};
use std::path::Path;
use std::fs;
use crate::vm::instructions::Instruction;
use crate::vm::types::{Lexeme, Types};
use crate::vm::cpu::arch::{Register, WAFFLE, get_register_idx};
use std::process::exit;


pub fn read_source_file(filename: &Path) -> io::Lines<io::BufReader<fs::File>> {
    let file = fs::File::open(filename).unwrap();
    io::BufReader::new(file).lines()
}

pub fn generate_source(cpu: &WAFFLE, filename: &Path, lexemes: Vec<Lexeme>) {
    let file = fs::File::create(filename).unwrap_or_else(|_| panic!("Error: Failed to create file {}", filename.to_str().unwrap()));
    let mut file = LineWriter::new(file);
    let mut line = String::new();
    for lex in lexemes {
        if !line.is_empty() && lex.type_info == Types::OPERATION {
            file.write_all(line.as_bytes()).unwrap();
            file.write_all(b"\n").unwrap();
            line = String::new();
        }
        match lex.type_info {
            Types::OPERATION => line += &lex.instruction.to_string(),
            Types::REGISTER => line += &Register::from(lex.value[0]).to_string(),
            Types::INTEGER|Types::ADDRESS => line += &cpu.cast_from_bytes(lex.value_as_array()).to_string(),
            Types::DECIMAL => line += &cpu.cast_from_bytesf(lex.value_as_array()).to_string(),
            _ => {}
        }
        line += " ";
    }
}


pub fn read_as_byte_vec(filename: &str, buffer: &mut Vec<u8>) {
    let mut f = fs::File::open(&filename).unwrap_or_else(|_| panic!("Error: Failed to load file {}", filename));
    f.read_exact(buffer).expect("Error: Buffer overflow! Consider increasing the memory size");
}


pub fn compiler(lexemes: &[Lexeme]) -> Vec<u8> {
    let mut byte_code:Vec<u8> = Vec::new();
    for op in lexemes {
        byte_code.extend(&op.value);
    }
    byte_code
}


pub fn consume_syrup(filename: &Path) -> Vec<Lexeme> {
    let mut lexemes: Vec<Lexeme> = Vec::new();
    let mut file = fs::File::open(filename).unwrap_or_else(|_| panic!("Error: Could not open file {}", filename.to_str().unwrap()));
    let mut buffer = [0u8; 8];
    let mut current_types = vec![Types::OPERATION];
    let mut current_map_pos = 0;
    let mut instruction = Instruction::HALT;
    let mut read_size = 0;
    let size = file.metadata().unwrap().len();
    while read_size<=size {
        let current_type = *current_types.get(current_map_pos)
            .unwrap_or_else(|| panic!("Error: Failed to get Map of the operands for {:?}", instruction));
        match current_type {
            Types::OPERATION => {
                (&mut file).take(1).read_exact(&mut buffer[..1]).unwrap();
                instruction = Instruction::from(buffer[0]);
                lexemes.push(Lexeme::new(current_type, instruction,vec![buffer[0]]));
                current_types = instruction.arg_types_map();
                current_map_pos += 1;
                read_size += 1;
            },
            Types::REGISTER => {
                (&mut file).take(1).read_exact(&mut buffer[..1]).unwrap();
                lexemes.push(Lexeme::new(current_type, instruction,vec![buffer[0]]));
                current_map_pos += 1;
                read_size += 1;
            },
            Types::DECIMAL|Types::INTEGER |Types::ADDRESS => {
                (&mut file).take(8).read_exact(&mut buffer).unwrap();
                lexemes.push(Lexeme::new(current_type, instruction,buffer.to_vec()));
                current_map_pos += 1;
                read_size += 8;
            }
            _ => break
        }
        // RESET THE OPERATION READER
        if current_map_pos == current_types.len() {
            current_map_pos = 0;
            current_types[0] = Types::OPERATION
        }
    }
    lexemes
}


pub fn lexer(cpu: &WAFFLE, filename: &Path) -> Vec<Lexeme> {
    let mut lexemes: Vec<Lexeme> = Vec::new();
    for (num, line) in read_source_file(filename).enumerate() {
        let mut types_map: Vec<Types> = Vec::with_capacity(3);
        let mut operation = Instruction::HALT;
        for (idx, op) in line.unwrap().split_whitespace().enumerate() {
            if op.starts_with('#') { // It's a comment at the end of a line.
                break  // Skip the rest of a line.
            }
            if idx == 0 {
                operation = op.parse::<Instruction>().unwrap();
                types_map = operation.arg_types_map();
                lexemes.push(Lexeme::new(*types_map.get(0).unwrap(), operation,
                                         vec![operation.into()]));
            } else {
                let arg_type = types_map
                    .get(idx)
                    .unwrap_or_else(|| panic!("\nERROR: Wrong number of arguments at {:?}:{}:{}", filename, num+1, idx+1));
                let value = match arg_type {
                    Types::REGISTER => vec![get_register_idx(&op.parse::<Register>().unwrap())],
                    Types::ADDRESS => Vec::from(cpu.cast_to_bytesz(op.parse::<usize>().unwrap())),
                    Types::INTEGER => Vec::from(cpu.cast_to_bytes(op.parse::<i64>().unwrap())),
                    Types::DECIMAL => Vec::from(cpu.cast_to_bytesf(op.parse::<f64>().unwrap())),
                    Types::STRING => Vec::from(op.as_bytes()),
                    _ => {
                        println!("\nERROR: at {:?}:{}:{}", filename, num+1, idx+1);
                        exit(52)},
                };
                lexemes.push(Lexeme::new(*arg_type, operation, value));
            }
        }
    }
    lexemes
}
