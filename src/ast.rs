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
    pub enum Expr {
        Binary {
            operator: Token,
            left: Box<Expr>,
            right: Box<Expr>,
        },

        Grouping {
            expression: Box<Expr>,
        },

        Literal {
            expression: Box<Expr>,
        },

        Unary {
            right: Box<Expr>,
            operator: Token,
        },
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