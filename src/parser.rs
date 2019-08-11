use crate::expr::Expr;
use crate::token::{Token, TokenType};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    fn match_token(&mut self, types: &[TokenType]) -> Option<&Token> {
        let matched = self
            .tokens
            .get(self.current)
            .filter(|token| types.contains(&token.tag));

        if matched.is_some() {
            self.current += 1;
        }
        matched
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    fn is_at_end(&self) -> bool {
        match self.peek() {
            Some(token) if token.tag.eq(&TokenType::EOF) => true,
            _ => false,
        }
    }
}
