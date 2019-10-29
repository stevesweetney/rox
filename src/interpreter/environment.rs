use crate::expr::LiteralValue;
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

    pub fn get(&self, key: &str) -> Result<&LiteralValue, String> {
        self.globals
            .get(key)
            .ok_or_else(|| format!("Variable '{}' is not defined", key))
    }
}
