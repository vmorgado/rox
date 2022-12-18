#![allow(dead_code, unused_variables, unused_imports, unused_assignments)]
use crate::ast::{
    AbstractExpr, AbstractStmt, Binary, Expr, Expression, Grouping, Literal, Primitive, Print,
    Token, TokenType, Unary,
};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { current: 0, tokens }
    }
    pub fn parse(&mut self) -> Vec<AbstractStmt> {
        let mut statements = Vec::<AbstractStmt>::new();
        while !self.is_at_end() {
            statements.push(self.statement())
        }

        statements
    }
    pub fn statement(&mut self) -> AbstractStmt {
        if self.do_match(Vec::<TokenType>::from([TokenType::Print])) {
            return self.print_stmt();
        }

        self.expr_stmt()
    }

    pub fn print_stmt(&mut self) -> AbstractStmt {
        let value = *self.expression();
        self.consume(TokenType::SemiColon, "Expected ; after value.");

        AbstractStmt::Print(Print {
            expression: Box::new(value),
        })
    }

    pub fn expr_stmt(&mut self) -> AbstractStmt {
        let value = *self.expression();
        self.consume(TokenType::SemiColon, "Expected ; after expression.");

        AbstractStmt::Expression(Expression {
            expression: Box::new(value),
        })
    }

    pub fn previous(&mut self) -> &Token {
        self.tokens.get(self.current - 1).unwrap()
    }

    pub fn do_match(&mut self, token_types: Vec<TokenType>) -> bool {
        for token_type in token_types {
            if self.do_check(token_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    pub fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    pub fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    pub fn peek(&self) -> &Token {
        self.tokens.get(self.current).unwrap()
    }

    pub fn do_check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        self.peek().token_type == token_type
    }

    pub fn comparison(&mut self) -> Box<AbstractExpr> {
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
                right,
                operator: Box::new(operator),
            }));
        }
        expr
    }

    pub fn term(&mut self) -> Box<AbstractExpr> {
        let mut expr = self.factor();
        while self.do_match(Vec::<TokenType>::from([TokenType::Minus, TokenType::Plus])) {
            let operator = self.previous().clone();
            let right = self.factor();
            expr = Box::new(AbstractExpr::Binary(Binary {
                left: expr,
                right,
                operator: Box::new(operator),
            }));
        }
        expr
    }

    pub fn factor(&mut self) -> Box<AbstractExpr> {
        let mut expr = self.unary();
        while self.do_match(Vec::<TokenType>::from([TokenType::Slash, TokenType::Star])) {
            let operator = self.previous().clone();
            let right = self.unary();
            expr = Box::new(AbstractExpr::Binary(Binary {
                left: expr,
                right,
                operator: Box::new(operator),
            }));
        }
        expr
    }

    pub fn unary(&mut self) -> Box<AbstractExpr> {
        if self.do_match(Vec::<TokenType>::from([TokenType::Bang, TokenType::Minus])) {
            let operator = self.previous().clone();
            let right = self.unary();
            return Box::new(AbstractExpr::Unary(Unary {
                right,
                operator: Box::new(operator),
            }));
        }
        self.primary()
    }

    pub fn primary(&mut self) -> Box<AbstractExpr> {
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
            let expr = self.expression();
            self.consume(TokenType::RightParen, "Expected ')' after expression.");
            return Box::new(AbstractExpr::Grouping(Grouping { expression: expr }));
        }

        self.error(self.peek(), "Expected Expression.");
        panic!("");
    }
    pub fn error(&self, token: &Token, message: &str) {
        panic!("{} {:?}", message, token);
    }

    pub fn synchronize(&mut self) {
        self.advance();
        while !self.is_at_end() {
            if self.previous().token_type == TokenType::SemiColon {
                return;
            }

            if self.peek().token_type == TokenType::Return {
                return;
            }

            self.advance();
        }
    }

    pub fn consume(&mut self, token_type: TokenType, message: &str) -> &Token {
        if self.do_check(token_type) {
            return self.advance();
        }

        self.error(self.peek(), message);
        // TODO: Handle errors better;
        panic!("");
    }

    pub fn expression(&mut self) -> Box<AbstractExpr> {
        self.equality()
    }

    pub fn equality(&mut self) -> Box<AbstractExpr> {
        let mut expr = self.comparison();

        while self.do_match(Vec::<TokenType>::from([
            TokenType::BangEqual,
            TokenType::EqualEqual,
        ])) {
            let operator = self.previous().clone();

            let right = self.comparison();
            expr = Box::new(AbstractExpr::Binary(Binary {
                left: expr,
                right,
                operator: Box::new(operator),
            }));
        }
        expr
    }
}
