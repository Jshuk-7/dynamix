use std::{fmt::Display, ops::Index};

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd)]
pub enum ObjectType {
    String,
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd)]
pub struct Object {
    pub typ3: ObjectType,
    pub bytes: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Constant {
    Number(f64),
    Bool(bool),
    Char(char),
    Obj(Object),
    Null,
}

impl Constant {
    pub fn type_to_string(&self) -> &str {
        match self {
            Constant::Number(..) => "number",
            Constant::Bool(..) => "bool",
            Constant::Char(..) => "char",
            Constant::Obj(obj) => match obj.typ3 {
                ObjectType::String => "String",
            },
            Constant::Null => "null",
        }
    }
}

impl Display for Constant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Constant::Number(x) => write!(f, "{x}"),
            Constant::Bool(x) => write!(f, "{x}"),
            Constant::Char(c) => write!(f, "{c}"),
            Constant::Obj(obj) => match obj.typ3 {
                ObjectType::String => {
                    write!(f, "{}", String::from_utf8(obj.bytes.clone()).unwrap())
                }
            },
            Constant::Null => write!(f, "null"),
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
