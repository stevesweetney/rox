use crate::expr::{Expr, LiteralValue};
use crate::statement::Stmt;
use crate::token::{Token, TokenType};
use std::io::{self, Write};

mod environment;
use environment::Environment;

pub type EvalResult = Result<LiteralValue, String>;
pub type ExecuteResult = Result<(), String>;

pub struct Interpreter<'a> {
    stdout: Box<dyn Write + 'a>,
    environment: Environment,
}

impl<'a> Default for Interpreter<'a> {
    fn default() -> Self {
        let stdout = io::stdout();
        Self::new(stdout)
    }
}

impl<'a> Interpreter<'a> {
    pub fn new(stdout: impl Write + 'a) -> Self {
        Self {
            stdout: Box::new(stdout),
            environment: Environment::new(),
        }
    }

    pub fn interpret(&mut self, statements: &[Stmt]) -> ExecuteResult {
        for s in statements {
            self.execute(s.clone())?;
        }

        Ok(())
    }

    pub fn execute(&mut self, s: Stmt) -> ExecuteResult {
        match s {
            Stmt::Expr(e) => self.evaluate(e).map(|_| ()),
            Stmt::Print(e) => {
                let val = self.evaluate(e)?;
                writeln!(self.stdout, "{}", val).expect("failed to print");
                Ok(())
            }
            Stmt::VarDec { name, initializer } => {
                let value = if let Some(expr) = initializer {
                    Some(self.evaluate(expr)?)
                } else {
                    None
                };

                self.environment.define(name, value);
                Ok(())
            }
        }
    }

    pub fn evaluate(&self, e: Expr) -> EvalResult {
        match e {
            Expr::Literal(v) => Ok(v),
            Expr::Variable(ident) => self.environment.get(&ident).map(|lit_val| lit_val.clone()),
            Expr::Grouping { expr } => self.evaluate(*expr),
            Expr::Unary { operator, operand } => {
                let evaluated = self.evaluate(*operand)?;
                match (evaluated, operator.tag) {
                    (LiteralValue::Number(n), TokenType::Minus) => Ok(LiteralValue::Number(-n)),
                    (_, TokenType::Minus) => {
                        Err("expected a number in negation expression".to_owned())
                    }
                    (v, TokenType::Bang) => Ok(LiteralValue::from(!v.is_truthy())),
                    (_, o) => Err(format!("Did not expected {:#?} in unary expression", o)),
                }
            }
            Expr::Binary {
                left,
                operator,
                right,
            } => self.handle_binary_expression(*left, operator, *right),
            Expr::Ternary {
                condition,
                true_expr,
                false_expr,
            } => {
                let condition_evaluated = self.evaluate(*condition)?;
                match condition_evaluated {
                    LiteralValue::True => self.evaluate(*true_expr),
                    LiteralValue::False => self.evaluate(*false_expr),
                    _ => Err(
                        "expected a boolean expression as condition in ternary statement"
                            .to_owned(),
                    ),
                }
            }
        }
    }

    fn handle_binary_expression(&self, left: Expr, operator: Token, right: Expr) -> EvalResult {
        let left_evaluated = self.evaluate(left)?;
        let right_evaluated = self.evaluate(right)?;

        match (operator.tag, left_evaluated, right_evaluated) {
            (TokenType::Minus, LiteralValue::Number(l_num), LiteralValue::Number(r_num)) => {
                Ok(LiteralValue::Number(l_num - r_num))
            }
            (TokenType::Slash, LiteralValue::Number(l_num), LiteralValue::Number(r_num)) => {
                if r_num == 0.0 {
                    Err("Divide by zero error".to_string())
                } else {
                    Ok(LiteralValue::Number(l_num / r_num))
                }
            }
            (TokenType::Star, LiteralValue::Number(l_num), LiteralValue::Number(r_num)) => {
                Ok(LiteralValue::Number(l_num * r_num))
            }
            (TokenType::Plus, LiteralValue::Number(l_num), LiteralValue::Number(r_num)) => {
                Ok(LiteralValue::Number(l_num + r_num))
            }
            (TokenType::Plus, LiteralValue::STRING(l_str), LiteralValue::STRING(r_str)) => {
                Ok(LiteralValue::STRING(l_str + &r_str))
            }
            (TokenType::Plus, ref left_val, ref right_val)
                if !left_val.is_number() ^ !right_val.is_number() =>
            {
                Err("Can not add a 'String' and a 'Number' in addition operation".to_owned())
            }
            (TokenType::Greater, LiteralValue::Number(l_num), LiteralValue::Number(r_num)) => {
                Ok(LiteralValue::from(l_num > r_num))
            }
            (TokenType::GreaterEqual, LiteralValue::Number(l_num), LiteralValue::Number(r_num)) => {
                Ok(LiteralValue::from(l_num >= r_num))
            }
            (TokenType::Less, LiteralValue::Number(l_num), LiteralValue::Number(r_num)) => {
                Ok(LiteralValue::from(l_num < r_num))
            }
            (TokenType::LessEqual, LiteralValue::Number(l_num), LiteralValue::Number(r_num)) => {
                Ok(LiteralValue::from(l_num <= r_num))
            }
            (TokenType::EqualEqual, left_val, right_val) => {
                Ok(LiteralValue::from(left_val == right_val))
            }
            (TokenType::BangEqual, left_val, right_val) => {
                Ok(LiteralValue::from(left_val != right_val))
            }
            (ref tt, ref left_val, ref right_val)
                if !left_val.is_number() || !right_val.is_number() =>
            {
                Err(format!(
                    "Expected operands to be numbers in {} expression",
                    tt
                ))
            }
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_addition() {
        let interpreter = Interpreter::default();
        let expr_1 = Expr::Binary {
            left: Box::new(Expr::Literal(LiteralValue::Number(10.0))),
            right: Box::new(Expr::Literal(LiteralValue::Number(2.0))),
            operator: Token::new(TokenType::Plus, 0),
        };

        let expr_2 = Expr::Binary {
            left: Box::new(expr_1.clone()),
            right: Box::new(Expr::Literal(LiteralValue::Number(-5.0))),
            operator: Token::new(TokenType::Plus, 0),
        };

        assert_eq!(interpreter.evaluate(expr_1), Ok(LiteralValue::Number(12.0)));
        assert_eq!(interpreter.evaluate(expr_2), Ok(LiteralValue::Number(7.0)));
    }
}
