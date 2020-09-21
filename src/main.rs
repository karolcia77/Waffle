#[allow(dead_code)]
mod vm;
use clap::{App, load_yaml};
use vm::cpu::arch::WAFFLE;
use vm::cpu::ops::run;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    println!("{:#?}", matches);
    let cpu = WAFFLE::new(1024);
    vec![0x13, , , , 0x0]
}
