#[allow(dead_code)]
mod vm;
use clap::{App, load_yaml};
use crate::vm::cpu::arch::WAFFLE;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let binary = matches.value_of("BINARY").unwrap();
    let file = matches.value_of("FILE").unwrap();
    let mem = matches.value_of("MEMORY").unwrap();

    let mut cpu = WAFFLE::new(mem.parse::<usize>().unwrap());


}
