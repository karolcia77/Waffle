use crate::vm::cpu::arch::{WAFFLE};
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
        INST::HALT  => {},
        INST::PSH   => cpu.stack_push(cpu.registers[cpu.memory_read(destination) as usize]),
        INST::PSHF  => cpu.stack_pushf(cpu.fregisters[cpu.memory_read(destination) as usize -8]),
        INST::POP   => {}
        INST::POPF   => {}
        INST::DSPL  => print!("{}",cpu.registers[cpu.memory_read(destination) as usize]),
        INST::DSPLN => println!("{}",cpu.registers[cpu.memory_read(destination) as usize]),

        INST::MOV => {cpu.registers[cpu.memory_read(destination) as usize] = cpu.registers[cpu.memory_read(source) as usize]},
        INST::MOVF => cpu.fregisters[cpu.memory_read(destination) as usize - 8] = cpu.fregisters[cpu.memory_read(source) as usize - 8],
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
    while cpu.inst != INST::STOP {
        fetch(cpu);
        execute(cpu);
    }
}