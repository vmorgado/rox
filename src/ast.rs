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
        String(String),
        Number(f64),
        Comment(String),
    }

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
        pub fn print(self: &Self, expr: &dyn Expr) -> String {
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
            self.parenthesize(
                &exp.operator.lexme.clone().unwrap(),
                Vec::from([
                    Box::<&dyn Expr>::new(&*exp.left),
                    Box::<&dyn Expr>::new(&*exp.right),
                ]),
            )
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
            }
        }

        fn visit_unary(self: &Self, exp: &Unary) -> String {
            self.parenthesize(
                &exp.operator.lexme.clone().unwrap(),
                Vec::from([Box::<&dyn Expr>::new(&*exp.right)]),
            )
        }
    }

    pub struct Binary {
        pub operator: Token,
        pub left: Box<dyn Expr>,
        pub right: Box<dyn Expr>,
    }
    pub struct Grouping {
        pub expression: Box<dyn Expr>,
    }
    pub struct Literal {
        pub value: Box<Primitive>,
    }
    pub struct Unary {
        pub right: Box<dyn Expr>,
        pub operator: Token,
    }

    impl Expr for Binary {
        fn accept(self: &Self, v: &dyn Visitor) -> String {
            v.visit_binary(self)
        }
    }
    impl Expr for Grouping {
        fn accept(self: &Self, v: &dyn Visitor) -> String {
            v.visit_grouping(self)
        }
    }
    impl Expr for Literal {
        fn accept(self: &Self, v: &dyn Visitor) -> String {
            v.visit_literal(self)
        }
    }
    impl Expr for Unary {
        fn accept(self: &Self, v: &dyn Visitor) -> String {
            v.visit_unary(self)
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
