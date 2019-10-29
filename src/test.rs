use proptest::prelude::*;
use std::io::Write;

use crate::interpreter::{ExecuteResult, Interpreter};
use crate::parser::Parser;
use crate::scanner::Scanner;

fn scan_parse_and_interpret(source: String, buffer: &mut impl Write) -> ExecuteResult {
    let mut interpreter = Interpreter::new(buffer);
    let mut s = Scanner::new(source);
    let tokens = s.scan_tokens().to_vec();
    let mut parser = Parser::new(tokens);

    parser
        .parse()
        .and_then(|statements| interpreter.interpret(&statements))
}

proptest! {

    #[test]
    fn test_binary_operations(op1 in any::<f32>(), op2 in any::<f32>(), op3 in any::<f32>()) {
        let mut buffer = Vec::new();
        let input = format!("print {} + {} * {};", op1, op2, op3);
        scan_parse_and_interpret(input, &mut buffer).unwrap();

        prop_assert_eq!((op1 + op2 * op3).to_string() + "\n", String::from_utf8(buffer.clone()).unwrap());
        buffer.clear();

        let input = format!("print {} * {} + {};", op1, op2, op3);
        scan_parse_and_interpret(input, &mut buffer).unwrap();

        prop_assert_eq!((op1 * op2 + op3).to_string() + "\n", String::from_utf8(buffer.clone()).unwrap());
        buffer.clear();

        let input = format!("print {} * {} / {};", op1, op2, op3);
        let result = scan_parse_and_interpret(input, &mut buffer);

        if op3 != 0.0 {
            prop_assert_eq!((op1 * op2 / op3).to_string() + "\n", String::from_utf8(buffer.clone()).unwrap());
        } else {
            prop_assert_eq!(Err("Divide by zero error".to_string()), result)
        }
    }

    #[test]
    fn test_ternary_operation(condition in any::<bool>(), expr1 in "[a-zA-Z0-9]+", expr2 in "[a-zA-Z0-9]+") {
        let mut buffer = Vec::new();
        let input = format!(r#"print {} ? "{}" : "{}";"#, condition, expr1, expr2);
        scan_parse_and_interpret(input, &mut buffer).unwrap();

        let eval_expr = if condition {expr1} else {expr2};
        prop_assert_eq!(eval_expr + "\n", String::from_utf8(buffer).unwrap())
    }

    #[test]
    fn test_variable_declarations(op1 in any::<f32>(), op2 in any::<f32>()) {
        let mut buffer = Vec::new();
        let input = format!(r#"var a = {}; var b = {}; print a + b;"#, op1, op2);
        scan_parse_and_interpret(input, &mut buffer).unwrap();

        prop_assert_eq!(format!("{}\n", op1 + op2), String::from_utf8(buffer).unwrap())
    }
}
