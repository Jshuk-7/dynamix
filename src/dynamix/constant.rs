use std::{fmt::Display, ops::Index};

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(align(4))]
pub enum Constant {
    Number(f64),
    Bool(bool),
    Char(char),
    Null,
}

impl Display for Constant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Constant::Number(x) => write!(f, "'{x}'"),
            Constant::Bool(x) => write!(f, "'{x}'"),
            Constant::Char(c) => write!(f, "'{c}'"),
            Constant::Null => write!(f, "'null'"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ConstantPool {
    pub constants: Vec<Constant>,
}

impl Index<usize> for ConstantPool {
    type Output = Constant;

    fn index(&self, index: usize) -> &Self::Output {
        &self.constants[index]
    }
}

impl ConstantPool {
    pub fn new() -> Self {
        Self {
            constants: Vec::new(),
        }
    }

    pub fn push(&mut self, value: Constant) {
        self.constants.push(value);
    }

    pub fn len(&self) -> usize {
        self.constants.len()
    }

    pub fn is_empty(&self) -> bool {
        self.constants.is_empty()
    }
}

impl Default for ConstantPool {
    fn default() -> Self {
        Self::new()
    }
}
