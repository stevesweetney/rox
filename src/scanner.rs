use crate::error::report;
use crate::token::{Token, TokenType};
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref KEYWORDS: HashMap<String, TokenType> = {
        let mut m = HashMap::new();
        m.insert("for".to_owned(), TokenType::For);
        m.insert("fun".to_owned(), TokenType::Fun);
        m.insert("if".to_owned(), TokenType::If);
        m.insert("or".to_owned(), TokenType::Or);
        m.insert("nil".to_owned(), TokenType::Nil);
        m.insert("print".to_owned(), TokenType::Print);
        m.insert("return".to_owned(), TokenType::Return);
        m.insert("super".to_owned(), TokenType::Super);
        m.insert("this".to_owned(), TokenType::This);
        m.insert("true".to_owned(), TokenType::True);
        m.insert("false".to_owned(), TokenType::False);
        m.insert("var".to_owned(), TokenType::Var);
        m.insert("while".to_owned(), TokenType::While);
        m.insert("if".to_owned(), TokenType::If);
        m
    };
}

pub struct Scanner {
    source: String,
    chars: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: u32,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        let chars = source.chars().collect();
        Self {
            source,
            chars,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 0,
        }
    }

    pub fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    pub fn advance(&mut self) -> char {
        self.current += 1;
        self.chars[self.current - 1]
    }

    pub fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.chars[self.current] != expected {
            return false;
        }

        self.current += 1;
        true
    }

    pub fn peek(&self) -> Option<char> {
        self.chars.get(self.current).cloned()
    }

    pub fn peek_next(&self) -> Option<char> {
        self.chars.get(self.current + 1).cloned()
    }

    pub fn scan_tokens(&mut self) -> &[Token] {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.add_token(TokenType::EOF);

        &self.tokens
    }

    pub fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '?' => self.add_token(TokenType::QuestionMark),
            ':' => self.add_token(TokenType::Colon),
            '!' => {
                if self.match_char('=') {
                    self.add_token(TokenType::BangEqual)
                } else {
                    self.add_token(TokenType::Bang)
                }
            }
            '=' => {
                if self.match_char('=') {
                    self.add_token(TokenType::EqualEqual)
                } else {
                    self.add_token(TokenType::Equal)
                }
            }
            '<' => {
                if self.match_char('=') {
                    self.add_token(TokenType::LessEqual)
                } else {
                    self.add_token(TokenType::Less)
                }
            }
            '>' => {
                if self.match_char('=') {
                    self.add_token(TokenType::GreaterEqual)
                } else {
                    self.add_token(TokenType::Greater)
                }
            }
            '/' => {
                if self.match_char('/') {
                    while let Some(c) = self.peek() {
                        if c == '\n' {
                            break;
                        }

                        let _ = self.advance();
                    }
                } else if self.match_char('*') {
                    self.handle_block_comment()
                } else {
                    self.add_token(TokenType::Slash)
                }
            }
            '\n' => self.line += 1,
            '\t' | '\r' | ' ' => (), // Ignore whitespace
            '"' => self.handle_string(),
            d if d.is_digit(10) => self.handle_number(),
            a if is_alpha(a) => self.handle_identifier(),
            _ => report(self.line, &format!("Unexpected character: {}", c)),
        };
    }

    pub fn add_token(&mut self, t: TokenType) {
        self.tokens.push(Token::new(t, self.line));
    }

    fn handle_block_comment(&mut self) {
        while let Some(c) = self.peek() {
            match (c, self.peek_next()) {
                ('*', Some('/')) => {
                    // consume both the closing star and slash
                    self.advance();
                    self.advance();
                    break;
                }
                ('/', Some('*')) => {
                    // consume both the opening slash and star
                    self.advance();
                    self.advance();
                    self.handle_block_comment();
                }
                ('\n', _) => {
                    self.line += 1;
                    self.advance();
                }
                _ => {
                    self.advance();
                }
            }
        }
    }

    fn handle_string(&mut self) {
        while let Some(c) = self.peek() {
            match c {
                '"' => break, // closing double quote
                '\n' => self.line += 1,
                _ => {
                    self.advance();
                }
            }
        }

        if self.is_at_end() {
            report(self.line, "Unterminated string");
            return;
        }

        let _ = self.advance();
        let value = self.source[self.start + 1..self.current - 1].to_owned();
        self.add_token(TokenType::STRING(value));
    }

    fn handle_number(&mut self) {
        self.take_numbers();

        match (self.peek(), self.peek_next()) {
            (Some('.'), Some(c)) if c.is_digit(10) => {
                self.advance(); // consume the '.'
                self.take_numbers();
            }
            _ => (),
        }

        let s_literal = &self.source[self.start..self.current];
        if let Ok(n) = s_literal.parse::<f32>() {
            self.add_token(TokenType::Number(n));
        }
    }

    fn handle_identifier(&mut self) {
        while let Some(c) = self.peek() {
            if is_alphanumeric(c) {
                self.advance();
            } else {
                break;
            }
        }

        let literal = self.source[self.start..self.current].to_owned();

        let token_type = match KEYWORDS.get(&literal) {
            Some(t) => (*t).clone(),
            None => TokenType::Identifer(literal),
        };
        self.add_token(token_type);
    }

    fn take_numbers(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_digit(10) {
                self.advance();
            } else {
                break;
            }
        }
    }
}

fn is_alpha(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}

fn is_alphanumeric(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
}
