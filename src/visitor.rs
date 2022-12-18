    use crate::ast::{Binary, Grouping, Literal, Unary};

    pub trait Visitor<T> {
        fn visit_binary(&self, b: &Binary) -> T;
        fn visit_grouping(&self, g: &Grouping) -> T;
        fn visit_literal(&self, b: &Literal) -> T;
        fn visit_unary(&self, b: &Unary) -> T;
    }
