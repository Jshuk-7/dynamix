use crate::lexer::{Lexer, TokenType};

pub struct Compiler {
    lexer: Lexer,
}

impl Compiler {
    pub fn new(source: &String) -> Self {
        Self {
            lexer: Lexer::new(source),
        }
    }

    pub fn compile(&mut self) {
        while let Some(token) = self.lexer.next() {
            if let TokenType::Eof = token.typ3 {
                break;
            }

            println!("{token}");
        }
    }
}
