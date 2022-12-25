#![allow(dead_code, unused_imports)]
use crate::visitor::Visitor;

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub lexme: Option<String>,
    pub literal: Option<Primitive>,
    pub line: usize,
}
#[derive(PartialEq, Clone, Debug)]
pub enum Primitive {
    Nil,
    Boolean(bool),
    String(String),
    Number(f64),
    Comment(String),
}

#[derive(Debug, Clone)]
pub enum AbstractExpr {
    Assign(Assign),
    Binary(Binary),
    Grouping(Grouping),
    Literal(Literal),
    Unary(Unary),
    Variable(Variable),
}

pub trait Visitable<T> {
    fn accept(&self, v: &mut dyn Visitor<T>) -> T;
}

#[derive(Debug, Clone)]
pub enum AbstractStmt {
    Statement(Statement),
    Print(Print),
    Var(Var),
}

#[derive(Debug, Clone)]
pub struct Statement {
    pub expression: Box<AbstractExpr>,
}

#[derive(Debug, Clone)]
pub struct Print {
    pub expression: Box<AbstractExpr>,
}

#[derive(Debug, Clone)]
pub struct Var {
    pub name: Box<Token>,
    pub initializer: Option<AbstractExpr>,
}

#[derive(Debug, Clone)]
pub struct Assign {
    pub name: Box<Token>,
    pub value: Box<AbstractExpr>,
}

#[derive(Debug, Clone)]
pub struct Binary {
    pub operator: Box<Token>,
    pub left: Box<AbstractExpr>,
    pub right: Box<AbstractExpr>,
}

#[derive(Debug, Clone)]
pub struct Grouping {
    pub expression: Box<AbstractExpr>,
}

#[derive(Debug, Clone)]
pub struct Literal {
    pub value: Box<Primitive>,
}

#[derive(Debug, Clone)]
pub struct Unary {
    pub right: Box<AbstractExpr>,
    pub operator: Box<Token>,
}
#[derive(Debug, Clone)]
pub struct Variable {
    pub name: Box<Token>,
}

impl Visitable<String> for AbstractStmt {
    fn accept(&self, v: &mut dyn Visitor<String>) -> String {
        match self {
            AbstractStmt::Statement(exp) => v.visit_stmt(exp),
            AbstractStmt::Print(val) => v.visit_print(val),
            AbstractStmt::Var(val) => v.visit_var(val),
        };
        "".to_string()
    }
}

impl Visitable<Box<AbstractStmt>> for AbstractStmt {
    fn accept(&self, v: &mut dyn Visitor<Box<AbstractStmt>>) -> Box<AbstractStmt> {
        match self {
            AbstractStmt::Statement(exp) => v.visit_stmt(exp),
            AbstractStmt::Print(val) => v.visit_print(val),
            AbstractStmt::Var(val) => v.visit_var(val),
        };
        Box::new(AbstractStmt::Print(Print {
            expression: Box::new(AbstractExpr::Literal(Literal {
                value: Box::new(Primitive::String("".to_string())),
            })),
        }))
    }
}

impl Visitable<Box<Primitive>> for AbstractStmt {
    fn accept(&self, v: &mut dyn Visitor<Box<Primitive>>) -> Box<Primitive> {
        match self {
            AbstractStmt::Statement(exp) => v.visit_stmt(exp),
            AbstractStmt::Print(val) => v.visit_print(val),
            AbstractStmt::Var(val) => v.visit_var(val),
        };
        Box::new(Primitive::Boolean(true))
    }
}
impl Visitable<String> for AbstractExpr {
    fn accept(&self, v: &mut dyn Visitor<String>) -> String {
        match self {
            AbstractExpr::Binary(val) => v.visit_binary(val),
            AbstractExpr::Grouping(val) => v.visit_grouping(val),
            AbstractExpr::Literal(val) => v.visit_literal(val),
            AbstractExpr::Unary(val) => v.visit_unary(val),
            AbstractExpr::Variable(val) => v.visit_variable(val),
            AbstractExpr::Assign(val) => v.visit_assign(val),
        }
    }
}

impl Visitable<Box<Primitive>> for AbstractExpr {
    fn accept(&self, v: &mut dyn Visitor<Box<Primitive>>) -> Box<Primitive> {
        match self {
            AbstractExpr::Binary(val) => v.visit_binary(val),
            AbstractExpr::Grouping(val) => v.visit_grouping(val),
            AbstractExpr::Literal(val) => v.visit_literal(val),
            AbstractExpr::Unary(val) => v.visit_unary(val),
            AbstractExpr::Variable(val) => v.visit_variable(val),
            AbstractExpr::Assign(val) => v.visit_assign(val),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    SemiColon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
    Comment,
}
