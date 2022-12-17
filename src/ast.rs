#[allow(dead_code, unused_variables)]
pub mod ast {

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

    #[derive(Debug)]
    pub enum AbstractExpr {
        Binary(Binary),
        Grouping(Grouping),
        Literal(Literal),
        Unary(Unary),
    }

    pub trait Expr {
        fn accept(&self, v: &dyn Visitor) -> String;
    }
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

    pub trait Visitor {
        // fn new() -> Self;
        fn visit_binary(&self, b: &Binary) -> String;
        fn visit_grouping(&self, g: &Grouping) -> String;
        fn visit_literal(&self, b: &Literal) -> String;
        fn visit_unary(&self, b: &Unary) -> String;
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

    #[derive(Debug)]
    pub struct Binary {
        pub operator: Box<Token>,
        pub left: Box<AbstractExpr>,
        pub right: Box<AbstractExpr>,
    }

    #[derive(Debug)]
    pub struct Grouping {
        pub expression: Box<AbstractExpr>,
    }

    #[derive(Debug)]
    pub struct Literal {
        pub value: Box<Primitive>,
    }

    #[derive(Debug)]
    pub struct Unary {
        pub right: Box<AbstractExpr>,
        pub operator: Box<Token>,
    }

    impl Expr for AbstractExpr {
        fn accept(self: &Self, v: &dyn Visitor) -> String {
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
