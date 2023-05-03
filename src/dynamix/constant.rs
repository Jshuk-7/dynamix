use std::{ops::Index, fmt::Display};

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(align(4))]
pub enum Constant {
    Double(f64),
}

impl Display for Constant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Constant::Double(x) => write!(f, "'{x}'"),
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
        Self { constants: Vec::new() }
    }

    pub fn push(&mut self, value: Constant) {
        self.constants.push(value);
    }

    pub fn len(&self) -> usize {
        self.constants.len()
    }
}
