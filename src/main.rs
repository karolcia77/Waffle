#[allow(dead_code)]
mod vm;
use clap::{App, load_yaml};
use crate::vm::cpu::arch::{WAFFLE, Register, get_register_idx};
use crate::vm::instructions::Instruction;
use std::convert::TryInto;
use std::any::Any;
use std::process::exit;
use crate::vm::parser::lexer;
use std::path::Path;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    match matches.subcommand() {
        ("with", Some(run_comm)) => {
            let mem: usize = run_comm.
                value_of("MEMORY")
                .unwrap_or_default()
                .parse::<usize>().unwrap();

            let file = run_comm.value_of("SYRUP").expect("A file was not provided. Ask --help");
            let mut cpu = WAFFLE::new(mem);
        },
        ("syrup", Some(compile_comm)) => {
            let file_path = compile_comm.value_of("SOURCE").expect("A source file was not provided. Ask --help");
            let out_path = compile_comm.value_of("OUT").unwrap_or_default();
            let mut cpu = WAFFLE::new(1024);
            println!("LEXING...");
            let lexemes = lexer(cpu, Path::new(file_path));
            println!("DONE");
            println!("SYRUP: {:?}", lexemes);

        },
        _ => {println!("No argument was supplied. Ask --help"); exit(1)},
    }


}
