use crate::ast::{
    Assign, Binary, Block, Call, Function, Grouping, If, Literal, Logical, Print, Return,
    Statement, Unary, Var, Variable, While,
};

pub trait Visitor<T> {
    fn visit_binary(&mut self, b: &Binary) -> T;
    fn visit_grouping(&mut self, g: &Grouping) -> T;
    fn visit_literal(&mut self, b: &Literal) -> T;
    fn visit_logical(&mut self, b: &Logical) -> T;
    fn visit_unary(&mut self, b: &Unary) -> T;
    fn visit_variable(&mut self, b: &Variable) -> T;
    fn visit_assign(&mut self, b: &Assign) -> T;
    fn visit_call(&mut self, b: &Call) -> T;

    fn visit_var(&mut self, b: &Var);
    fn visit_stmt(&mut self, b: &Statement);
    fn visit_print(&mut self, b: &Print);
    fn visit_block(&mut self, b: &Block);
    fn visit_if(&mut self, b: &If);
    fn visit_while(&mut self, b: &While);
    fn visit_function(&mut self, b: &Function);
    fn visit_return(&mut self, b: &Return);
}
