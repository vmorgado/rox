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

    pub fn push_new_stack(&mut self, values: HashMap<String, Primitive>) {
        self.stack.push(values);
    }

    pub fn pop_stack(&mut self) {
        self.stack.pop();
    }

    pub fn define(&mut self, name: String, value: Primitive) {
        match self.stack.last_mut() {
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

        for values in self.stack.iter_mut().rev() {
            if values.contains_key(key) {
                values.insert(key.to_string(), value);
                return;
            }
        }

        self.define(key.to_string(), value);
    }

    pub fn get(&self, name: &Token) -> Box<Primitive> {
        let key = name.lexme.as_ref().unwrap();

        for values in self.stack.iter().rev() {
            if values.contains_key(key) {
                return Box::new(values.get(key).unwrap().clone());
            }
        }

        panic!("Undefined Variable");
    }
}
