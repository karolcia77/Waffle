#[derive(Clone, Copy, Debug)]
pub enum Types {
    STRING,
    NUMBER,
    DECIMAL,
    OPERATION,
    ADDRESS,
    REGISTER,
    FUNCTION,
    ANY,
}

#[derive(Debug)]
pub struct Lexeme {
    type_info: Types, // stores type information
    value: Vec<u8>,   // stores bytes
    position: usize
}


impl Lexeme {
    pub fn new(type_info: Types, value: Vec<u8>, position: usize) -> Self {
        Lexeme {type_info, value, position}
    }

    pub fn get_type_size(&self) -> usize {
        self.value.len()
    }
}