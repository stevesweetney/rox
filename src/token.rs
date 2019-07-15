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
    Number(String),
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

pub struct Token {
    tag: TokenType,
    line: u32,
}

impl Token {
    fn new(tag: TokenType, line: u32) -> Self {
        Self { tag, line }
    }
}
