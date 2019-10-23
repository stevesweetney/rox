use crate::expr::{Expr, LiteralValue};
use crate::token::{Token, TokenType};

pub type InterpretResult = Result<LiteralValue, String>;

pub fn interpret(e: Expr) -> InterpretResult {
    match e {
        Expr::Literal(v) => Ok(v),
        Expr::Grouping { expr } => interpret(*expr),
        Expr::Unary { operator, operand } => {
            let evaluated = interpret(*operand)?;
            match (evaluated, operator.tag) {
                (LiteralValue::Number(n), TokenType::Minus) => Ok(LiteralValue::Number(-n)),
                (_, TokenType::Minus) => Err("expected a number in negation expression".to_owned()),
                (v, TokenType::Bang) => Ok(LiteralValue::from(!v.is_truthy())),
                (_, o) => Err(format!("Did not expected {:#?} in unary expression", o)),
            }
        }
        Expr::Binary {
            left,
            operator,
            right,
        } => handle_binary_expression(*left, operator, *right),
        Expr::Ternary {
            condition,
            true_expr,
            false_expr,
        } => {
            let condition_evaluated = interpret(*condition)?;
            match condition_evaluated {
                LiteralValue::True => interpret(*true_expr),
                LiteralValue::False => interpret(*false_expr),
                _ => Err(
                    "expected a boolean expression as condition in ternary statement".to_owned(),
                ),
            }
        }
    }
}

fn handle_binary_expression(left: Expr, operator: Token, right: Expr) -> InterpretResult {
    let left_evaluated = interpret(left)?;
    let right_evaluated = interpret(right)?;

    match (operator.tag, left_evaluated, right_evaluated) {
        (TokenType::Minus, LiteralValue::Number(l_num), LiteralValue::Number(r_num)) => {
            Ok(LiteralValue::Number(l_num - r_num))
        }
        (TokenType::Slash, LiteralValue::Number(l_num), LiteralValue::Number(r_num)) => {
            Ok(LiteralValue::Number(l_num / r_num))
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
