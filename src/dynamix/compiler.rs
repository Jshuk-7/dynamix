use crate::lexer::{Lexer, TokenType};

pub struct Compiler<'a> {
    lexer: Lexer<'a>,
}

impl<'a> Compiler<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            lexer: Lexer::new(source),
        }
    }

    pub fn compile(&mut self) {
        for token in self.lexer.by_ref() {
            if let TokenType::Eof = token.typ3 {
                break;
            }

            println!("{token}");
        }
    }
}
