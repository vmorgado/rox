pub mod printer {
    use crate::ast::ast::{Binary, Expr, Grouping, Literal, Primitive, Unary};
    use crate::visitor::visitor::Visitor;
    pub struct Printer {}
    impl Printer {
        pub fn new() -> Printer {
            Printer {}
        }
        fn parenthesize(self: &Self, name: &str, exprs: Vec<Box<&dyn Expr>>) -> String {
            let mut builder = "".to_owned();
            builder.push_str("(");
            builder.push_str(name);
            for expr in exprs {
                builder.push_str(" ");
                builder.push_str(&expr.accept(self));
            }
            builder.push_str(")");

            builder.clone()
        }
        pub fn print(self: &Self, expr: Box<dyn Expr>) -> String {
            expr.accept(self)
        }
    }

    impl Visitor for Printer {
        fn visit_binary(self: &Self, exp: &Binary) -> String {
            match &exp.operator.lexme.clone() {
                Some(res) => self.parenthesize(
                    res,
                    Vec::from([
                        Box::<&dyn Expr>::new(&*exp.left),
                        Box::<&dyn Expr>::new(&*exp.right),
                    ]),
                ),
                None => "".to_string(),
            }
        }
        fn visit_grouping(self: &Self, exp: &Grouping) -> String {
            self.parenthesize(
                &"group".to_string(),
                Vec::from([Box::<&dyn Expr>::new(&*exp.expression)]),
            )
        }
        fn visit_literal(self: &Self, exp: &Literal) -> String {
            match &*exp.value {
                Primitive::String(val) => val.to_string(),
                Primitive::Number(val) => val.to_string(),
                Primitive::Comment(val) => val.to_string(),
                Primitive::Boolean(val) => val.to_string(),
                Primitive::Nil => "nil".to_string(),
            }
        }

        fn visit_unary(self: &Self, exp: &Unary) -> String {
            self.parenthesize(
                &exp.operator.lexme.clone().unwrap(),
                Vec::from([Box::<&dyn Expr>::new(&*exp.right)]),
            )
        }
    }
}
