use std::collections::HashMap;

use crate::ast::{Primitive, Token};

#[derive(Clone)]
pub struct Environment {
    pub enclosing: Option<Box<Environment>>,
    pub values: HashMap<String, Primitive>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            enclosing: None,
            values: HashMap::new(),
        }
    }

    pub fn enclosed_by(enclosing: Box<Environment>) -> Environment {
        Environment {
            enclosing: Some(enclosing),
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: Primitive) {
        self.values.insert(name, value);
    }

    pub fn assign(&mut self, name: &Token, value: Primitive) {
        let key = name.lexme.as_ref().unwrap();
        if self.values.contains_key(key) {
            self.values.insert(key.to_string(), value);
            return;
        }

        match self.enclosing.as_mut() {
            Some(env) => {
                env.assign(name, value);
                return;
            }
            None => {}
        }

        panic!("Undefined variable {}", key)
    }

    pub fn get(&self, name: &Token) -> Box<Primitive> {
        let key = name.lexme.as_ref().unwrap();
        if self.values.contains_key(key) {
            return Box::new(self.values.get(key).unwrap().clone());
        }

        match &self.enclosing {
            Some(env) => {
                return env.get(name);
            }
            None => {}
        }

        panic!("Undefined Variable");
    }
}
