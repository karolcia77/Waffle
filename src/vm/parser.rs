use std::fs;
use std::io::{BufRead, self};
use std::path::Path;
use std::fs::{File, read_to_string};
use crate::vm::instructions::Instruction;
use crate::vm::types::{Lexeme, Types};
use crate::vm::cpu::arch::Register;
use std::process::exit;

pub fn read_source_file(filename: &Path) -> io::Lines<io::BufReader<File>> {
    let file = File::open(filename).unwrap();
    io::BufReader::new(file).lines()
}

pub fn lexer(filename: &Path) {
    let mut lexemes: Vec<Lexeme> = Vec::new();
    for (num, line) in read_source_file(filename).enumerate() {
        let types_map: Vec<Types>;
        for (idx, op) in line.unwrap().split_whitespace().enumerate() {
            if idx == 0 {
                let operation = op.parse::<Instruction>().unwrap();
                types_map = operation.arg_types_map();
                operation.into();
                lexemes.push(Lexeme::new(*types_map.get(0).unwrap(),
                                         vec![operation.into()],
                                         num*idx));
            } else {
                let arg_type = types_map
                    .get(idx)
                    .expect(&format!("Wrong number of arguments at col:{} row:{}", num, idx))
                let value = match arg_type {
                    Types::REGISTER => op.parse::<Register>().unwrap(),
                    Types::ADDRESS => op.parse::<usize>(),
                    Types::NUMBER => op.parse::<i64>(),
                    Types::DECIMAL => op.parse::<f64>(),
                    Types::STRING => op,
                    _ => exit(52),
                };
                value.
                Lexeme::new(arg_type, )
            }
        }
        println!("INSTRUCTION: {}", inst);

    }
}


pub fn compile(filename: &Path) {
    let operation: Instruction;
    for (num, line) in read_source_file(filename).enumerate() {
        for (idx, op) in line.unwrap().split_whitespace().enumerate() {
            if idx == 0 {
                operation = ln.0.as_str().parse::<Instruction>();
            } else {
                
            }
            
        }
        println!("INSTRUCTION: {}", inst);

    }
}