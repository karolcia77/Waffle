#[allow(dead_code)]
mod vm;
use clap::{App, load_yaml};
use crate::vm::cpu::arch::{WAFFLE, Register, get_register_idx};
use crate::vm::instructions::Instruction;
use std::convert::TryInto;
use std::any::Any;
use std::process::exit;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    match matches.subcommand() {
        ("run", Some(run_comm)) => {
            let mem: usize = run_comm.
                value_of("MEMORY")
                .unwrap_or("1024")
                .parse::<usize>().unwrap();

            let file = run_comm.value_of("FILE").expect("A file was not provided. Ask --help");
            let mut cpu = WAFFLE::new(mem);
        },
        ("compile", Some(compile_comm)) => {},
        _ => {println!("No argument was supplied. Ask --help"); exit(1)},
    }


}
