use crate::constant::{Constant, ConstantPool};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpCode {
    Return,
    Constant,
    ConstantLong,
}

impl OpCode {
    pub fn from(value: u8) -> Result<Self, ()> {
        match value {
            value if value == OpCode::Return as u8 => Ok(OpCode::Return),
            value if value == OpCode::Constant as u8 => Ok(OpCode::Constant),
            value if value == OpCode::ConstantLong as u8 => Ok(OpCode::ConstantLong),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ByteBlock {
    pub bytes: Vec<u8>,
    pub constants: ConstantPool,
    pub lines: Vec<u32>,
}

impl ByteBlock {
    pub fn new() -> Self {
        Self {
            bytes: Vec::new(),
            constants: ConstantPool::new(),
            lines: Vec::new(),
        }
    }

    pub fn push(&mut self, byte: u8, line: u32) {
        self.bytes.push(byte);
        self.lines.push(line);
    }

    pub fn push_constant(&mut self, value: Constant) -> u8 {
        self.constants.push(value);
        self.constants.len() as u8 - 1
    }
}
