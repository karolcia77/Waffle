use crate::vm::cpu::arch::{WAFFLE};
use crate::vm::instructions::Instruction as INST;

fn fetch(cpu: &mut WAFFLE) {
    cpu.pc+=1;
    cpu.inst = cpu.mem[cpu.pc].into();
}

fn execute(cpu: &mut WAFFLE) {
    let destination = cpu.pc + 1;
    let source = cpu.pc + 9;
    match cpu.inst {
        INST::CLF   => {
            cpu.flags_clear();
            cpu.pc += 1;
        },
        INST::HALT  => {},
        INST::PSH   => {
            cpu.stack_push(cpu.registers[cpu.memory_read(destination) as usize]);
            cpu.pc += 8;
        },
        INST::PSHF  => {
            cpu.stack_pushf(cpu.fregisters[cpu.memory_read(destination) as usize -8]);
            cpu.pc += 8;
        },
        INST::POP   => {
            cpu.register_write(cpu.memory_read(source) as usize, cpu.memory_read(cpu.sp))
        },
        INST::POPF  => {
            cpu.register_writef(cpu.memory_read(source) as usize, cpu.memory_readf(cpu.sp))
        },
        INST::DSPL  => {
            print!("{}",cpu.registers[cpu.memory_read(destination) as usize])
        },
        INST::DSPLN => {
            println!("{}",cpu.registers[cpu.memory_read(destination) as usize])
        },

        // INTEGER OPS
        INST::MOV => {
            cpu.registers[cpu.memory_read(destination) as usize] =
                cpu.registers[cpu.memory_read(source) as usize]
        },
        INST::LDI => {},
        INST::STI => {},
        INST::LLI => cpu.register_write(cpu.memory_read(destination) as usize, cpu.memory_read(source)),
        INST::ADD => cpu.register_write(cpu.registers[]),
        INST::SUB => {},
        INST::MUL => {},
        INST::DIV => {},
        INST::SHR => {},
        INST::SHL => {},

        // FLOAT OPS
        INST::MOVF => {
            cpu.fregisters[cpu.memory_read(destination) as usize - 8] =
                cpu.fregisters[cpu.memory_read(source) as usize - 8]
        },

        // LOGIC OPS
        INST::OR  => cpu.registers[cpu.memory_read(destination) as usize] |=
            cpu.registers[cpu.memory_read(source) as usize],

        _ => {},
    };
}


pub fn run(cpu: &mut WAFFLE) {
    while cpu.inst != INST::STOP {
        fetch(cpu);
        execute(cpu);
    }
}