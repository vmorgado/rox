pub mod parser {
    use crate::ast::ast::{
        AbstractExpr, Binary, Expr, Grouping, Literal, Primitive, Token, TokenType, Unary,
    };

    pub struct Parser {
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
        pub fn parse(self: &mut Self) -> Box<AbstractExpr> {
            self.expression()
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

        pub fn comparison(self: &mut Self) -> Box<AbstractExpr> {
            let mut expr = self.term();

            while self.do_match(Vec::<TokenType>::from([
                TokenType::Greater,
                TokenType::GreaterEqual,
                TokenType::Less,
                TokenType::LessEqual,
            ])) {
                let operator = self.previous().clone();
                let right = self.term();
                expr = Box::new(AbstractExpr::Binary(Binary {
                    left: expr,
                    right: right,
                    operator: Box::new(operator),
                }));
            }
            expr
        }

        pub fn term(self: &mut Self) -> Box<AbstractExpr> {
            let mut expr = self.factor();
            while self.do_match(Vec::<TokenType>::from([TokenType::Minus, TokenType::Plus])) {
                let operator = self.previous().clone();
                let right = self.factor();
                expr = Box::new(AbstractExpr::Binary(Binary {
                    left: expr,
                    right: right,
                    operator: Box::new(operator),
                }));
            }
            expr
        }

        pub fn factor(self: &mut Self) -> Box<AbstractExpr> {
            let mut expr = self.unary();
            while self.do_match(Vec::<TokenType>::from([TokenType::Slash, TokenType::Star])) {
                let operator = self.previous().clone();
                let right = self.unary();
                expr = Box::new(AbstractExpr::Binary(Binary {
                    left: expr,
                    right: right,
                    operator: Box::new(operator),
                }));
            }
            expr
        }

        pub fn unary(self: &mut Self) -> Box<AbstractExpr> {
            if self.do_match(Vec::<TokenType>::from([TokenType::Bang, TokenType::Minus])) {
                let operator = self.previous().clone();
                let right = self.unary();
                return Box::new(AbstractExpr::Unary(Unary {
                    right: right,
                    operator: Box::new(operator),
                }));
            }
            self.primary()
        }

        pub fn primary(self: &mut Self) -> Box<AbstractExpr> {
            if self.do_match(Vec::<TokenType>::from([TokenType::False])) {
                return Box::new(AbstractExpr::Literal(Literal {
                    value: Box::new(Primitive::Boolean(false)),
                }));
            }
            if self.do_match(Vec::<TokenType>::from([TokenType::True])) {
                return Box::new(AbstractExpr::Literal(Literal {
                    value: Box::new(Primitive::Boolean(true)),
                }));
            }
            if self.do_match(Vec::<TokenType>::from([TokenType::Nil])) {
                return Box::new(AbstractExpr::Literal(Literal {
                    value: Box::new(Primitive::Nil),
                }));
            }
            if self.do_match(Vec::<TokenType>::from([
                TokenType::Number,
                TokenType::String,
            ])) {
                return Box::new(AbstractExpr::Literal(Literal {
                    value: Box::new(self.previous().literal.as_ref().unwrap().clone()),
                }));
            }

            if self.do_match(Vec::<TokenType>::from([TokenType::LeftParen])) {
                let mut expr = self.expression();
                self.consume(TokenType::RightParen, "Expected ')' after expression.");
                return Box::new(AbstractExpr::Grouping(Grouping { expression: expr }));
            }

            self.error(self.peek(), "Expected Expression.");
            panic!("");
        }
        pub fn error(self: &Self, token: &Token, message: &str) {
            panic!("{}", message);
        }

        pub fn synchronize(self: &mut Self) {
            self.advance();
            while !self.is_at_end() {
                if self.previous().token_type == TokenType::SemiColon {
                    return;
                }

                match self.peek().token_type {
                    TokenType::Return => return,
                    _ => {}
                }

                self.advance();
            }
        }

        pub fn consume(self: &mut Self, token_type: TokenType, message: &str) -> &Token {
            if self.do_check(token_type) {
                return self.advance();
            }

            self.error(self.peek(), message);
            // TODO: Handle errors better;
            panic!("");
        }

        pub fn expression(self: &mut Self) -> Box<AbstractExpr> {
            self.equality()
        }

        pub fn equality(self: &mut Self) -> Box<AbstractExpr> {
            let mut expr = self.comparison();

            while self.do_match(Vec::<TokenType>::from([
                TokenType::BangEqual,
                TokenType::EqualEqual,
            ])) {
                let operator = self.previous().clone();

                let right = self.comparison();
                expr = Box::new(AbstractExpr::Binary(Binary {
                    left: expr,
                    right: right,
                    operator: Box::new(operator),
                }));
            }
            expr
        }
    }
}
