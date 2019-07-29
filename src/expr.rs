use crate::token::Token;
use std::fmt::{self, Display, Formatter};

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

pub enum LiteralValue {
    True,
    False,
    Nil,
    STRING(String),
    Number(f32),
}

impl Display for LiteralValue {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            LiteralValue::True => write!(f, "true"),
            LiteralValue::False => write!(f, "false"),
            LiteralValue::Nil => write!(f, "nil"),
            LiteralValue::STRING(s) => write!(f, "{}", s),
            LiteralValue::Number(s) => write!(f, "{}", s.to_string()),
        }
    }
}

mod print {
    use super::Expr;

    fn print_ast(e: &Expr) -> String {
        match e {
            Expr::Binary {
                left,
                operator,
                right,
            } => parenthesize(&operator.tag.to_string(), &[left, right]),
            Expr::Unary { operator, operand } => {
                parenthesize(&operator.tag.to_string(), &[operand])
            }
            Expr::Literal(val) => val.to_string(),
            Expr::Grouping { expr } => parenthesize("grouping", &[expr]),
        }
    }

    fn parenthesize(name: &str, exprs: &[&Expr]) -> String {
        let mut res = format!("({}", name);
        for e in exprs {
            res += " ";
            res += &print_ast(e);
        }
        res.push(')');
        res
    }
}
