use crate::token::Token;

enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Grouping {
        expr: Box<Expr>,
    },
    Literal(LiteralValue),
    Unary {
        operator: Token,
        operand: Box<Expr>,
    },
}

enum LiteralValue {
    True,
    False,
    Nil,
    STRING(String),
    Number(f32),
}
