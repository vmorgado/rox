use std::collections::HashMap;

use crate::ast::{Primitive, Token};

#[derive(Clone)]
pub struct Environment {
    pub values: HashMap<String, Primitive>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: Primitive) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &Token) -> Box<Primitive> {
        let key = name.lexme.as_ref().unwrap();
        if self.values.contains_key(key) {
            return Box::new(self.values.get(key).unwrap().clone());
        }
        // TODO: handle error here better
        panic!("Undefined Variable");
    }
}
