use std::collections::HashMap;

use crate::ast::{Primitive, Token};

#[derive(Clone)]
pub struct Environment {
    pub stack: Vec<HashMap<String, Primitive>>,
}

impl Environment {
    pub fn new() -> Environment {
        let values = HashMap::new();
        Environment {
            stack: Vec::from([values]),
        }
    }

    pub fn push_new_stack(&mut self) {
        let values = HashMap::new();
        self.stack.push(values);
    }

    pub fn pop_stack(&mut self) {
        self.stack.pop();
    }

    pub fn define(&mut self, name: String, value: Primitive) {
        match self.stack.get_mut(0) {
            Some(hash) => {
                hash.insert(name, value);
            }
            None => {
                panic!("No stack initialized?");
            }
        }
    }

    pub fn assign(&mut self, name: &Token, value: Primitive) {
        let key = name.lexme.as_ref().unwrap();

        for i in 0..self.stack.len() {
            let values = self.stack.get_mut(i);
            if let Some(hash) = values {
                hash.insert(key.to_string(), value.clone());
                return;
            }
        }

        self.define(key.to_string(), value);
        return;
    }

    pub fn get(&self, name: &Token) -> Box<Primitive> {
        let key = name.lexme.as_ref().unwrap();

        for values in &self.stack {
            if values.contains_key(key) {
                return Box::new(values.get(key).unwrap().clone());
            }
        }

        panic!("Undefined Variable");
    }
}
