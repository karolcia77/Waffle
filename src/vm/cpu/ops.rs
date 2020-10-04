use crate::vm::cpu::arch::{WAFFLE};
use crate::vm::instructions::Instruction as INST;


fn fetch(cpu: &mut WAFFLE) {
    cpu.inst = cpu.mem[cpu.pc].into();
    // println!("{:x}@{:?} {:?}", cpu.pc, cpu.mem[cpu.pc], cpu.inst);
}


fn execute(cpu: &mut WAFFLE) {
    let mut source = 0;
    let mut destination = 0;
    let mut shift = 1;
    let args = cpu.inst.arg_types_map();
    if args.len() > 1 {
        destination = cpu.pc + 1;
        shift += args[1].bytes_needed();
    }
    if args.len() > 2 {
        source = cpu.pc + args[1].bytes_needed();
        shift += args[2].bytes_needed();
    }
    cpu.pc += shift;
    match cpu.inst {
        INST::STOP => {println!("Received STOP SIGNAL")},
        INST::CLF   => {
            cpu.flags_clear();
        },
        INST::HALT  => {},
        INST::PSH   => cpu.stack_push(cpu.register_read(destination)),
        INST::PSHF  => cpu.stack_pushf(cpu.register_readf(destination)),
        INST::POP   => cpu.register_write(destination, cpu.memory_read(cpu.sp)),
        INST::POPF  => cpu.register_writef(destination, cpu.memory_readf(cpu.sp)),
        INST::DSPL  => {
            if destination > 7 {
                print!("{}", cpu.register_readf(destination));
            } else {
                print!("{}", cpu.register_read(destination));
            }
        },
        INST::DSPLN => {
            if destination > 7 {
                println!("{}", cpu.register_readf(destination));
            } else {
                println!("{}", cpu.register_read(destination));
            }
        },
        INST::INC => {
            if destination > 7 {
                cpu.register_writef(destination, cpu.register_readf(destination) + 1.0);
            } else {
                cpu.register_write(destination, cpu.register_read(destination) + 1);
            }
        },

        INST::DEC => {
            if destination > 7 {
                cpu.register_writef(destination, cpu.register_readf(destination) - 1.0);
            } else {
                cpu.register_write(destination, cpu.register_read(destination) + 1);
            }
        }

        // INTEGER OPS
        INST::MOV => cpu.register_write(destination, cpu.register_read(source)),
        INST::LDI => cpu.register_write(destination, cpu.memory_read(source)),
        INST::STI => cpu.memory_write(destination, cpu.register_read(source)),
        INST::LLI => cpu.register_write(destination, cpu.memory_read(source)),
        INST::ADD => cpu.register_write(destination, cpu.register_read(destination) + cpu.register_read(source)),
        INST::SUB => cpu.register_write(destination, cpu.register_read(destination) - cpu.register_read(source)),
        INST::MUL => cpu.register_write(destination, cpu.register_read(destination) * cpu.register_read(source)),
        INST::DIV => cpu.register_write(destination, cpu.register_read(destination) / cpu.register_read(source)),
        INST::SHR => cpu.register_write(destination, cpu.register_read(destination) >> cpu.memory_read(source)),
        INST::SHL => cpu.register_write(destination, cpu.register_read(destination) << cpu.memory_read(source)),

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