use crate::error::report;
use crate::token::{Token, TokenType};

struct Scanner {
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
        self.source.len() >= self.current
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

    pub fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen('(')),
            ')' => self.add_token(TokenType::RightParen(')')),
            '{' => self.add_token(TokenType::LeftBrace('{')),
            '}' => self.add_token(TokenType::RightBrace('}')),
            ',' => self.add_token(TokenType::Comma(',')),
            '.' => self.add_token(TokenType::Dot('.')),
            '-' => self.add_token(TokenType::Minus('-')),
            '+' => self.add_token(TokenType::Plus('+')),
            ';' => self.add_token(TokenType::Semicolon(';')),
            '*' => self.add_token(TokenType::Star('*')),
            '!' => {
                if self.match_char('=') {
                    self.add_token(TokenType::BangEqual("!=".to_owned()))
                } else {
                    self.add_token(TokenType::Bang('!'))
                }
            }
            '=' => {
                if self.match_char('=') {
                    self.add_token(TokenType::EqualEqual("!=".to_owned()))
                } else {
                    self.add_token(TokenType::Equal('!'))
                }
            }
            '<' => {
                if self.match_char('=') {
                    self.add_token(TokenType::LessEqual("!=".to_owned()))
                } else {
                    self.add_token(TokenType::Less('!'))
                }
            }
            '>' => {
                if self.match_char('=') {
                    self.add_token(TokenType::GreaterEqual("!=".to_owned()))
                } else {
                    self.add_token(TokenType::Greater('!'))
                }
            }
            '/' => {
                if self.match_char('/') {
                    while let Some(c) = self.peek() {
                        if c == 'n' {
                            break;
                        }

                        let _ = self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash('/'))
                }
            }
            '\n' => self.line += 1,
            '\t' | '\r' | ' ' => (), // Ignore whitespace
            _ => report(self.line, &format!("Unexpected character: {}", c)),
        };
    }

    pub fn add_token(&mut self, t: TokenType) {
        self.tokens.push(Token::new(t, self.line));
    }
}
