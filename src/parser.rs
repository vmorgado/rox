pub mod parser {
    use crate::ast::ast::{Binary, Expr, Grouping, Literal, Primitive, Token, TokenType, Unary};

    struct Parser {
        tokens: Vec<Token>,
        current: usize,
    }

    impl Parser {
        pub fn new(tokens: Vec<Token>) -> Self {
            Parser {
                current: 0,
                tokens: tokens,
            }
        }
        pub fn previous(self: &Self) -> &Token {
            self.tokens.get(self.current - 1).unwrap()
        }

        pub fn do_match(self: &mut Self, token_types: Vec<TokenType>) -> bool {
            for token_type in token_types {
                if self.do_check(token_type) {
                    self.advance();
                    return true;
                }
            }
            false
        }

        pub fn advance(self: &mut Self) -> &Token {
            if !self.is_at_end() {
                self.current = self.current + 1;
            }
            self.previous()
        }

        pub fn is_at_end(self: &Self) -> bool {
            self.peek().token_type == TokenType::Eof
        }

        pub fn peek(self: &Self) -> &Token {
            self.tokens.get(self.current).unwrap()
        }

        pub fn do_check(self: &Self, token_type: TokenType) -> bool {
            if self.is_at_end() {
                return false;
            }

            self.peek().token_type == token_type
        }

        pub fn comparison(self: &mut Self) -> Box<dyn Expr> {
            let mut expr = self.term();

            while self.do_match(Vec::<TokenType>::from([
                TokenType::Greater,
                TokenType::GreaterEqual,
                TokenType::Less,
                TokenType::LessEqual,
            ])) {
                let operator = self.previous().clone();
                let right = self.term();
                expr = Box::new(Binary {
                    left: expr,
                    right: right,
                    operator: Box::new(operator),
                });
            }
            expr
        }

        pub fn term(self: &mut Self) -> Box<dyn Expr> {
            let mut expr = self.factor();
            while self.do_match(Vec::<TokenType>::from([TokenType::Minus, TokenType::Plus])) {
                let operator = self.previous().clone();
                let right = self.factor();
                expr = Box::new(Binary {
                    left: expr,
                    right: right,
                    operator: Box::new(operator),
                });
            }
            expr
        }

        pub fn factor(self: &mut Self) -> Box<dyn Expr> {
            let mut expr = self.unary();
            while self.do_match(Vec::<TokenType>::from([TokenType::Slash, TokenType::Star])) {
                let operator = self.previous().clone();
                let right = self.unary();
                expr = Box::new(Binary {
                    left: expr,
                    right: right,
                    operator: Box::new(operator),
                });
            }
            expr
        }

        pub fn unary(self: &mut Self) -> Box<dyn Expr> {
            if self.do_match(Vec::<TokenType>::from([TokenType::Bang, TokenType::Minus])) {
                let operator = self.previous().clone();
                let right = self.unary();
                return Box::new(Unary {
                    right: right,
                    operator: Box::new(operator),
                });
            }
            self.primary()
        }

        pub fn primary(self: &mut Self) -> Box<dyn Expr> {
            if self.do_match(Vec::<TokenType>::from([TokenType::False])) {
                return Box::new(Literal {
                    value: Box::new(Primitive::Boolean(false)),
                });
            }
            if self.do_match(Vec::<TokenType>::from([TokenType::True])) {
                return Box::new(Literal {
                    value: Box::new(Primitive::Boolean(true)),
                });
            }
            if self.do_match(Vec::<TokenType>::from([TokenType::Nil])) {
                return Box::new(Literal {
                    value: Box::new(Primitive::Nil),
                });
            }
            if self.do_match(Vec::<TokenType>::from([
                TokenType::Number,
                TokenType::String,
            ])) {
                return Box::new(Literal {
                    value: Box::new(self.previous().literal.as_ref().unwrap().clone()),
                });
            }

            if self.do_match(Vec::<TokenType>::from([TokenType::LeftParen])) {
                let mut expr = self.expression();
                // self.consume(TokenType::RightParen, "Expected ')' after expression.");
                return Box::new(Grouping { expression: expr });
            }

            //TODO: this should output error
            Box::new(Literal {
                value: Box::new(Primitive::Nil),
            })
        }

        pub fn expression(self: &mut Self) -> Box<dyn Expr> {
            self.equality()
        }

        pub fn equality(self: &mut Self) -> Box<dyn Expr> {
            let mut expr = self.comparison();

            while self.do_match(Vec::<TokenType>::from([
                TokenType::BangEqual,
                TokenType::EqualEqual,
            ])) {
                let operator = self.previous().clone();
                let right = self.comparison();
                expr = Box::new(Binary {
                    left: expr,
                    right: right,
                    operator: Box::new(operator),
                });
            }
            expr
        }
    }
}
