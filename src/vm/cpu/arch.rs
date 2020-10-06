use crate::vm::instructions::Instruction;
use std::str::FromStr;
use std::convert::From;
use std::process::exit;

#[derive(Copy, Clone, Debug)]
pub enum Register {
    AX, BX, CX, DX, EX, FX, GX, HX,
    FAX, FBX, FCX, FDX, FEX, FFX, FGX, FHX,
}

impl std::fmt::Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FromStr for Register {
    type Err = ();
    fn from_str(orig: &str) -> Result<Register, ()> {
        let reg = match orig {
            "AX" => Register::AX,
            "BX" => Register::BX,
            "CX" => Register::CX,
            "DX" => Register::DX,
            "EX" => Register::EX,
            "FX" => Register::FX,
            "GX" => Register::GX,
            "HX" => Register::HX,
            "FAX" => Register::FAX,
            "FBX" => Register::FBX,
            "FCX" => Register::FCX,
            "FDX" => Register::FDX,
            "FEX" => Register::FEX,
            "FFX" => Register::FFX,
            "FGX" => Register::FGX,
            "FHX" => Register::FHX,
            _ => exit(42),
        };
        Ok(reg)
    }
}

impl From<u8> for Register {
    fn from(orig: u8) -> Self {
        match orig {
            0x00 => Register::AX,
            0x01 => Register::BX,
            0x02 => Register::CX,
            0x03 => Register::DX,
            0x04 => Register::EX,
            0x05 => Register::FX,
            0x06 => Register::GX,
            0x07 => Register::HX,
            0x08 => Register::FAX,
            0x09 => Register::FBX,
            0x0a => Register::FCX,
            0x0b => Register::FDX,
            0x0c => Register::FEX,
            0x0d => Register::FFX,
            0x0e => Register::FGX,
            0x0f => Register::FHX,
            _ => exit(42),
        }
    }
}


pub fn get_register_idx(reg: &Register) -> u8 {
    *reg as u8
}


pub enum Flags {
    ZERO   = 1,
    CARRY  = 2,
    SIGN   = 4,
    OVRFLW = 8,
    PARITY = 16
}

impl Flags {
    fn binary(self) -> u8 {
        self as u8
    }
}


pub struct WAFFLE {
    pub mem: Vec<u8>,
    pub mem_size: usize,
    pub pc: usize,
    pub sp: usize,
    pub registers: [i64; 8],
    pub fregisters: [f64; 8],
    flags: u8,
    pub inst: Instruction,
    pub _platform_big_endian: bool
}

fn is_big_endian() -> bool {
    let x: u32 = 0x1;
    if (x>>24)&0xff == 1{
        false
    } else {
        true
    }
}


impl WAFFLE {
    pub fn new(mem_size: usize) -> WAFFLE {
        WAFFLE {
            mem: vec![0; mem_size],
            mem_size,
            pc: 0,
            sp: mem_size - 1,
            registers: [0; 8],
            fregisters: [0.0; 8],
            flags: 0b00000000,
            inst: Instruction::HALT,
            _platform_big_endian: is_big_endian(),
        }
    }

    pub fn load_program(&mut self, prog: Vec<u8>) {
        self.mem = prog
    }

    pub fn cast_from_bytes(&self, data: [u8;8]) -> i64 {
        if self._platform_big_endian {
            i64::from_be_bytes(data)
        } else {
            i64::from_le_bytes(data)
        }
    }

    pub fn cast_from_bytesf(&self, data: [u8;8]) -> f64 {
        if self._platform_big_endian {
            f64::from_be_bytes(data)
        } else {
            f64::from_le_bytes(data)
        }
    }

    pub fn cast_to_bytes(&self, data: i64) -> [u8; 8] {
        if self._platform_big_endian {
            data.to_be_bytes()
        } else {
            data.to_le_bytes()
        }
    }

    pub fn cast_to_bytesz(&self, data: usize) -> [u8; 8] {
        if self._platform_big_endian {
            data.to_be_bytes()
        } else {
            data.to_le_bytes()
        }
    }

    pub fn cast_to_bytesf(&self, data: f64) -> [u8; 8] {
        if self._platform_big_endian {
            data.to_be_bytes()
        } else {
            data.to_le_bytes()
        }
    }

    pub fn stack_push(&mut self, data: i64){
        let res = self.cast_to_bytes(data);
        self.sp -= 1;
        for val in &res {
            self.mem[self.sp] = *val;
            self.sp -= 1
        }
    }

    pub fn stack_pushf(&mut self, data: f64){
        let res = self.cast_to_bytesf(data);
        self.sp -= 1;
        for val in &res {
            self.mem[self.sp] = *val;
            self.sp -= 1
        }
    }

    pub fn memory_write(&mut self, addr: usize, value: i64) {
        let as_bytes= self.cast_to_bytes(value);
        self.mem[addr..=addr+9].iter_mut().enumerate().map(|(idx, el)|*el = as_bytes[idx]).count();
    }

    pub fn memory_writef(&mut self, addr: usize, value: f64) {
        let as_bytes= self.cast_to_bytesf(value);
        self.mem[addr..=addr+9].iter_mut().enumerate().map(|(idx, el)|*el = as_bytes[idx]).count();
    }

    pub fn memory_read_byte(&self, addr: usize) -> u8 {
        self.mem[addr]
    }

    pub fn memory_read(&self, addr: usize) -> i64 {
        let mut arr = [0u8;8];
        self.mem[addr..=addr+7].iter().enumerate().map(|(idx,el)| arr[idx] = *el).count();
        if self._platform_big_endian{
            i64::from_be_bytes(arr)
        } else {
            i64::from_le_bytes(arr)
        }
    }

    pub fn memory_readf(&self, addr: usize) -> f64 {
        let mut arr = [0u8;8];
        self.mem[addr..=addr+9].iter().enumerate().map(|(idx,el)| arr[idx] = *el).count();
        if self._platform_big_endian{
            f64::from_be_bytes(arr)
        } else {
            f64::from_le_bytes(arr)
        }
    }

    pub fn register_read(&self, addr: usize) -> i64 {
        self.registers[self.mem[addr] as usize]
    }

    pub fn register_readf(&self, addr: usize) -> f64 {
        self.fregisters[self.mem[addr] as usize]
    }

    pub fn register_write(&mut self, addr: usize, data: i64) {
        self.registers[self.mem[addr] as usize] = data;
    }

    pub fn register_writef(&mut self, addr: usize, data: f64) {
        self.fregisters[self.mem[addr] as usize] = data;
    }

    pub fn flags_clear(&mut self) {self.flags = 0;}

    pub fn flag_check(&self, flag: Flags) -> bool {self.flags & flag.binary() != 0}

    pub fn flag_clear(&mut self, flag: Flags) {self.flags &= !flag.binary();}

    pub fn flag_set(&mut self, flag: Flags) {self.flags |= flag.binary();}
}
