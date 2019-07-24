use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone)]
pub enum TokenType {
    LeftParen(char),
    RightParen(char),
    LeftBrace(char),
    RightBrace(char),
    Comma(char),
    Dot(char),
    Minus(char),
    Plus(char),
    Semicolon(char),
    Slash(char),
    Star(char),
    Bang(char),
    BangEqual(String),
    Equal(char),
    EqualEqual(String),
    Greater(char),
    GreaterEqual(String),
    Less(char),
    LessEqual(String),
    Identifer(String),
    STRING(String),
    Number(f32),
    // Keywords
    And(String),
    Class(String),
    Else(String),
    False(String),
    Fun(String),
    For(String),
    If(String),
    Nil(String),
    Or(String),
    Print(String),
    Return(String),
    Super(String),
    This(String),
    True(String),
    Var(String),
    While(String),
    EOF(String),
}

impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub struct Token {
    tag: TokenType,
    line: u32,
}

impl Token {
    pub fn new(tag: TokenType, line: u32) -> Self {
        Self { tag, line }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.tag, self.line)
    }
}
