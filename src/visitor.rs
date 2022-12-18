use crate::ast::{Binary, Grouping, Literal, Print, Statement, Unary};

pub trait Visitor<T> {
    fn visit_binary(&self, b: &Binary) -> T;
    fn visit_grouping(&self, g: &Grouping) -> T;
    fn visit_literal(&self, b: &Literal) -> T;
    fn visit_unary(&self, b: &Unary) -> T;
    fn visit_stmt(&self, b: &Statement);
    fn visit_print(&self, b: &Print);
}
