use std::io::{BufRead, self};
use std::path::Path;
use std::fs::{File};
use crate::vm::instructions::Instruction;
use crate::vm::types::{Lexeme, Types};
use crate::vm::cpu::arch::{Register, WAFFLE, get_register_idx};
use std::process::exit;


pub fn read_source_file(filename: &Path) -> io::Lines<io::BufReader<File>> {
    let file = File::open(filename).unwrap();
    io::BufReader::new(file).lines()
}

pub fn lexer(cpu: WAFFLE, filename: &Path) -> Vec<Lexeme> {
    let mut lexemes: Vec<Lexeme> = Vec::new();
    for (num, line) in read_source_file(filename).enumerate() {
        let mut types_map: Vec<Types> = Vec::with_capacity(3);
        for (idx, op) in line.unwrap().split_whitespace().enumerate() {
            if idx == 0 {
                let operation = op.parse::<Instruction>().unwrap();
                types_map = operation.arg_types_map();
                lexemes.push(Lexeme::new(*types_map.get(0).unwrap(),
                                         vec![operation.into()],
                                         num*idx));
            } else {

                let arg_type = types_map
                    .get(idx)
                    .expect(&format!("\nERROR: Wrong number of arguments at {:?}:{}:{}", filename, num, idx));
                let value = match arg_type {
                    Types::REGISTER => vec![get_register_idx(&op.parse::<Register>().unwrap())],
                    Types::ADDRESS => Vec::from(cpu.cast_to_bytesz(op.parse::<usize>().unwrap())),
                    Types::NUMBER => Vec::from(cpu.cast_to_bytes(op.parse::<i64>().unwrap())),
                    Types::DECIMAL => Vec::from(cpu.cast_to_bytesf(op.parse::<f64>().unwrap())),
                    Types::STRING => Vec::from(op.as_bytes()),
                    _ => {
                        println!("\nERROR: at {:?}:{}:{}", filename, num, idx);
                        exit(52)},
                };
                lexemes.push(Lexeme::new(*arg_type, value, num*idx));
            }
        }
    }
    lexemes
}
