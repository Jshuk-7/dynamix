use crate::constant::{Constant, ConstantPool};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpCode {
    Print,
    Pop,
    DefineGlobal,
    GetGlobal,
    SetGlobal,
    GetLocal,
    SetLocal,
    Jz,
    Jmp,
    Loop,
    Constant,
    True,
    False,
    Char,
    Null,
    Equal,
    Greater,
    Less,
    Negate,
    Not,
    Add,
    Sub,
    Mul,
    Div,
    Return,
}

pub enum OpError {
    UnknownOperation,
}

impl OpCode {
    pub fn from(value: u8) -> Result<Self, OpError> {
        match value {
            value if value == OpCode::Print as u8 => Ok(OpCode::Print),
            value if value == OpCode::Pop as u8 => Ok(OpCode::Pop),
            value if value == OpCode::DefineGlobal as u8 => Ok(OpCode::DefineGlobal),
            value if value == OpCode::GetGlobal as u8 => Ok(OpCode::GetGlobal),
            value if value == OpCode::SetGlobal as u8 => Ok(OpCode::SetGlobal),
            value if value == OpCode::GetLocal as u8 => Ok(OpCode::GetLocal),
            value if value == OpCode::SetLocal as u8 => Ok(OpCode::SetLocal),
            value if value == OpCode::Jz as u8 => Ok(OpCode::Jz),
            value if value == OpCode::Jmp as u8 => Ok(OpCode::Jmp),
            value if value == OpCode::Loop as u8 => Ok(OpCode::Loop),
            value if value == OpCode::Constant as u8 => Ok(OpCode::Constant),
            value if value == OpCode::True as u8 => Ok(OpCode::True),
            value if value == OpCode::False as u8 => Ok(OpCode::False),
            value if value == OpCode::Char as u8 => Ok(OpCode::Char),
            value if value == OpCode::Null as u8 => Ok(OpCode::Null),
            value if value == OpCode::Equal as u8 => Ok(OpCode::Equal),
            value if value == OpCode::Greater as u8 => Ok(OpCode::Greater),
            value if value == OpCode::Less as u8 => Ok(OpCode::Less),
            value if value == OpCode::Negate as u8 => Ok(OpCode::Negate),
            value if value == OpCode::Not as u8 => Ok(OpCode::Not),
            value if value == OpCode::Add as u8 => Ok(OpCode::Add),
            value if value == OpCode::Sub as u8 => Ok(OpCode::Sub),
            value if value == OpCode::Mul as u8 => Ok(OpCode::Mul),
            value if value == OpCode::Div as u8 => Ok(OpCode::Div),
            value if value == OpCode::Return as u8 => Ok(OpCode::Return),
            _ => Err(OpError::UnknownOperation),
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

    pub fn write_constant(&mut self, value: Constant, line: u32) {
        let constant = self.push_constant(value);
        self.push(constant, line);
    }

    pub fn push_constant(&mut self, value: Constant) -> u8 {
        self.constants.push(value);
        self.constants.len() as u8 - 1
    }
}

impl Default for ByteBlock {
    fn default() -> Self {
        Self::new()
    }
}
