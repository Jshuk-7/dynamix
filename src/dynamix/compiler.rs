use std::collections::HashMap;

use crate::{
    byte_block::{ByteBlock, OpCode},
    constant::Constant,
    disassembler::Disassembler,
    lexer::{Lexer, Token, TokenType},
};

struct Parser {
    cursor: Token,
    previous: Token,
    had_error: bool,
    panic_mode: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq)]
enum Precedence {
    None,
    Assignment,
    Or,
    And,
    Equality,
    Comparison,
    Term,
    Factor,
    Unary,
    Call,
    Atom,
}

impl Precedence {
    fn from_u32(value: u32) -> Self {
        use Precedence::*;
        match value {
            0 => None,
            1 => Assignment,
            2 => Or,
            3 => And,
            4 => Equality,
            5 => Comparison,
            6 => Term,
            7 => Factor,
            8 => Unary,
            9 => Call,
            10 => Atom,
            _ => panic!("Unknown value: {value}"),
        }
    }
}

type ParseFn<'a> = fn(&mut Compiler<'a>);

struct ParseRule<'a> {
    prefix: Option<Box<ParseFn<'a>>>,
    infix: Option<Box<ParseFn<'a>>>,
    precedence: Precedence,
}

pub struct Compiler<'a> {
    lexer: Lexer<'a>,
    parser: Parser,
    block: ByteBlock,
    parse_rules: HashMap<u32, ParseRule<'a>>,
}

impl<'a> Compiler<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            lexer: Lexer::new(source),
            parser: Parser {
                cursor: Token::new(),
                previous: Token::new(),
                had_error: false,
                panic_mode: false,
            },
            block: ByteBlock::new(),
            parse_rules: vec![
                (
                    TokenType::LParen,
                    ParseRule {
                        prefix: Some(Box::new(Compiler::grouping)),
                        infix: None,
                        precedence: Precedence::None,
                    },
                ),
                (
                    TokenType::RParen,
                    ParseRule {
                        prefix: None,
                        infix: None,
                        precedence: Precedence::None,
                    },
                ),
                (
                    TokenType::LCurly,
                    ParseRule {
                        prefix: None,
                        infix: None,
                        precedence: Precedence::None,
                    },
                ),
                (
                    TokenType::RCurly,
                    ParseRule {
                        prefix: None,
                        infix: None,
                        precedence: Precedence::None,
                    },
                ),
                (
                    TokenType::Comma,
                    ParseRule {
                        prefix: None,
                        infix: None,
                        precedence: Precedence::None,
                    },
                ),
                (
                    TokenType::Dot,
                    ParseRule {
                        prefix: None,
                        infix: None,
                        precedence: Precedence::None,
                    },
                ),
                (
                    TokenType::Minus,
                    ParseRule {
                        prefix: Some(Box::new(Compiler::unary)),
                        infix: Some(Box::new(Compiler::binary)),
                        precedence: Precedence::Term,
                    },
                ),
                (
                    TokenType::Plus,
                    ParseRule {
                        prefix: None,
                        infix: Some(Box::new(Compiler::binary)),
                        precedence: Precedence::Term,
                    },
                ),
                (
                    TokenType::Semicolon,
                    ParseRule {
                        prefix: None,
                        infix: None,
                        precedence: Precedence::None,
                    },
                ),
                (
                    TokenType::Slash,
                    ParseRule {
                        prefix: None,
                        infix: Some(Box::new(Compiler::binary)),
                        precedence: Precedence::Factor,
                    },
                ),
                (
                    TokenType::Star,
                    ParseRule {
                        prefix: None,
                        infix: Some(Box::new(Compiler::binary)),
                        precedence: Precedence::Factor,
                    },
                ),
                (
                    TokenType::Bang,
                    ParseRule {
                        prefix: None,
                        infix: None,
                        precedence: Precedence::None,
                    },
                ),
                (
                    TokenType::BangEq,
                    ParseRule {
                        prefix: None,
                        infix: None,
                        precedence: Precedence::None,
                    },
                ),
                (
                    TokenType::Eq,
                    ParseRule {
                        prefix: None,
                        infix: None,
                        precedence: Precedence::None,
                    },
                ),
                (
                    TokenType::EqEq,
                    ParseRule {
                        prefix: None,
                        infix: None,
                        precedence: Precedence::None,
                    },
                ),
                (
                    TokenType::Gt,
                    ParseRule {
                        prefix: None,
                        infix: None,
                        precedence: Precedence::None,
                    },
                ),
                (
                    TokenType::Gte,
                    ParseRule {
                        prefix: None,
                        infix: None,
                        precedence: Precedence::None,
                    },
                ),
                (
                    TokenType::Gte,
                    ParseRule {
                        prefix: None,
                        infix: None,
                        precedence: Precedence::None,
                    },
                ),
                (
                    TokenType::Lt,
                    ParseRule {
                        prefix: None,
                        infix: None,
                        precedence: Precedence::None,
                    },
                ),
                (
                    TokenType::Lte,
                    ParseRule {
                        prefix: None,
                        infix: None,
                        precedence: Precedence::None,
                    },
                ),
                (
                    TokenType::Ident,
                    ParseRule {
                        prefix: None,
                        infix: None,
                        precedence: Precedence::None,
                    },
                ),
                (
                    TokenType::String,
                    ParseRule {
                        prefix: None,
                        infix: None,
                        precedence: Precedence::None,
                    },
                ),
                (
                    TokenType::Number,
                    ParseRule {
                        prefix: Some(Box::new(Compiler::number)),
                        infix: None,
                        precedence: Precedence::None,
                    },
                ),
                (
                    TokenType::Char,
                    ParseRule {
                        prefix: None,
                        infix: None,
                        precedence: Precedence::None,
                    },
                ),
                (
                    TokenType::And,
                    ParseRule {
                        prefix: None,
                        infix: None,
                        precedence: Precedence::None,
                    },
                ),
                (
                    TokenType::Struct,
                    ParseRule {
                        prefix: None,
                        infix: None,
                        precedence: Precedence::None,
                    },
                ),
                (
                    TokenType::Else,
                    ParseRule {
                        prefix: None,
                        infix: None,
                        precedence: Precedence::None,
                    },
                ),
                (
                    TokenType::False,
                    ParseRule {
                        prefix: None,
                        infix: None,
                        precedence: Precedence::None,
                    },
                ),
                (
                    TokenType::For,
                    ParseRule {
                        prefix: None,
                        infix: None,
                        precedence: Precedence::None,
                    },
                ),
                (
                    TokenType::Fun,
                    ParseRule {
                        prefix: None,
                        infix: None,
                        precedence: Precedence::None,
                    },
                ),
                (
                    TokenType::If,
                    ParseRule {
                        prefix: None,
                        infix: None,
                        precedence: Precedence::None,
                    },
                ),
                (
                    TokenType::Null,
                    ParseRule {
                        prefix: None,
                        infix: None,
                        precedence: Precedence::None,
                    },
                ),
                (
                    TokenType::Or,
                    ParseRule {
                        prefix: None,
                        infix: None,
                        precedence: Precedence::None,
                    },
                ),
                (
                    TokenType::Print,
                    ParseRule {
                        prefix: None,
                        infix: None,
                        precedence: Precedence::None,
                    },
                ),
                (
                    TokenType::Return,
                    ParseRule {
                        prefix: None,
                        infix: None,
                        precedence: Precedence::None,
                    },
                ),
                (
                    TokenType::Super,
                    ParseRule {
                        prefix: None,
                        infix: None,
                        precedence: Precedence::None,
                    },
                ),
                (
                    TokenType::SSelf,
                    ParseRule {
                        prefix: None,
                        infix: None,
                        precedence: Precedence::None,
                    },
                ),
                (
                    TokenType::True,
                    ParseRule {
                        prefix: None,
                        infix: None,
                        precedence: Precedence::None,
                    },
                ),
                (
                    TokenType::Let,
                    ParseRule {
                        prefix: None,
                        infix: None,
                        precedence: Precedence::None,
                    },
                ),
                (
                    TokenType::While,
                    ParseRule {
                        prefix: None,
                        infix: None,
                        precedence: Precedence::None,
                    },
                ),
                (
                    TokenType::Error,
                    ParseRule {
                        prefix: None,
                        infix: None,
                        precedence: Precedence::None,
                    },
                ),
                (
                    TokenType::Eof,
                    ParseRule {
                        prefix: None,
                        infix: None,
                        precedence: Precedence::None,
                    },
                ),
            ]
            .into_iter()
            .map(|(k, v)| (k as u32, v))
            .collect(),
        }
    }

    pub fn compile(&mut self) -> bool {
        self.advance();
        self.expression();
        self.emit_return();
        self.consume(TokenType::Eof, "Expected end of expression".to_string());

        if cfg!(debug_assertions) && cfg!(feature = "debug-print") {
            if !self.parser.had_error {
                Disassembler::disassemble(&self.block, "code");
            }
        }

        !self.parser.had_error
    }

    pub fn bytes(&self) -> &ByteBlock {
        &self.block
    }

    fn advance(&mut self) {
        self.parser.previous = self.parser.cursor.clone();

        loop {
            self.parser.cursor = self.lexer.next().unwrap();
            if self.parser.cursor.typ3 != TokenType::Error {
                break;
            }

            self.error_at_cursor(&self.parser.cursor.lexeme.clone());
        }
    }

    fn expression(&mut self) {
        self.parse_precedence(Precedence::Assignment)
    }

    fn grouping(&mut self) {
        self.expression();
        self.consume(
            TokenType::RParen,
            "Expected ')' after expression".to_string(),
        );
    }

    fn binary(&mut self) {
        match self.get_operator_precedence() {
            Some((precedence, operator)) => {
                self.parse_precedence(Precedence::from_u32(precedence as u32 + 1));

                if let TokenType::Plus = operator {
                    self.emit_byte(OpCode::Add as u8);
                } else if let TokenType::Minus = operator {
                    self.emit_byte(OpCode::Sub as u8);
                } else if let TokenType::Star = operator {
                    self.emit_byte(OpCode::Mul as u8);
                } else if let TokenType::Slash = operator {
                    self.emit_byte(OpCode::Div as u8);
                } else {
                    let err = format!(
                        "Expected operator '+-*/' found '{}'",
                        self.parser.previous.lexeme
                    );
                    self.error(&err);
                }
            }
            None => {
                let err = format!(
                    "Expected operator '+-*/' found '{}'",
                    self.parser.previous.lexeme
                );
                self.error(&err);
            }
        }
    }

    fn number(&mut self) {
        let value = self.parser.previous.lexeme.parse::<f64>().unwrap();
        self.emit_constant(Constant::Double(value));
    }

    fn unary(&mut self) {
        let operator = self.parser.previous.typ3;

        // compile operand
        self.parse_precedence(Precedence::Unary);

        if let TokenType::Minus = operator {
            self.emit_byte(OpCode::Negate as u8);
        } else {
            let err = format!(
                "Expected unary operator '-' or '!' found '{}'",
                self.parser.previous.lexeme
            );
            self.error(&err);
        }
    }

    fn parse_precedence(&mut self, precedence: Precedence) {
        self.advance();

        if let Some(rule) = self.parse_rules.get(&(self.parser.previous.typ3 as u32)) {
            match rule.prefix.clone() {
                Some(func) => func(self),
                None => self.error(&"Expected expression".to_string()),
            }
        }

        while let Some(rule) = self.parse_rules.get(&(self.parser.cursor.typ3 as u32)) {
            if precedence <= rule.precedence {
                self.advance();

                if let Some(rule) = self.parse_rules.get(&(self.parser.previous.typ3 as u32)) {
                    match rule.infix.clone() {
                        Some(func) => func(self),
                        None => break,
                    }
                }
            } else {
                break;
            }
        }
    }

    fn consume(&mut self, typ3: TokenType, msg: String) {
        if typ3 == self.parser.cursor.typ3 {
            self.advance();
            return;
        }

        self.error_at_cursor(&msg);
    }

    fn get_operator_precedence(&self) -> Option<(Precedence, TokenType)> {
        let operator = self.parser.previous.typ3;
        match self.parse_rules.get(&(operator as u32)) {
            Some(rule) => Some((rule.precedence, operator)),
            None => None,
        }
    }

    fn emit_byte(&mut self, byte: u8) {
        self.block.push(byte, self.parser.previous.line as u32);
    }

    fn emit_bytes(&mut self, byte1: u8, byte2: u8) {
        self.emit_byte(byte1);
        self.emit_byte(byte2);
    }

    fn emit_return(&mut self) {
        self.emit_byte(OpCode::Return as u8)
    }

    fn make_constant(&mut self, constant: Constant) -> u8 {
        let index = self.block.push_constant(constant);

        if index > u8::MAX {
            self.error(&"Too many constants in one block".to_string());
            return 0;
        }

        index
    }

    fn emit_constant(&mut self, constant: Constant) {
        self.emit_byte(OpCode::Constant as u8);
        let index = self.make_constant(constant);
        self.emit_byte(index);
    }

    fn error_at_cursor(&mut self, msg: &String) {
        self.error_at(&self.parser.cursor.clone(), msg)
    }

    fn error(&mut self, msg: &String) {
        self.error_at(&self.parser.previous.clone(), msg)
    }

    fn error_at(&mut self, token: &Token, msg: &String) {
        if self.parser.panic_mode {
            return;
        } else {
            self.parser.panic_mode = true;
        }

        print!("[line:{:2}] Error:", token.line);

        if let TokenType::Eof = token.typ3 {
            print!(" at end");
        } else if let TokenType::Error = token.typ3 {
            // Nothing
        } else {
            print!(" at '{}'", token.lexeme);
        }

        println!(": {msg}");

        self.parser.had_error = true;
    }
}
