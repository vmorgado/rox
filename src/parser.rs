#![allow(dead_code, unused_variables, unused_imports, unused_assignments)]
use crate::ast::{
    AbstractExpr, AbstractStmt, Assign, Binary, Block, Grouping, If, Literal, Logical, Primitive,
    Print, Statement, Token, TokenType, Unary, Var, Variable, Visitable, While,
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
            statements.push(*self.declaration())
        }

        statements
    }

    pub fn block(&mut self) -> Vec<Box<AbstractStmt>> {
        let mut statements: Vec<Box<AbstractStmt>> = Vec::new();

        while !self.do_check(TokenType::RightBrace) && !self.is_at_end() {
            statements.push(Box::new(*self.declaration()));
        }

        self.consume(TokenType::RightBrace, "Expected '}' after block.");
        statements
    }

    pub fn declaration(&mut self) -> Box<AbstractStmt> {
        if self.do_match(Vec::from([TokenType::Var])) {
            return self.var_declaration();
        }

        return Box::new(self.statement());
        // TODO : handle properly error here
        // self.synchronize();
        // None
    }

    // assignment
    pub fn assignment(&mut self) -> Box<AbstractExpr> {
        let expr = self.exp_or();

        if self.do_match(Vec::<TokenType>::from([TokenType::Equal])) {
            let equals = self.previous();
            let value = self.assignment();

            match *expr {
                AbstractExpr::Variable(var) => {
                    let name = var.name;
                    return Box::new(AbstractExpr::Assign(Assign { name, value }));
                }
                _ => {
                    panic!("Invalid assignment Target.");
                }
            }
        }

        return expr;
    }

    pub fn var_declaration(&mut self) -> Box<AbstractStmt> {
        let name = self
            .consume(TokenType::Identifier, "Expect variable name.")
            .clone();

        let mut initializer: Option<AbstractExpr> = None;

        if self.do_match(Vec::from([TokenType::Equal])) {
            initializer = Some(*self.expression());
        }

        self.consume(TokenType::SemiColon, "Expected ';' after variable.");
        Box::new(AbstractStmt::Var(Var {
            name: Box::new(name),
            initializer,
        }))
    }

    pub fn statement(&mut self) -> AbstractStmt {
        if self.do_match(Vec::<TokenType>::from([TokenType::Print])) {
            return self.print_stmt();
        }

        if self.do_match(Vec::<TokenType>::from([TokenType::While])) {
            return self.while_stmt();
        }

        if self.do_match(Vec::<TokenType>::from([TokenType::LeftBrace])) {
            return AbstractStmt::Block(Block {
                stmts: self.block(),
            });
        }

        if self.do_match(Vec::<TokenType>::from([TokenType::For])) {
            return self.for_stmt();
        }

        if self.do_match(Vec::<TokenType>::from([TokenType::If])) {
            return self.if_stmt();
        }

        self.expr_stmt()
    }

    pub fn for_stmt(&mut self) -> AbstractStmt {
        self.consume(TokenType::LeftParen, "Expected '(' after 'for'.");

        let mut initializer = None;

        if self.do_match(Vec::<TokenType>::from([TokenType::SemiColon])) {
            initializer = None;
        } else if self.do_match(Vec::<TokenType>::from([TokenType::Var])) {
            initializer = Some(self.var_declaration());
        } else {
            initializer = Some(Box::new(self.expr_stmt()));
        }

        let mut condition = None;

        if !self.do_check(TokenType::SemiColon) {
            condition = Some(*self.expression());
        }

        self.consume(TokenType::SemiColon, "Expected ';' after loop condition.");

        let mut increment = None;
        if !self.do_check(TokenType::RightParen) {
            increment = Some(*self.expression());
        }

        self.consume(TokenType::RightParen, "Expected ')' after for clauses.");

        let mut body = self.statement();

        if increment != None {
            body = AbstractStmt::Block(Block {
                stmts: Vec::from([
                    Box::new(body),
                    Box::new(AbstractStmt::Statement(Statement {
                        expression: Box::new(increment.unwrap()),
                    })),
                ]),
            })
        }

        if condition == None {
            condition = Some(AbstractExpr::Literal(Literal {
                value: Box::new(Primitive::Boolean(true)),
            }))
        }

        body = AbstractStmt::While(While {
            condition: Box::new(condition.unwrap()),
            body: Box::new(body),
        });

        match initializer {
            Some(init) => {
                return AbstractStmt::Block(Block {
                    stmts: Vec::<Box<AbstractStmt>>::from([init, Box::new(body)]),
                })
            }
            None => {}
        }

        return body;
    }

    pub fn if_stmt(&mut self) -> AbstractStmt {
        self.consume(TokenType::LeftParen, "Expected '(' after 'if'.");
        let condition = self.expression();
        self.consume(TokenType::RightParen, "Expected ')' after condition.");
        let then_branch = self.statement();
        let mut else_branch = None;
        if self.do_match(Vec::from([TokenType::Else])) {
            else_branch = Some(Box::new(self.statement()));
        }

        AbstractStmt::If(If {
            condition,
            then_branch: Box::new(then_branch),
            else_branch,
        })
    }

    pub fn while_stmt(&mut self) -> AbstractStmt {
        self.consume(TokenType::LeftParen, "Expects '(' after 'while'.");
        let condition = Box::new(*self.expression().clone());
        self.consume(TokenType::RightParen, "Expects ')' after condition.");
        let body = Box::new(self.statement());

        AbstractStmt::While(While { condition, body })
    }

    pub fn print_stmt(&mut self) -> AbstractStmt {
        let value = *self.expression();

        self.consume(TokenType::SemiColon, "Expected ';' after value.");

        AbstractStmt::Print(Print {
            expression: Box::new(value),
        })
    }

    pub fn expr_stmt(&mut self) -> AbstractStmt {
        let value = *self.expression();
        self.consume(TokenType::SemiColon, "Expected ';' after expression.");

        AbstractStmt::Statement(Statement {
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

    pub fn exp_and(&mut self) -> Box<AbstractExpr> {
        let mut expr = self.equality();

        while self.do_match(Vec::<TokenType>::from([TokenType::And])) {
            let operator = self.previous().clone();
            let right = self.equality();
            expr = Box::new(AbstractExpr::Logical(Logical {
                left: expr,
                right,
                operator: Box::new(operator),
            }));
        }

        expr
    }

    pub fn exp_or(&mut self) -> Box<AbstractExpr> {
        let mut expr = self.exp_and();

        while self.do_match(Vec::<TokenType>::from([TokenType::Or])) {
            let operator = self.previous().clone();
            let right = self.exp_and();
            expr = Box::new(AbstractExpr::Logical(Logical {
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

        if self.do_match(Vec::<TokenType>::from([TokenType::Identifier])) {
            return Box::new(AbstractExpr::Variable(Variable {
                name: Box::new(self.previous().clone()),
            }));
        }

        self.error(self.peek(), "Expected Expression.");
        panic!("");
    }

    pub fn error(&self, token: &Token, message: &str) {
        panic!("{} Instead found {:?}", message, token);
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
        self.assignment()
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
