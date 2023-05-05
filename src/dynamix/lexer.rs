use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    LCurly,
    RCurly,
    LParen,
    RParen,
    Comma,
    Dot,
    Semicolon,
    Plus,
    Minus,
    Slash,
    Star,

    Bang,
    BangEq,
    Eq,
    EqEq,
    Gt,
    Gte,
    Lt,
    Lte,

    Ident,
    String,
    Number,
    Char,

    And,
    Or,
    Struct,
    SSelf,
    Super,
    Fun,
    Let,
    For,
    While,
    If,
    Else,
    Return,
    Print,
    True,
    False,
    Null,

    Error,
    Eof,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub typ3: TokenType,
    pub lexeme: String,
    pub line: usize,
}

impl Token {
    pub fn new() -> Self {
        Self {
            typ3: TokenType::Ident,
            lexeme: String::new(),
            line: 0,
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{:8} line:{:2} {:?}]", self.lexeme, self.line, self.typ3)
    }
}

pub struct Lexer {
    source: String,
    chars: Vec<char>,
    start: usize,
    cursor: usize,
    line: usize,
    keywords: HashMap<String, TokenType>,
}

impl Lexer {
    pub fn new(source: &String) -> Self {
        Self {
            source: source.clone(),
            chars: source.chars().collect(),
            start: 0,
            cursor: 0,
            line: 1,
            keywords: vec![
                ("print", TokenType::Print),
                ("if", TokenType::If),
                ("else", TokenType::Else),
                ("&&", TokenType::And),
                ("||", TokenType::Or),
                ("let", TokenType::Let),
                ("struct", TokenType::Struct),
                ("self", TokenType::SSelf),
                ("while", TokenType::While),
                ("for", TokenType::For),
                ("return", TokenType::Return),
                ("fun", TokenType::Fun),
                ("true", TokenType::True),
                ("false", TokenType::False),
                ("null", TokenType::Null),
            ]
            .into_iter()
            .map(|(k, v)| (String::from(k), v))
            .collect(),
        }
    }

    fn advance(&mut self) -> char {
        self.cursor += 1;
        self.chars[self.cursor - 1]
    }

    fn peek(&self) -> char {
        self.chars[self.cursor]
    }

    fn peek_next(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        self.chars[self.cursor + 1]
    }

    fn trim(&mut self) {
        loop {
            let c = self.peek();
            match c {
                ' ' | '\t' | '\r' => {
                    self.advance();
                    continue;
                }
                '/' => {
                    if self.peek_next() == '/' {
                        while self.peek() != '\n' && !self.is_at_end() {
                            self.advance();
                        }
                    } else {
                        return;
                    }
                }
                '\n' => {
                    self.line += 1;
                    self.advance();
                    continue;
                }
                _ => return,
            }
        }
    }

    fn is_at_end(&self) -> bool {
        self.cursor >= self.chars.len()
    }

    fn matches(&mut self, c: char) -> bool {
        if self.is_at_end() {
            false
        } else if self.chars[self.cursor] == c {
            self.advance();
            true
        } else {
            false
        }
    }

    fn make_token(&self, typ3: TokenType) -> Token {
        Token {
            typ3,
            lexeme: String::from(&self.source[self.start..self.cursor]),
            line: self.line,
        }
    }

    fn error_token(&mut self, msg: String) -> Token {
        Token {
            typ3: TokenType::Error,
            lexeme: msg,
            line: self.line,
        }
    }

    fn char(&mut self) -> Option<Token> {
        self.start += 1;
        self.advance();

        let res = Some(self.make_token(TokenType::Char));

        if self.peek() != '\'' {
            return Some(self.error_token("Unterminated character literal".to_string()));
        }

        self.advance();
        res
    }

    fn string(&mut self) -> Option<Token> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }

            self.advance();
        }

        if self.is_at_end() {
            return Some(self.error_token("Unterminated string literal".to_string()));
        }

        self.advance();
        Some(self.make_token(TokenType::String))
    }

    fn number(&mut self) -> Option<Token> {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        if self.peek() == '.' {
            self.advance();
            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        Some(self.make_token(TokenType::Number))
    }

    fn identifier(&mut self) -> Option<Token> {
        while self.peek().is_ascii_alphanumeric() || "_&|".contains(self.peek()) {
            self.advance();
        }

        let value = String::from(&self.source[self.start..self.cursor]);

        let typ3 = if self.keywords.iter().any(|(s, t)| s == &value) {
            *self.keywords.get(&value).unwrap()
        } else {
            TokenType::Ident
        };

        Some(self.make_token(typ3))
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.trim();

        self.start = self.cursor;
        let c = self.advance();
        if self.is_at_end() {
            return Some(self.make_token(TokenType::Eof));
        }

        if c.is_ascii_digit() {
            return self.number();
        } else if c.is_alphabetic() || "_&|".contains(c) {
            return self.identifier();
        }

        return match c {
            '{' => Some(self.make_token(TokenType::LCurly)),
            '}' => Some(self.make_token(TokenType::RCurly)),
            '(' => Some(self.make_token(TokenType::LParen)),
            ')' => Some(self.make_token(TokenType::RParen)),
            ';' => Some(self.make_token(TokenType::Semicolon)),
            ',' => Some(self.make_token(TokenType::Comma)),
            '.' => Some(self.make_token(TokenType::Dot)),
            '-' => Some(self.make_token(TokenType::Minus)),
            '+' => Some(self.make_token(TokenType::Plus)),
            '/' => Some(self.make_token(TokenType::Slash)),
            '*' => Some(self.make_token(TokenType::Star)),
            '!' => {
                let typ3 = if self.matches('=') {
                    TokenType::BangEq
                } else {
                    TokenType::Bang
                };
                Some(self.make_token(typ3))
            }
            '=' => {
                let typ3 = if self.matches('=') {
                    TokenType::EqEq
                } else {
                    TokenType::Eq
                };
                Some(self.make_token(typ3))
            }
            '<' => {
                let typ3 = if self.matches('=') {
                    TokenType::Lte
                } else {
                    TokenType::Lt
                };
                Some(self.make_token(typ3))
            }
            '>' => {
                let typ3 = if self.matches('=') {
                    TokenType::Gte
                } else {
                    TokenType::Gt
                };
                Some(self.make_token(typ3))
            }
            '\'' => self.char(),
            '"' => self.string(),
            _ => {
                let err = format!("Unexpected character '{c}'");
                Some(self.error_token(err))
            }
        };
    }
}
