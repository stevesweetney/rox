use crate::token::Token;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, PartialEq)]
pub enum Expr {
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
    Ternary {
        condition: Box<Expr>,
        true_expr: Box<Expr>,
        false_expr: Box<Expr>,
    },
}

#[derive(Debug, PartialEq)]
pub enum LiteralValue {
    True,
    False,
    Nil,
    STRING(String),
    Number(f32),
}

impl LiteralValue {
    pub fn is_truthy(&self) -> bool {
        match self {
            LiteralValue::False | LiteralValue::Nil => false,
            _ => true,
        }
    }

    pub fn is_number(&self) -> bool {
        match self {
            LiteralValue::Number(_) => true,
            _ => false,
        }
    }
}

impl From<bool> for LiteralValue {
    fn from(b: bool) -> Self {
        if b {
            LiteralValue::True
        } else {
            LiteralValue::False
        }
    }
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

pub mod print {
    use super::Expr;

    pub fn print_ast(e: &Expr) -> String {
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
            Expr::Ternary {
                condition,
                true_expr,
                false_expr,
            } => parenthesize("?", &[condition, true_expr, false_expr]),
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

    #[cfg(test)]
    fn rpn(e: &Expr) -> String {
        match e {
            Expr::Binary {
                left,
                operator,
                right,
            } => format!("{} {} {}", rpn(left), rpn(right), operator.tag.to_string()),
            Expr::Literal(val) => val.to_string(),
            Expr::Grouping { expr } => rpn(expr),
            Expr::Unary { operator, operand } => {
                format!("{}{}", operator.tag.to_string(), rpn(operand))
            }
            _ => unreachable!(),
        }
    }

    #[cfg(test)]
    mod test {
        use super::{print_ast, rpn, Expr};
        use crate::expr::LiteralValue;
        use crate::token::{Token, TokenType};

        #[test]
        fn test_print_ast() {
            let minus_operator = Token::new(TokenType::Minus, 1);
            let mul_operator = Token::new(TokenType::Star, 1);
            let lit_123 = LiteralValue::Number(123.0);
            let lit_4567 = LiteralValue::Number(45.67);
            let expr = Expr::Binary {
                left: Box::new(Expr::Unary {
                    operator: minus_operator,
                    operand: Box::new(Expr::Literal(lit_123)),
                }),
                operator: mul_operator,
                right: Box::new(Expr::Grouping {
                    expr: Box::new(Expr::Literal(lit_4567)),
                }),
            };

            assert_eq!(print_ast(&expr), "(* (- 123) (grouping 45.67))")
        }

        #[test]
        fn test_rpn() {
            let minus_operator = Token::new(TokenType::Minus, 1);
            let mul_operator = Token::new(TokenType::Star, 1);
            let plus_operator = Token::new(TokenType::Plus, 1);
            let expr = Expr::Binary {
                left: Box::new(Expr::Binary {
                    left: Box::new(Expr::Literal(LiteralValue::Number(1.0))),
                    operator: plus_operator,
                    right: Box::new(Expr::Literal(LiteralValue::Number(2.0))),
                }),
                operator: mul_operator,
                right: Box::new(Expr::Binary {
                    left: Box::new(Expr::Literal(LiteralValue::Number(4.0))),
                    operator: minus_operator,
                    right: Box::new(Expr::Literal(LiteralValue::Number(3.0))),
                }),
            };

            assert_eq!(rpn(&expr), "1 2 + 4 3 - *")
        }
    }
}
