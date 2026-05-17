use crate::vm::opcode::OpCode;

pub struct Chunk {
    pub code: Vec<u8>,
    pub constants: Vec<crate::runtime::Value>,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            code: Vec::new(),
            constants: Vec::new(),
        }
    }

    pub fn write(&mut self, byte: u8) {
        self.code.push(byte);
    }

    pub fn add_constant(&mut self, value: crate::runtime::Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }
}
