#[allow(dead_code)]
mod vm;
use clap::{App, load_yaml};
use crate::vm::cpu::arch::WAFFLE;
use crate::vm::cpu::ops::run;
use std::fs::File;
use std::process::exit;
use crate::vm::parser::{lexer, compiler, consume_syrup, read_as_byte_vec, generate_source};
use std::path::Path;
use std::io::Write;

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
            read_as_byte_vec(file, &mut cpu.mem);
            run(&mut cpu);
        },
        ("titbits", Some(run_comm)) => {
            let file = run_comm.value_of("SYRUP").expect("A file was not provided. Ask --help");
            let fout = run_comm.value_of("OUT").expect("A file was not provided. Ask --help");
            print!("Reversing...");
            let lexemes = consume_syrup(Path::new(file));
            println!("Done");
            let mut cpu = WAFFLE::new(1024);
            print!("Writing into {} ...", fout);
            generate_source(&cpu, Path::new(fout), lexemes);
            println!("Done");
        }
        ("syrup", Some(compile_comm)) => {
            let file_path = compile_comm.value_of("SOURCE").expect("A source file was not provided. Ask --help");
            let out_path = compile_comm.value_of("OUT").unwrap_or_default();
            let mut cpu = WAFFLE::new(1024);
            print!("LEXING...");
            let lexemes = lexer(&cpu, Path::new(file_path));
            println!("DONE");

            print!("COMPILING...");
            let bc = compiler(&lexemes);
            println!("DONE");
            print!("Writing into {} ...", out_path);
            let mut out = File::create(out_path).expect(&format!("Error: Could not create a file {}", out_path));
            out.write_all(bc.as_slice()).expect("Error: Failed to write bytecode to the file");
            println!("DONE");
        },
        _ => {println!("No argument was supplied. Ask --help"); exit(1)},
    }


}
