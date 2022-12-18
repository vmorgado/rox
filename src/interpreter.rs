#![allow(dead_code, unused_imports)]
use crate::ast::{AbstractExpr, Binary, Expr, Grouping, Literal, Primitive, TokenType, Unary};
use crate::visitor::Visitor;

pub struct Interpreter {}
impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {}
    }
    pub fn interpret(self, exp: Box<dyn Expr<Box<Primitive>>>) {
        let value = self.evaluate(&*exp);
        println!("{:?}", self.stringify(value));
    }

    pub fn stringify(self, p: Box<Primitive>) -> String {
        match *p {
            Primitive::Nil => "null".to_string(),
            Primitive::String(val) => val,
            Primitive::Number(n) => n.to_string(),
            Primitive::Boolean(b) => b.to_string(),
            _ => "".to_string(),
        }
    }

    pub fn evaluate(&self, exp: &dyn Expr<Box<Primitive>>) -> Box<Primitive> {
        exp.accept(self)
    }

    pub fn is_truthy(&self, p: Box<Primitive>) -> bool {
        match *p {
            Primitive::Nil => false,
            Primitive::Boolean(val) => val,
            _ => true,
        }
    }
}

impl Visitor<Box<Primitive>> for Interpreter {
    fn visit_binary(&self, exp: &Binary) -> Box<Primitive> {
        let left = match &*exp.left {
            AbstractExpr::Literal(l) => l.value.clone(),
            val => self.evaluate(&val.clone()),
        };

        let right = match &*exp.right {
            AbstractExpr::Literal(l) => l.value.clone(),
            val => self.evaluate(&val.clone()),
        };

        match exp.operator.token_type {
                TokenType::Minus => match *left {
                    Primitive::Number(left_val) => match *right {
                        Primitive::Number(right_val) => {
                            return Box::new(Primitive::Number(left_val - right_val))
                        }
                        Primitive::String(right_val_str) => {
                            let right_val = match right_val_str.parse::<f64>() {
                                Ok(v) => v,
                                _ => panic!("Cannot cast the string to number for right value for subtraction")
                            };
                            return Box::new(Primitive::Number(left_val - right_val));
                        }
                        _ => panic!("TODO: Not implemented - Second value from subtraction not casted to Number"),
                    },
                    Primitive::String(left_val_str) => match *right {
                        Primitive::Number(right_val) => {
                            let left_val = match left_val_str.parse::<f64>() {
                                Ok(v) => v,
                                _ => panic!("Cannot cast string to number for left value for subtraction")
                            };

                            return Box::new(Primitive::Number(left_val - right_val));
                        }
                        Primitive::String(right_val_str) => {
                            let left_val = match left_val_str.parse::<f64>() {
                                Ok(v) => v,
                                _ => panic!("Cannot cast string to number for left value for subtraction")
                            };
                            let right_val = match right_val_str.parse::<f64>() {
                                Ok(v) => v,
                                _ => panic!("Cannot cast the string to number for right value for subtraction")
                            };
                            return Box::new(Primitive::Number(left_val - right_val));
                        }
                        _ => panic!("TODO: Not implemented - Second value from subtraction not casted to Number"),
                    }
                    _ => panic!("TODO: Not implemented - First value from subtraction not casted to Number"),
                },
                TokenType::Slash => match *left {
                    Primitive::Number(left_val) => match *right {
                        Primitive::Number(right_val) => {
                            return Box::new(Primitive::Number(left_val / right_val))
                        }
                        _ => panic!("TODO: Not implemented - Second value from division not casted to Number"),
                    },
                        _ => panic!("TODO: Not implemented - First value from division not casted to Number"),
                },
                TokenType::Star => match *left {
                    Primitive::Number(left_val) => match *right {
                        Primitive::Number(right_val) => {
                            return Box::new(Primitive::Number(left_val * right_val))
                        }
                        _ => panic!("TODO: Not implemented - Second value from multiplication not casted to Number"),
                    },
                        _ => panic!("TODO: Not implemented - First value from multiplication not casted to Number"),
                },
                TokenType::Plus => match *left {
                    Primitive::Number(left_val) => match *right {
                        Primitive::Number(right_val) => {
                            return Box::new(Primitive::Number(left_val + right_val))
                        }
                        Primitive::String(right_val_str) => {
                            let right_val = match right_val_str.parse::<f64>() {
                                Ok(v) => v,
                                _ => panic!("Cannot cast the string to number for right value for sum")
                            };
                            return Box::new(Primitive::Number(left_val + right_val));
                        }
                        _ => panic!("TODO: Not implemented - Second value from sum not casted to Number"),
                    },
                    Primitive::String(left_val_str) => match *right {
                        Primitive::Number(right_val) => {
                            let left_val = match left_val_str.parse::<f64>() {
                                Ok(v) => v,
                                _ => panic!("Cannot cast string to number for left value for sum")
                            };

                            return Box::new(Primitive::Number(left_val + right_val));
                        }
                        Primitive::String(right_val_str) => {
                            let left_val = match left_val_str.parse::<f64>() {
                                Ok(v) => v,
                                _ => panic!("Cannot cast string to number for left value for sum")
                            };
                            let right_val = match right_val_str.parse::<f64>() {
                                Ok(v) => v,
                                _ => panic!("Cannot cast the string to number for right value for sum")
                            };
                            return Box::new(Primitive::Number(left_val + right_val));
                        }
                        _ => panic!("TODO: Not implemented - Second value from sum not casted to Number"),
                    }
                    _ => panic!("TODO: Not implemented - First value from sum not casted to Number"),
                },
                TokenType::Greater => match *left {
                    Primitive::Number(left_val) => match *right {
                        Primitive::Number(right_val) => {
                            return Box::new(Primitive::Boolean(left_val > right_val))
                        }
                        _ => panic!("TODO: Not implemented - Can only compare Numbers"),
                    },
                        _ => panic!("TODO: Not implemented - Can only compare Numbers"),
                },
                TokenType::GreaterEqual => match *left {
                    Primitive::Number(left_val) => match *right {
                        Primitive::Number(right_val) => {
                            return Box::new(Primitive::Boolean(left_val >= right_val))
                        }
                        _ => panic!("TODO: Not implemented - Can only compare Numbers"),
                    },
                        _ => panic!("TODO: Not implemented - Can only compare Numbers"),
                },
                TokenType::Less => match *left {
                    Primitive::Number(left_val) => match *right {
                        Primitive::Number(right_val) => {
                            return Box::new(Primitive::Boolean(left_val < right_val))
                        }
                        _ => panic!("TODO: Not implemented - Can only compare Numbers"),
                    },
                        _ => panic!("TODO: Not implemented - Can only compare Numbers"),
                },
                TokenType::LessEqual => match *left {
                    Primitive::Number(left_val) => match *right {
                        Primitive::Number(right_val) => {
                            return Box::new(Primitive::Boolean(left_val <= right_val))
                        }
                        _ => panic!("TODO: Not implemented - Can only compare Numbers"),
                    },
                        _ => panic!("TODO: Not implemented - Can only compare Numbers"),
                },
                TokenType::BangEqual => match *left {
                    Primitive::Number(left_val) => match *right {
                        Primitive::Number(right_val) => {
                            return Box::new(Primitive::Boolean(left_val != right_val))
                        }
                        _ => panic!("TODO: Not implemented - Can only compare Numbers"),
                    },
                    left_val => return Box::new(Primitive::Boolean(left_val != *right)),
                },
                TokenType::EqualEqual => match *left {
                    Primitive::Number(left_val) => match *right {
                        Primitive::Number(right_val) => {
                            return Box::new(Primitive::Boolean(left_val == right_val))
                        }
                        _ => panic!("TODO: Not implemented - Can only compare Numbers"),
                    },
                    left_val => return Box::new(Primitive::Boolean(left_val == *right)),
                },
                _ => {}
            };

        Box::new(Primitive::Nil)
    }

    fn visit_grouping(&self, exp: &Grouping) -> Box<Primitive> {
        let val = &*exp.expression;
        self.evaluate(val)
    }

    fn visit_literal(&self, exp: &Literal) -> Box<Primitive> {
        let val = &exp.value;
        Box::new(*val.clone())
    }

    fn visit_unary(&self, exp: &Unary) -> Box<Primitive> {
        let val = &*exp.right;
        let right = self.evaluate(val);

        match *right {
            Primitive::Number(val) => match exp.operator.token_type {
                TokenType::Minus => Box::new(Primitive::Number(-val)),
                _ => Box::new(Primitive::Nil),
            },
            _ => match exp.operator.token_type {
                TokenType::Bang => Box::new(Primitive::Boolean(!self.is_truthy(right))),
                _ => Box::new(Primitive::Nil),
            },
        }
    }
}
