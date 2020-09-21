use std::process::exit;

#[derive(PartialEq, PartialOrd, Debug)]
pub enum Instructions {
    CLF,                      // Clear CPU Flags
    MOV, MOVF, INC, DEC,      // Register ops
    CMP, CMPF, CMPI, CMPFI,   // Compare ops
    LDI, LDF, STI, STF,       // Memory ops
    PSH, POP, PSHF, POPF,     // STACK ops
    LLI, LLF,                 // Local Load <Int/Float>
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

impl From<u8> for Instructions {
    fn from(orig: u8) -> Self {
        match orig {
            0x00 => Instructions::STOP,
            0x01 => Instructions::HALT,
            0x02 => Instructions::PSH,
            0x03 => Instructions::POP,
            0x04 => Instructions::PSHF,
            0x05 => Instructions::POPF,
            0x06 => Instructions::DSPL,
            0x07 => Instructions::DSPLN,
            // INT
            0x10 => Instructions::MOV,
            0x11 => Instructions::LDI,
            0x12 => Instructions::STI,
            0x13 => Instructions::LLI,
            0x14 => Instructions::ADD,
            0x15 => Instructions::SUB,
            0x16 => Instructions::MUL,
            0x17 => Instructions::DIV,
            0x18 => Instructions::SHR,
            0x19 => Instructions::SHL,
            // FLOAT
            0x20 => Instructions::MOVF,
            0x21 => Instructions::LDF,
            0x22 => Instructions::STF,
            0x23 => Instructions::LLF,
            0x24 => Instructions::FADD,
            0x25 => Instructions::FSUB,
            0x26 => Instructions::FMUL,
            0x27 => Instructions::FDIV,
            // LOGIC & BINARY
            0x30 => Instructions::LAND,
            0x31 => Instructions::LOR,
            0x32 => Instructions::LNOT,
            0x33 => Instructions::AND,
            0x34 => Instructions::OR,
            0x35 => Instructions::XOR,
            0x36 => Instructions::NOT,
            // CONDITION
            0x40 => Instructions::JMP,
            0x41 => Instructions::JA,
            0x42 => Instructions::JB,
            0x43 => Instructions::JE,
            0x44 => Instructions::JNE,
            0x45 => Instructions::JZ,
            0x46 => Instructions::JNZ,
            0x47 => Instructions::JC,
            0x48 => Instructions::JNC,
            0x49 => Instructions::JS,
            0x4A => Instructions::JNS,
            0x4B => Instructions::JL,
            0x4C => Instructions::JNL,
            0x4D => Instructions::JP,
            0x4E => Instructions::JNP,
            // INVALID OPCODES
            _ => exit(42)
        }
    }
}