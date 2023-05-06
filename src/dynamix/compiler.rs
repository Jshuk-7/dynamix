use std::collections::HashMap;

use crate::{
    byte_block::{ByteBlock, OpCode},
    constant::{Constant, Object, ObjectType},
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

type ParseFn<'a> = fn(&mut Compiler<'a>, bool);

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
                        prefix: Some(Box::new(Compiler::unary)),
                        infix: None,
                        precedence: Precedence::None,
                    },
                ),
                (
                    TokenType::BangEq,
                    ParseRule {
                        prefix: None,
                        infix: Some(Box::new(Compiler::binary)),
                        precedence: Precedence::Equality,
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
                        infix: Some(Box::new(Compiler::binary)),
                        precedence: Precedence::Equality,
                    },
                ),
                (
                    TokenType::Gt,
                    ParseRule {
                        prefix: None,
                        infix: Some(Box::new(Compiler::binary)),
                        precedence: Precedence::Comparison,
                    },
                ),
                (
                    TokenType::Gte,
                    ParseRule {
                        prefix: None,
                        infix: Some(Box::new(Compiler::binary)),
                        precedence: Precedence::Comparison,
                    },
                ),
                (
                    TokenType::Lt,
                    ParseRule {
                        prefix: None,
                        infix: Some(Box::new(Compiler::binary)),
                        precedence: Precedence::Comparison,
                    },
                ),
                (
                    TokenType::Lte,
                    ParseRule {
                        prefix: None,
                        infix: Some(Box::new(Compiler::binary)),
                        precedence: Precedence::Comparison,
                    },
                ),
                (
                    TokenType::Ident,
                    ParseRule {
                        prefix: Some(Box::new(Compiler::variable)),
                        infix: None,
                        precedence: Precedence::None,
                    },
                ),
                (
                    TokenType::String,
                    ParseRule {
                        prefix: Some(Box::new(Compiler::string)),
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
                        prefix: Some(Box::new(Compiler::character)),
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
                        prefix: Some(Box::new(Compiler::literal)),
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
                        prefix: Some(Box::new(Compiler::literal)),
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
                        prefix: Some(Box::new(Compiler::literal)),
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

        while !self.matches(TokenType::Eof) {
            self.declaration();
        }

        self.emit_return();
        self.consume(TokenType::Eof, "Expected end of expression".to_string());

        if !self.parser.had_error && cfg!(debug_assertions) && cfg!(feature = "debug-print") {
            Disassembler::disassemble(&self.block, "code");
        }

        !self.parser.had_error
    }

    pub fn byte_code(&self) -> &ByteBlock {
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

    fn let_declaration(&mut self) {
        let global = self.parse_variable("Expected variable name".to_string());

        if self.matches(TokenType::Eq) {
            self.expression();
        } else {
            self.emit_byte(OpCode::Null as u8);
        }

        self.consume(
            TokenType::Semicolon,
            "Expected ';' after expression".to_string(),
        );

        self.define_variable(global);
    }

    fn print_statement(&mut self) {
        self.expression();
        self.consume(
            TokenType::Semicolon,
            "Expected ';' after expression".to_string(),
        );

        self.emit_byte(OpCode::Print as u8);
    }

    fn expression_statement(&mut self) {
        self.expression();
        self.consume(
            TokenType::Semicolon,
            "Expected ';' after expression".to_string(),
        );

        self.emit_byte(OpCode::Pop as u8);
    }

    fn declaration(&mut self) {
        if self.matches(TokenType::Let) {
            self.let_declaration();
        } else {
            self.statement();
        }

        if self.parser.panic_mode {
            self.synchronize();
        }
    }

    fn statement(&mut self) {
        if self.matches(TokenType::Print) {
            self.print_statement();
        } else {
            self.expression_statement();
        }
    }

    fn synchronize(&mut self) {
        self.parser.panic_mode = false;

        while self.parser.cursor.typ3 != TokenType::Eof {
            if let TokenType::Semicolon = self.parser.previous.typ3 {
                break;
            }

            use TokenType::*;
            match self.parser.cursor.typ3 {
                Struct | Fun | For | If | While | Let | Print | Return => break,
                _ => (),
            }

            self.advance();
        }
    }

    fn grouping(&mut self, _can_assign: bool) {
        self.expression();
        self.consume(
            TokenType::RParen,
            "Expected ')' after expression".to_string(),
        );
    }

    fn binary(&mut self, _can_assign: bool) {
        match self.get_operator_precedence() {
            Some((precedence, operator)) => {
                self.parse_precedence(Precedence::from_u32(precedence as u32 + 1));

                match operator {
                    TokenType::Plus => self.emit_byte(OpCode::Add as u8),
                    TokenType::Minus => self.emit_byte(OpCode::Sub as u8),
                    TokenType::Star => self.emit_byte(OpCode::Mul as u8),
                    TokenType::Slash => self.emit_byte(OpCode::Div as u8),
                    TokenType::BangEq => {
                        self.emit_bytes(vec![OpCode::Equal as u8, OpCode::Not as u8])
                    }
                    TokenType::EqEq => self.emit_byte(OpCode::Equal as u8),
                    TokenType::Gt => self.emit_byte(OpCode::Greater as u8),
                    TokenType::Gte => self.emit_bytes(vec![OpCode::Less as u8, OpCode::Not as u8]),
                    TokenType::Lt => self.emit_byte(OpCode::Less as u8),
                    TokenType::Lte => {
                        self.emit_bytes(vec![OpCode::Greater as u8, OpCode::Not as u8])
                    }
                    _ => {
                        let err = format!(
                            "Expected operator '+-*/' found '{}'",
                            self.parser.previous.lexeme
                        );

                        self.error(&err);
                    }
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

    fn literal(&mut self, _can_assign: bool) {
        match self.parser.previous.typ3 {
            TokenType::False => self.emit_byte(OpCode::False as u8),
            TokenType::True => self.emit_byte(OpCode::True as u8),
            TokenType::Null => self.emit_byte(OpCode::Null as u8),
            _ => unreachable!(),
        }
    }

    fn named_variable(&mut self, name: Token, can_assign: bool) {
        let arg = self.identifier_constant(&name);

        if can_assign && self.matches(TokenType::Eq) {
            self.expression();
            self.emit_bytes(vec![OpCode::SetGlobal as u8, arg]);
        } else {
            self.emit_bytes(vec![OpCode::GetGlobal as u8, arg]);
        }
    }

    fn variable(&mut self, can_assign: bool) {
        self.named_variable(self.parser.previous.clone(), can_assign);
    }

    fn string(&mut self, _can_assign: bool) {
        let mut value = self.parser.previous.lexeme.as_bytes().to_owned();

        // remove quotes from conversion
        value.remove(0);
        value.remove(value.len() - 1);

        self.emit_constant(Constant::Obj(Object {
            typ3: ObjectType::String,
            bytes: value,
        }));
    }

    fn number(&mut self, _can_assign: bool) {
        let value = self.parser.previous.lexeme.parse::<f64>().unwrap();
        self.emit_constant(Constant::Number(value));
    }

    fn character(&mut self, _can_assign: bool) {
        let value = self.parser.previous.lexeme.parse::<char>().unwrap();
        self.emit_constant(Constant::Char(value));
    }

    fn unary(&mut self, _can_assign: bool) {
        let operator = self.parser.previous.typ3;

        // compile operand
        self.parse_precedence(Precedence::Unary);

        if let TokenType::Minus = operator {
            self.emit_byte(OpCode::Negate as u8);
        } else if let TokenType::Bang = operator {
            self.emit_byte(OpCode::Not as u8);
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

        let can_assign = precedence <= Precedence::Assignment;

        if let Some(rule) = self.parse_rules.get(&(self.parser.previous.typ3 as u32)) {
            match rule.prefix.clone() {
                Some(func) => func(self, can_assign),
                None => {
                    let err = format!(
                        "Expected expression found '{}'",
                        self.parser.previous.lexeme
                    );
                    self.error(&err);
                }
            }
        }

        while let Some(rule) = self.parse_rules.get(&(self.parser.cursor.typ3 as u32)) {
            if precedence <= rule.precedence {
                self.advance();

                if let Some(rule) = self.parse_rules.get(&(self.parser.previous.typ3 as u32)) {
                    match rule.infix.clone() {
                        Some(func) => func(self, can_assign),
                        None => break,
                    }
                }
            } else {
                break;
            }
        }

        if can_assign && self.matches(TokenType::Eq) {
            self.error(&"Invalid assignment target".to_string());
        }
    }

    fn identifier_constant(&mut self, name: &Token) -> u8 {
        self.make_constant(Constant::Obj(Object {
            typ3: ObjectType::String,
            bytes: name.lexeme.bytes().collect(),
        }))
    }

    fn parse_variable(&mut self, error: String) -> u8 {
        self.consume(TokenType::Ident, error);
        self.identifier_constant(&self.parser.previous.clone())
    }

    fn define_variable(&mut self, global: u8) {
        self.emit_bytes(vec![OpCode::DefineGlobal as u8, global]);
    }

    fn consume(&mut self, typ3: TokenType, msg: String) {
        if typ3 == self.parser.cursor.typ3 {
            self.advance();
            return;
        }

        self.error_at_cursor(&msg);
    }

    fn matches(&mut self, typ3: TokenType) -> bool {
        if !self.check(typ3) {
            return false;
        }

        self.advance();
        true
    }

    fn check(&self, typ3: TokenType) -> bool {
        self.parser.cursor.typ3 == typ3
    }

    fn get_operator_precedence(&self) -> Option<(Precedence, TokenType)> {
        let operator = self.parser.previous.typ3;
        self.parse_rules
            .get(&(operator as u32))
            .map(|rule| (rule.precedence, operator))
    }

    fn emit_byte(&mut self, byte: u8) {
        self.block.push(byte, self.parser.previous.line as u32);
    }

    fn emit_bytes(&mut self, bytes: Vec<u8>) {
        for byte in bytes.iter() {
            self.emit_byte(*byte);
        }
    }

    fn emit_return(&mut self) {
        self.emit_byte(OpCode::Return as u8)
    }

    fn make_constant(&mut self, constant: Constant) -> u8 {
        let index = self.block.push_constant(constant);

        if index == u8::MAX {
            self.error(&"Too many constants in one block".to_string());
            return 0;
        }

        index
    }

    fn emit_constant(&mut self, constant: Constant) {
        let index = self.make_constant(constant);
        self.emit_bytes(vec![OpCode::Constant as u8, index]);
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

        let line = token.line;
        print!("[line:{line:2}] Compiler Error:");

        if let TokenType::Eof = token.typ3 {
            print!(" at end:");
        } else if let TokenType::Error = token.typ3 {
            // Nothing
        } else {
            print!(" at '{}':", token.lexeme);
        }

        println!(" {msg}");

        self.parser.had_error = true;
    }
}
