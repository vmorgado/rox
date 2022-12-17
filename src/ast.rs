#[allow(dead_code, unused_variables)]
pub mod ast {

    use crate::visitor::visitor::Visitor;

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
        Binary(Binary),
        Grouping(Grouping),
        Literal(Literal),
        Unary(Unary),
    }

    pub trait Expr<T> {
        fn accept(&self, v: &dyn Visitor<T>) -> T;
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

    impl Expr<String> for AbstractExpr {
        fn accept(self: &Self, v: &dyn Visitor<String>) -> String {
            match self {
                AbstractExpr::Binary(val) => v.visit_binary(val),
                AbstractExpr::Grouping(val) => v.visit_grouping(val),
                AbstractExpr::Literal(val) => v.visit_literal(val),
                AbstractExpr::Unary(val) => v.visit_unary(val),
            }
        }
    }

    impl Expr<Box<Primitive>> for AbstractExpr {
        fn accept(self: &Self, v: &dyn Visitor<Box<Primitive>>) -> Box<Primitive> {
            match self {
                AbstractExpr::Binary(val) => v.visit_binary(val),
                AbstractExpr::Grouping(val) => v.visit_grouping(val),
                AbstractExpr::Literal(val) => v.visit_literal(val),
                AbstractExpr::Unary(val) => v.visit_unary(val),
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
}
