#![allow(dead_code, unused_imports)]
use std::collections::HashMap;

use crate::ast::{
    AbstractExpr, AbstractStmt, Assign, Binary, Block, Call, Function, Grouping, If, Literal,
    Logical, Primitive, Print, Return, Statement, TokenType, Unary, Var, Variable,
    Visitable, While,
};
use crate::environment::{self, Environment};
use crate::visitor::Visitor;

pub fn stringify(p: &Primitive) -> String {
    match p {
        Primitive::Nil => "null".to_string(),
        Primitive::String(val) => val.to_string(),
        Primitive::Number(n) => n.to_string(),
        Primitive::Boolean(b) => b.to_string(),
        _ => "".to_string(),
    }
}

#[derive(Clone)]
pub struct Interpreter {
    environment: Box<Environment>,
    return_value: Option<Primitive>,
}

impl Interpreter {
    pub fn new(environment: Box<Environment>) -> Interpreter {
        Interpreter { environment, return_value: None }
    }
    pub fn interpret(mut self, statements: Vec<AbstractStmt>) {
        for statement in statements {
            self.execute(&statement);
        }
    }
    pub fn evaluate(&mut self, exp: &dyn Visitable<Box<Primitive>>) -> Box<Primitive> {
        exp.accept(self)
    }
    pub fn execute(&mut self, stmt: &dyn Visitable<Box<AbstractStmt>>) {
        stmt.accept(self);
    }
    pub fn execute_block(
        &mut self,
        stmts: &Vec<Box<AbstractStmt>>,
        function_params: Option<HashMap<String, Primitive>>,
    ) {
        let function_environment = match function_params {
            Some(params) => params,
            None => HashMap::new(),
        };

        self.environment.push_new_stack(function_environment);

        for stmt in stmts {
            self.execute(stmt);
            if self.return_value.is_some() {
                break;
            }
        }

        self.environment.pop_stack();
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
    fn visit_binary(&mut self, exp: &Binary) -> Box<Primitive> {
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
    fn visit_grouping(&mut self, exp: &Grouping) -> Box<Primitive> {
        let val = &*exp.expression;
        self.evaluate(val)
    }
    fn visit_literal(&mut self, exp: &Literal) -> Box<Primitive> {
        let val = &exp.value;
        Box::new(*val.clone())
    }
    fn visit_logical(&mut self, exp: &Logical) -> Box<Primitive> {
        let val = &*exp.left;
        let left = self.evaluate(val);

        match exp.operator.token_type {
            TokenType::Or => {
                if self.is_truthy(left.clone()) {
                    return left;
                }
            }
            _ => {
                if !self.is_truthy(left.clone()) {
                    return left;
                }
            }
        }

        self.evaluate(&*exp.right)
    }
    fn visit_unary(&mut self, exp: &Unary) -> Box<Primitive> {
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
    fn visit_variable(&mut self, b: &Variable) -> Box<Primitive> {
        self.environment.get(&*b.name)
    }
    fn visit_assign(&mut self, expr: &Assign) -> Box<Primitive> {
        let value = self.evaluate(&*expr.value);
        self.environment.assign(&*expr.name, *value.clone());

        value
    }
    fn visit_call(&mut self, stmt: &Call) -> Box<Primitive> {
        let callee = self.evaluate(&*stmt.callee.clone());

        let mut args: Vec<Primitive> = Vec::new();
        for arg in &stmt.arguments {
            args.push(*self.evaluate(&**arg));
        }

        match *callee {
            Primitive::Callable(function) => {
                if args.len() != function.params.len() {
                    panic!(
                        "Expected {} arguments but got {}",
                        function.params.len(),
                        args.len()
                    );
                }

                let mut params = HashMap::new();
                for (i, param) in function.params.iter().enumerate() {
                    params.insert(param.lexme.clone().unwrap(), args[i].clone());
                }

                self.execute_block(&function.body, Some(params));
                Box::new(self.return_value.take().unwrap_or(Primitive::Nil))
            }
            _ => {
                panic!("Can only call functions and classes");
            }
        }
    }

    // Statement visitors are no-ops in this impl; execution goes through Visitor<Box<AbstractStmt>>
    fn visit_var(&mut self, b: &Var) {}
    fn visit_print(&mut self, b: &Print) {}
    fn visit_stmt(&mut self, b: &Statement) {}
    fn visit_block(&mut self, b: &Block) {}
    fn visit_if(&mut self, b: &If) {}
    fn visit_while(&mut self, b: &While) {}
    fn visit_function(&mut self, b: &Function) {}
    fn visit_return(&mut self, b: &Return) {}
}

impl Visitor<Box<AbstractStmt>> for Interpreter {
    fn visit_binary(&mut self, exp: &Binary) -> Box<AbstractStmt> {
        panic!("Not implemented")
    }
    fn visit_grouping(&mut self, exp: &Grouping) -> Box<AbstractStmt> {
        panic!("Not implemented")
    }
    fn visit_literal(&mut self, exp: &Literal) -> Box<AbstractStmt> {
        panic!("Not implemented")
    }
    fn visit_logical(&mut self, exp: &Logical) -> Box<AbstractStmt> {
        panic!("Not implemented")
    }
    fn visit_unary(&mut self, exp: &Unary) -> Box<AbstractStmt> {
        panic!("Not implemented")
    }
    fn visit_variable(&mut self, b: &Variable) -> Box<AbstractStmt> {
        panic!("Not implemented")
    }
    fn visit_assign(&mut self, exp: &Assign) -> Box<AbstractStmt> {
        panic!("Not implemented")
    }
    fn visit_call(&mut self, exp: &Call) -> Box<AbstractStmt> {
        panic!("Not implemented")
    }

    fn visit_print(&mut self, b: &Print) {
        let value = self.evaluate(&*b.expression.clone());
        println!("{:?}", stringify(&value));
    }
    fn visit_stmt(&mut self, b: &Statement) {
        self.evaluate(&*b.expression);
    }
    fn visit_var(&mut self, b: &Var) {
        let value = match &b.initializer {
            Some(exp) => *self.evaluate(exp),
            None => Primitive::Nil,
        };

        let name: String = String::from(b.name.lexme.as_ref().unwrap());
        self.environment.define(name, value);
    }
    fn visit_block(&mut self, b: &Block) {
        self.execute_block(&b.stmts, None);
    }
    fn visit_if(&mut self, stmt: &If) {
        let cond_result = self.evaluate(&*stmt.condition.clone());
        if self.is_truthy(cond_result) {
            self.execute(&*stmt.then_branch.clone());
            return;
        }
        match stmt.else_branch.clone() {
            Some(else_stmt) => {
                self.execute(&*else_stmt.clone());
            }
            None => {}
        }
    }
    fn visit_while(&mut self, stmt: &While) {
        let mut running = true;
        while running {
            let cond = stmt.condition.clone();
            let eval = self.evaluate(&*cond);

            match *eval {
                Primitive::Boolean(val) => {
                    running = self.is_truthy(Box::new(Primitive::Boolean(val)));

                    if !running {
                        return;
                    }
                }
                _ => {
                    panic!("Loop needs to resolve to Boolean");
                }
            }
            self.execute(&*stmt.body);
        }
    }
    fn visit_function(&mut self, b: &Function) {
        let function = Primitive::Callable(Box::new(b.clone()));
        let name = b.name.lexme.as_ref().unwrap().clone();
        self.environment.define(name, function);
    }
    fn visit_return(&mut self, b: &Return) {
        let value = self.evaluate(&*b.value);
        self.return_value = Some(*value);
    }
}
