#![allow(dead_code, unused_imports)]
use crate::ast::{Binary, Grouping, Literal, Primitive, Print, Statement, Unary, Visitable};
use crate::visitor::Visitor;
pub struct Printer {}
impl Printer {
    pub fn new() -> Printer {
        Printer {}
    }
    fn parenthesize(&self, name: &str, exprs: Vec<Box<&dyn Visitable<String>>>) -> String {
        let mut builder = "".to_owned();
        builder.push('(');
        builder.push_str(name);
        for expr in exprs {
            builder.push(' ');
            builder.push_str(&expr.accept(self));
        }
        builder.push(')');

        builder.clone()
    }
    pub fn print(&self, expr: Box<dyn Visitable<String>>) -> String {
        expr.accept(self)
    }
}

impl Visitor<String> for Printer {
    fn visit_binary(&self, exp: &Binary) -> String {
        match &exp.operator.lexme.clone() {
            Some(res) => self.parenthesize(
                res,
                Vec::from([
                    Box::<&dyn Visitable<String>>::new(&*exp.left),
                    Box::<&dyn Visitable<String>>::new(&*exp.right),
                ]),
            ),
            None => "".to_string(),
        }
    }
    fn visit_grouping(&self, exp: &Grouping) -> String {
        self.parenthesize(
            &"group".to_string(),
            Vec::from([Box::<&dyn Visitable<String>>::new(&*exp.expression)]),
        )
    }
    fn visit_literal(&self, exp: &Literal) -> String {
        match &*exp.value {
            Primitive::String(val) => val.to_string(),
            Primitive::Number(val) => val.to_string(),
            Primitive::Comment(val) => val.to_string(),
            Primitive::Boolean(val) => val.to_string(),
            Primitive::Nil => "nil".to_string(),
        }
    }

    fn visit_unary(&self, exp: &Unary) -> String {
        self.parenthesize(
            &exp.operator.lexme.clone().unwrap(),
            Vec::from([Box::<&dyn Visitable<String>>::new(&*exp.right)]),
        )
    }

    fn visit_print(&self, exp: &Print) {}

    fn visit_stmt(&self, exp: &Statement) {}
}
