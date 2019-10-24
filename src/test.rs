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
        let result = scan_parse_and_interpret(input).unwrap().to_string();

        prop_assert_eq!((op1 * op2 / op3).to_string(), result);
    }
}
