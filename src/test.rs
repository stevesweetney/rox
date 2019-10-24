use proptest::prelude::*;

use crate::interpreter::{interpret, InterpretResult};
use crate::parser::Parser;
use crate::scanner::Scanner;

fn scan_parse_and_interpret(source: String) -> InterpretResult {
    let mut s = Scanner::new(source);
    let tokens = s.scan_tokens().to_vec();
    let mut parser = Parser::new(tokens);

    parser.parse().and_then(interpret)
}

proptest! {

    #[test]
    fn test_binary_operations(op1 in any::<f32>(), op2 in any::<f32>(), op3 in any::<f32>()) {
        let input = format!("{} + {} * {}", op1, op2, op3);
        let result = scan_parse_and_interpret(input).unwrap().to_string();

        prop_assert_eq!((op1 + op2 * op3).to_string(), result);

        let input = format!("{} * {} + {}", op1, op2, op3);
        let result = scan_parse_and_interpret(input).unwrap().to_string();

        prop_assert_eq!((op1 * op2 + op3).to_string(), result);

        let input = format!("{} * {} / {}", op1, op2, op3);
        let result = scan_parse_and_interpret(input).map(|val| val.to_string());

        if op3 != 0.0 {
            prop_assert_eq!((op1 * op2 / op3).to_string(), result.unwrap());
        } else {
            prop_assert_eq!(Err("Divide by zero error".to_string()), result)
        }
    }

    #[test]
    fn test_ternary_operation(condition in any::<bool>(), expr1 in "[a-zA-Z0-9]+", expr2 in "[a-zA-Z0-9]+" ) {
        let input = format!(r#"{} ? "{}" : "{}""#, condition, expr1, expr2);
        let result = scan_parse_and_interpret(input).unwrap().to_string();

        prop_assert_eq!(if condition {expr1} else {expr2}, result)
    }
}
