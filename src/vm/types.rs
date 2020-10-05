use crate::vm::instructions::Instruction;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Types {
    STRING,
    INTEGER,
    DECIMAL,
    OPERATION,
    ADDRESS,
    REGISTER,
    FUNCTION,
    ERROR,
    ANY,
}


impl Types {
    pub fn bytes_needed(self) -> usize {
        match self {
            Types::INTEGER |Types::DECIMAL|Types::ADDRESS => 8,
            Types::OPERATION|Types::REGISTER|Types::FUNCTION => 1,
            _ => 8
        }
    }
}


#[derive(Debug)]
pub struct Lexeme {
    pub instruction: Instruction,
    pub type_info: Types, // stores type information
    pub value: Vec<u8>,   // stores bytes
}


impl Lexeme {
    pub fn new(type_info: Types, instruction: Instruction, value: Vec<u8>) -> Self {
        Lexeme {type_info, instruction, value}
    }

    pub fn value_as_array(&self) -> [u8;8] {
        let mut arr = [0u8; 8];
        self.value.iter().enumerate().map(|(idx,x)| arr[idx] = *x).count();
        arr
    }

    pub fn get_type_size(&self) -> usize {
        self.value.len()
    }
}