use std::process::exit;
use std::str::FromStr;
use crate::vm::types::Types;

#[repr(u8)]
#[derive(PartialEq, PartialOrd, Debug)]
pub enum Instruction {
    CLF,                      // Clear CPU Flags [0]
    MOV, MOVF, INC, DEC,      // Register ops [2;1]
    CMP, CMPF, CMPI, CMPFI,   // Compare ops  [2]
    LDI, LDF, STI, STF,       // Memory ops  [2]
    PSH, POP, PSHF, POPF,     // STACK ops
    LLI, LLF,                 // Local Load <Int/Float> [2]
    ADD, SUB, MUL, DIV,       // MATH
    FADD, FSUB, FMUL, FDIV,   // MATH float
    JA, JB, JE, JNE, JMP,     // Conditions
    JZ, JNZ, JNC, JS, JNS,    // Conditions
    JL, JNL, JP, JNP, JC,     // Conditions
    LAND, LOR, LNOT,          // Logical (unary)
    AND, OR, XOR, NOT,        // Binary
    SHL, SHR,                 // Shift
    HALT,                     // Do nothing
    DSPL, DSPLN,              // Display
    STOP
}


impl Instruction {
    pub fn arg_types_map(&self) -> Vec<Types>{
        match self {
            Instruction::STOP => vec![Types::OPERATION],
            Instruction::HALT => vec![Types::OPERATION],
            Instruction::PSH => vec![Types::OPERATION, Types::REGISTER],
            Instruction::POP => vec![Types::OPERATION, Types::REGISTER],
            Instruction::PSHF => vec![Types::OPERATION, Types::REGISTER],
            Instruction::POPF => vec![Types::OPERATION, Types::REGISTER],
            Instruction::DSPL => vec![Types::OPERATION, Types::REGISTER],
            Instruction::DSPLN => vec![Types::OPERATION, Types::REGISTER],
            Instruction::MOV => vec![Types::OPERATION, Types::REGISTER, Types::REGISTER],
            Instruction::LDI => vec![Types::OPERATION, Types::REGISTER, Types::ADDRESS],
            Instruction::STI => vec![Types::OPERATION, Types::ADDRESS, Types::REGISTER],
            Instruction::LLI => vec![Types::OPERATION, Types::REGISTER, Types::NUMBER],
            Instruction::ADD => vec![Types::OPERATION, Types::REGISTER, Types::REGISTER],
            Instruction::SUB => vec![Types::OPERATION, Types::REGISTER, Types::REGISTER],
            Instruction::MUL => vec![Types::OPERATION, Types::REGISTER, Types::REGISTER],
            Instruction::DIV => vec![Types::OPERATION, Types::REGISTER, Types::REGISTER],
            Instruction::SHR => vec![Types::OPERATION, Types::REGISTER, Types::NUMBER],
            Instruction::SHL => vec![Types::OPERATION, Types::REGISTER, Types::NUMBER],
            Instruction::MOVF => vec![Types::OPERATION, Types::REGISTER, Types::REGISTER],
            Instruction::LDF => vec![Types::OPERATION, Types::REGISTER, Types::ADDRESS],
            Instruction::STF => vec![Types::OPERATION, Types::ADDRESS, Types::REGISTER],
            Instruction::LLF => vec![Types::OPERATION, Types::REGISTER, Types::NUMBER],
            Instruction::FADD => vec![Types::OPERATION, Types::REGISTER, Types::REGISTER],
            Instruction::FSUB => vec![Types::OPERATION, Types::REGISTER, Types::REGISTER],
            Instruction::FMUL => vec![Types::OPERATION, Types::REGISTER, Types::REGISTER],
            Instruction::FDIV => vec![Types::OPERATION, Types::REGISTER, Types::REGISTER],
            Instruction::LAND => vec![Types::OPERATION, Types::REGISTER, Types::REGISTER],
            Instruction::LOR => vec![Types::OPERATION, Types::REGISTER, Types::REGISTER],
            Instruction::LNOT => vec![Types::OPERATION, Types::REGISTER, Types::REGISTER],
            Instruction::AND => vec![Types::OPERATION, Types::REGISTER, Types::REGISTER],
            Instruction::OR => vec![Types::OPERATION, Types::REGISTER, Types::REGISTER],
            Instruction::XOR => vec![Types::OPERATION, Types::REGISTER, Types::REGISTER],
            Instruction::NOT => vec![Types::OPERATION, Types::REGISTER],
            Instruction::JMP => vec![Types::OPERATION, Types::ADDRESS],
            Instruction::JA => vec![Types::OPERATION, Types::ADDRESS],
            Instruction::JB => vec![Types::OPERATION, Types::ADDRESS],
            Instruction::JE => vec![Types::OPERATION, Types::ADDRESS],
            Instruction::JNE => vec![Types::OPERATION, Types::ADDRESS],
            Instruction::JZ => vec![Types::OPERATION, Types::ADDRESS],
            Instruction::JNZ => vec![Types::OPERATION, Types::ADDRESS],
            Instruction::JC => vec![Types::OPERATION, Types::ADDRESS],
            Instruction::JNC => vec![Types::OPERATION, Types::ADDRESS],
            Instruction::JS => vec![Types::OPERATION, Types::ADDRESS],
            Instruction::JNS => vec![Types::OPERATION, Types::ADDRESS],
            Instruction::JL => vec![Types::OPERATION, Types::ADDRESS],
            Instruction::JNL => vec![Types::OPERATION, Types::ADDRESS],
            Instruction::JP => vec![Types::OPERATION, Types::ADDRESS],
            Instruction::JNP => vec![Types::OPERATION, Types::ADDRESS],
            _ => exit(42),
        }
    }
}


impl FromStr for Instruction {
    type Err = ();
    fn from_str(orig: &str) -> Result<Instruction, ()> {
        let op = match orig {
            "STOP" => Instruction::STOP,
            "HALT" => Instruction::HALT,
            "PSH" => Instruction::PSH,
            "POP" => Instruction::POP,
            "PSHF" => Instruction::PSHF,
            "POPF" => Instruction::POPF,
            "DSPL" => Instruction::DSPL,
            "DSPLN" => Instruction::DSPLN,
            // INT
            "MOV" => Instruction::MOV,
            "LDI" => Instruction::LDI,
            "STI" => Instruction::STI,
            "LLI" => Instruction::LLI,
            "ADD" => Instruction::ADD,
            "SUB" => Instruction::SUB,
            "MUL" => Instruction::MUL,
            "DIV" => Instruction::DIV,
            "SHR" => Instruction::SHR,
            "SHL" => Instruction::SHL,
            // FLOAT
            "MOVF" => Instruction::MOVF,
            "LDF" => Instruction::LDF,
            "STF" => Instruction::STF,
            "LLF" => Instruction::LLF,
            "FADD" => Instruction::FADD,
            "FSUB" => Instruction::FSUB,
            "FMUL" => Instruction::FMUL,
            "FDIV" => Instruction::FDIV,
            // LOGIC & BINARY
            "LAND" => Instruction::LAND,
            "LOR" => Instruction::LOR,
            "LNOT" => Instruction::LNOT,
            "AND" => Instruction::AND,
            "OR" => Instruction::OR,
            "XOR" => Instruction::XOR,
            "NOT" => Instruction::NOT,
            // CONDITION
            "JMP" => Instruction::JMP,
            "JA" => Instruction::JA,
            "JB" => Instruction::JB,
            "JE" => Instruction::JE,
            "JNE" => Instruction::JNE,
            "JZ" => Instruction::JZ,
            "JNZ" => Instruction::JNZ,
            "JC" => Instruction::JC,
            "JNC" => Instruction::JNC,
            "JS" => Instruction::JS,
            "JNS" => Instruction::JNS,
            "JL" => Instruction::JL,
            "JNL" => Instruction::JNL,
            "JP" => Instruction::JP,
            "JNP" => Instruction::JNP,
            // INVALID OPCODES
            _ => exit(42)
        };
        Ok(op)
    }
}

impl Into<u8> for Instruction {
    fn into(self) -> u8 {
        match self {
            Instruction::STOP => 0x00,
            Instruction::HALT => 0x01,
            Instruction::PSH => 0x02,
            Instruction::POP => 0x03,
            Instruction::PSHF => 0x04,
            Instruction::POPF => 0x05,
            Instruction::DSPL => 0x06,
            Instruction::DSPLN =>0x07,
            Instruction::MOV => 0x10,
            Instruction::LDI => 0x11,
            Instruction::STI => 0x12,
            Instruction::LLI => 0x13,
            Instruction::ADD => 0x14,
            Instruction::SUB => 0x15,
            Instruction::MUL => 0x16,
            Instruction::DIV => 0x17,
            Instruction::SHR => 0x18,
            Instruction::SHL => 0x19,
            Instruction::MOVF =>0x20,
            Instruction::LDF => 0x21,
            Instruction::STF => 0x22,
            Instruction::LLF => 0x23,
            Instruction::FADD =>0x24,
            Instruction::FSUB =>0x25,
            Instruction::FMUL =>0x26,
            Instruction::FDIV =>0x27,
            Instruction::LAND =>0x30,
            Instruction::LOR => 0x31,
            Instruction::LNOT =>0x32,
            Instruction::AND => 0x33,
            Instruction::OR => 0x34,
            Instruction::XOR => 0x35,
            Instruction::NOT => 0x36,
            Instruction::JMP =>0x40,
            Instruction::JA => 0x41,
            Instruction::JB => 0x42,
            Instruction::JE => 0x43,
            Instruction::JNE =>0x44,
            Instruction::JZ => 0x45,
            Instruction::JNZ =>0x46,
            Instruction::JC => 0x47,
            Instruction::JNC =>0x48,
            Instruction::JS => 0x49,
            Instruction::JNS =>0x4A,
            Instruction::JL => 0x4B,
            Instruction::JNL =>0x4C,
            Instruction::JP => 0x4D,
            Instruction::JNP =>0x4E,
            _ => exit(42),
        }
    }
}


impl From<u8> for Instruction {
    fn from(orig: u8) -> Self {
        match orig {
            0x00 => Instruction::STOP,
            0x01 => Instruction::HALT,
            0x02 => Instruction::PSH,
            0x03 => Instruction::POP,
            0x04 => Instruction::PSHF,
            0x05 => Instruction::POPF,
            0x06 => Instruction::DSPL,
            0x07 => Instruction::DSPLN,
            // INT
            0x10 => Instruction::MOV,
            0x11 => Instruction::LDI,
            0x12 => Instruction::STI,
            0x13 => Instruction::LLI,
            0x14 => Instruction::ADD,
            0x15 => Instruction::SUB,
            0x16 => Instruction::MUL,
            0x17 => Instruction::DIV,
            0x18 => Instruction::SHR,
            0x19 => Instruction::SHL,
            // FLOAT
            0x20 => Instruction::MOVF,
            0x21 => Instruction::LDF,
            0x22 => Instruction::STF,
            0x23 => Instruction::LLF,
            0x24 => Instruction::FADD,
            0x25 => Instruction::FSUB,
            0x26 => Instruction::FMUL,
            0x27 => Instruction::FDIV,
            // LOGIC & BINARY
            0x30 => Instruction::LAND,
            0x31 => Instruction::LOR,
            0x32 => Instruction::LNOT,
            0x33 => Instruction::AND,
            0x34 => Instruction::OR,
            0x35 => Instruction::XOR,
            0x36 => Instruction::NOT,
            // CONDITION
            0x40 => Instruction::JMP,
            0x41 => Instruction::JA,
            0x42 => Instruction::JB,
            0x43 => Instruction::JE,
            0x44 => Instruction::JNE,
            0x45 => Instruction::JZ,
            0x46 => Instruction::JNZ,
            0x47 => Instruction::JC,
            0x48 => Instruction::JNC,
            0x49 => Instruction::JS,
            0x4A => Instruction::JNS,
            0x4B => Instruction::JL,
            0x4C => Instruction::JNL,
            0x4D => Instruction::JP,
            0x4E => Instruction::JNP,
            // INVALID OPCODES
            _ => exit(42)
        }
    }
}