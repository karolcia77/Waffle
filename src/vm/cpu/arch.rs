use crate::vm::instructions::Instructions;

pub enum Register {
    AX, BX, CX, DX, EX, FX, GX, HX,
    FAX, FBX, FCX, FDX, FEX, FFX, FGX, FHX,
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
    pub inst: Instructions,
    _platform_big_endian: bool
}

fn is_big_endian() -> bool {
    if cfg!(target_endian="big") {
        true
    } else {false}
}


impl WAFFLE {
    pub fn new(mem_size: usize) -> Self {
        WAFFLE {
            mem: Vec::with_capacity(mem_size),
            mem_size,
            pc: 0,
            sp: mem_size - 1,
            registers: [0; 8],
            fregisters: [0.0; 8],
            flags: 0b00000000,
            inst: Instructions::HALT,
            _platform_big_endian: is_big_endian(),
        }
    }

    pub fn cast_to_bytes(&self, data: i64) -> [u8; 8] {
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

    pub fn memory_read(&self, addr: usize) -> i64 {
        let mut arr = [0u8; 8];
        for ptr in 0..8 {
            arr[ptr] = self.mem[addr+ptr];
        }
        if self._platform_big_endian{
            i64::from_be_bytes(arr)
        } else {
            i64::from_le_bytes(arr)
        }
    }

    pub fn memory_readf(&self, addr: usize) -> f64 {
        let mut arr = [0u8; 8];
        for ptr in 0..8 {
            arr[ptr] = self.mem[addr+ptr];
        }
        if self._platform_big_endian{
            f64::from_be_bytes(arr)
        } else {
            f64::from_le_bytes(arr)
        }
    }

    pub fn register_write(&mut self, register: usize, data: [u8; 8]) {
        if self._platform_big_endian {
            self.registers[register] = i64::from_be_bytes(data)
        } else {
            self.registers[register] = i64::from_le_bytes(data)
        }
    }

    pub fn register_writef(&mut self, register: usize, data: [u8; 8]) {
        if self._platform_big_endian {
            self.fregisters[register] = f64::from_be_bytes(data)
        } else {
            self.fregisters[register] = f64::from_le_bytes(data)
        }
    }

    pub fn flags_clear(&mut self) {self.flags = 0;}

    pub fn flag_check(&self, flag: Flags) -> bool {self.flags & flag.binary() != 0}

    pub fn flag_clear(&mut self, flag: Flags) {self.flags &= !flag.binary();}

    pub fn flag_set(&mut self, flag: Flags) {self.flags |= flag.binary();}
}
