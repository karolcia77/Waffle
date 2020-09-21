use crate::vm::cpu::arch::{WAFFLE, Flags, Register};
use crate::vm::instructions::Instructions as INST;

fn fetch(cpu: &mut WAFFLE) {
    cpu.pc+=1;
    cpu.inst = cpu.mem[cpu.pc].into();
}

fn execute(cpu: &mut WAFFLE) {
    let source = cpu.pc + 1;
    let destination = cpu.pc + 9;
    match cpu.inst {
        INST::CLF   => cpu.flags_clear(),
        INST::HALT  => cpu.flag_set(Flags::HALT),
        INST::PSH   => cpu.stack_push(cpu.registers[cpu.mem[cpu.pc]]),
        INST::PSHF  => cpu.stack_push(cpu.fregisters[cpu.mem[cpu.pc]]),
        INST::POP   => {}
        INST::POPF   => {}
        INST::DSPL  => print!("{}",cpu.registers[cpu.dest]),
        INST::DSPLN => println!("{}",cpu.registers[cpu.dest]),

        INST::MOV => {cpu.registers[cpu.memory_read(destination)] = cpu.registers[cpu.memory_read(source)]},
        INST::MOVF => cpu.fregisters[cpu.memory_readf(destination) - 8] = cpu.fregisters[cpu.memory_read(source) - 8],
        INST::LDI => {},
        INST::STI => {},
        INST::LLI => {},
        INST::ADD => {},
        INST::SUB => {},
        INST::MUL => {},
        INST::DIV => {},
        INST::SHR => {},
        INST::SHL => {}


        _ => {},
    };
}


pub fn run(cpu: &mut WAFFLE) {
    while !cpu.flag_check(Flags::HALT) {
        fetch(cpu);
        execute(cpu);
    }
}