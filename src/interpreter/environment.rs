use crate::expr::LiteralValue;
use crate::token::Token;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Environment {
    globals: HashMap<String, LiteralValue>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            globals: HashMap::new(),
        }
    }

    pub fn define(&mut self, key: String, value: Option<LiteralValue>) {
        self.globals
            .insert(key, value.unwrap_or_else(|| LiteralValue::Nil));
    }

    pub fn get(&self, token: &Token) -> Result<&LiteralValue, String> {
        let var_name = &token
            .tag
            .get_identifier_value()
            .expect("expected identifier token");
        self.globals.get(var_name).ok_or_else(|| {
            format!(
                "[line {}] Error: variable '{}' is not defined",
                token.line, var_name
            )
        })
    }

    pub fn assign(&mut self, token: &Token, value: LiteralValue) -> Result<LiteralValue, String> {
        let var_name = token
            .tag
            .get_identifier_value()
            .expect("expected identifier token");

        if self.globals.contains_key(&var_name) {
            self.globals.insert(var_name, value.clone());
            Ok(value)
        } else {
            Err(format!(
                "[line {}] Error: variable '{}' is not defined",
                token.line, var_name
            ))
        }
    }
}
