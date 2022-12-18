#![allow(dead_code, unused_imports)]
use crate::ast::{Binary, Expr, Grouping, Literal, Primitive, Unary};
use crate::visitor::Visitor;
pub struct Printer {}
impl Printer {
    pub fn new() -> Printer {
        Printer {}
    }
    fn parenthesize(&self, name: &str, exprs: Vec<Box<&dyn Expr<String>>>) -> String {
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
    pub fn print(&self, expr: Box<dyn Expr<String>>) -> String {
        expr.accept(self)
    }
}

impl Visitor<String> for Printer {
    fn visit_binary(&self, exp: &Binary) -> String {
        match &exp.operator.lexme.clone() {
            Some(res) => self.parenthesize(
                res,
                Vec::from([
                    Box::<&dyn Expr<String>>::new(&*exp.left),
                    Box::<&dyn Expr<String>>::new(&*exp.right),
                ]),
            ),
            None => "".to_string(),
        }
    }
    fn visit_grouping(&self, exp: &Grouping) -> String {
        self.parenthesize(
            &"group".to_string(),
            Vec::from([Box::<&dyn Expr<String>>::new(&*exp.expression)]),
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
            Vec::from([Box::<&dyn Expr<String>>::new(&*exp.right)]),
        )
    }
}
