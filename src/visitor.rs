pub mod visitor {
    use crate::ast::ast::{Binary, Grouping, Literal, Unary};

    pub trait Visitor {
        fn visit_binary(&self, b: &Binary) -> String;
        fn visit_grouping(&self, g: &Grouping) -> String;
        fn visit_literal(&self, b: &Literal) -> String;
        fn visit_unary(&self, b: &Unary) -> String;
    }
}
