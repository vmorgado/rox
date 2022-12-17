#[allow(dead_code)]
pub mod scanner {

    use crate::ast::ast::{Primitive, Token, TokenType};
    use std::collections::HashMap;

    pub struct TokenScanner {
        source: String,
        tokens: Vec<Token>,
        start: usize,
        current: usize,
        line: usize,
        keywords: HashMap<&'static str, TokenType>,
    }

    pub trait Scanner {
        fn new(source: &str) -> Self;
        fn scan_tokens(&mut self) -> Vec<Token>;
        fn scan_token(&mut self);
        fn is_at_end(&self) -> bool;
        fn add_token(&mut self, token_type: TokenType, literal: Option<Primitive>);
        fn advance(&mut self) -> Option<char>;
        fn char_match(&mut self, to_match: char) -> bool;
        fn peek(&mut self) -> Option<char>;
        fn peek_next(&self) -> Option<char>;
        fn init_string(&mut self) -> Option<String>;
        fn init_number(&mut self) -> Option<f64>;
        fn init_identifier(&mut self) -> Option<(TokenType, String)>;
    }

    impl Scanner for TokenScanner {
        fn new(source: &str) -> TokenScanner {
            TokenScanner {
                source: source.to_string(),
                tokens: Vec::<Token>::new(),
                start: 0,
                current: 0,
                line: 1,
                keywords: HashMap::<&str, TokenType>::from([
                    ("and", TokenType::And),
                    ("class", TokenType::Class),
                    ("else", TokenType::Else),
                    ("false", TokenType::False),
                    ("for", TokenType::For),
                    ("fun", TokenType::Fun),
                    ("if", TokenType::If),
                    ("nil", TokenType::Nil),
                    ("or", TokenType::Or),
                    ("print", TokenType::Print),
                    ("return", TokenType::Return),
                    ("super", TokenType::Super),
                    ("this", TokenType::This),
                    ("true", TokenType::True),
                    ("var", TokenType::Var),
                    ("while", TokenType::While),
                ]),
            }
        }

        fn is_at_end(self: &Self) -> bool {
            self.current >= self.source.len()
        }

        fn add_token(self: &mut Self, token_type: TokenType, literal: Option<Primitive>) {
            let text = &self.source[self.start..self.current];
            match literal {
                Some(literal) => {
                    self.tokens.push(Token {
                        token_type: token_type,
                        lexme: Some(text.to_string()),
                        literal: Some(literal),
                        line: self.line,
                    });
                }
                None => {
                    self.tokens.push(Token {
                        token_type: token_type,
                        lexme: Some(text.to_string()),
                        literal: None,
                        line: self.line,
                    });
                }
            };
        }

        fn advance(self: &mut Self) -> Option<char> {
            self.current = self.current + 1;
            self.source.chars().nth(self.current)
        }

        fn char_match(self: &mut Self, expected: char) -> bool {
            if self.is_at_end() {
                return false;
            }
            let next_char = self.source.chars().nth(self.current).unwrap();
            if next_char != expected {
                return false;
            }
            self.current = self.current + 1;
            true
        }

        fn peek(self: &mut Self) -> Option<char> {
            if self.is_at_end() {
                return Some('\0');
            }

            self.source.chars().nth(self.current)
        }

        fn peek_next(self: &Self) -> Option<char> {
            if self.current + 1 >= self.source.len() {
                return Some('\0');
            }

            return self.source.chars().nth(self.current + 1);
        }

        fn init_string(self: &mut Self) -> Option<String> {
            while self.peek().unwrap() != '"' && !self.is_at_end() {
                if self.peek().unwrap() == '\n' {
                    self.line = self.line + 1;
                }
                self.advance();
            }

            if self.is_at_end() {
                panic!("{} : Unterminated String", self.line);
            }
            self.advance();
            let value = &self.source[self.start + 1..self.current - 1];
            Some(value.to_string())
        }

        fn init_number(self: &mut Self) -> Option<f64> {
            while self.peek().unwrap().is_numeric() {
                self.advance();
            }

            if self.peek().unwrap() == '.' && self.peek_next().unwrap().is_numeric() {
                while self.peek().unwrap().is_numeric() {
                    self.advance();
                }
            }

            let value = &self.source[self.start..self.current].to_string();
            println!("{:?}", value);
            Some(value.parse::<f64>().unwrap())
        }

        fn init_identifier(self: &mut Self) -> Option<(TokenType, String)> {
            while self.peek().unwrap().is_alphanumeric() {
                self.advance();
            }

            let value = &self.source[self.start..self.current];

            let token_type = match self.keywords.get(value) {
                Some(val) => *val,
                None => TokenType::Identifier,
            };
            Some((token_type, value.to_string()))
        }

        fn scan_token(self: &mut Self) {
            let token = self.peek().unwrap_or('\0');
            self.advance();
            match token {
                '(' => self.add_token(TokenType::LeftParen, None),
                ')' => self.add_token(TokenType::RightParen, None),
                '{' => self.add_token(TokenType::LeftBrace, None),
                '}' => self.add_token(TokenType::RightBrace, None),
                ',' => self.add_token(TokenType::Comma, None),
                '.' => self.add_token(TokenType::Dot, None),
                '-' => self.add_token(TokenType::Minus, None),
                '+' => self.add_token(TokenType::Plus, None),
                ';' => self.add_token(TokenType::SemiColon, None),
                '*' => self.add_token(TokenType::Star, None),
                '!' => {
                    let token_type = match self.char_match('=') {
                        true => TokenType::BangEqual,
                        false => TokenType::Bang,
                    };
                    self.add_token(token_type, None);
                }
                '=' => {
                    let token_type = match self.char_match('=') {
                        true => TokenType::EqualEqual,
                        false => TokenType::Equal,
                    };
                    self.add_token(token_type, None);
                }
                '<' => {
                    let token_type = match self.char_match('=') {
                        true => TokenType::LessEqual,
                        false => TokenType::Less,
                    };
                    self.add_token(token_type, None);
                }
                '>' => {
                    let token_type = match self.char_match('=') {
                        true => TokenType::GreaterEqual,
                        false => TokenType::Greater,
                    };
                    self.add_token(token_type, None);
                }
                '/' => {
                    let token_type = match self.char_match('/') {
                        true => {
                            while self.peek().unwrap() != '\n' && !self.is_at_end() {
                                self.advance();
                            }
                            TokenType::Comment
                        }
                        false => TokenType::Slash,
                    };
                    self.add_token(token_type, None);
                }
                ' ' => {}
                '\r' => {}
                '\t' => {}
                '\n' => {
                    self.line = self.line + 1;
                }
                '"' => {
                    let value = self.init_string();
                    match value {
                        Some(val) => {
                            self.add_token(TokenType::String, Some(Primitive::String(val)))
                        }
                        None => {}
                    }
                }
                'o' => {
                    if self.char_match('r') {
                        self.add_token(TokenType::Or, None);
                    }
                }
                '\u{0}' => {}

                c => {
                    if c.is_numeric() {
                        let value = self.init_number();
                        match value {
                            Some(val) => {
                                self.add_token(TokenType::Number, Some(Primitive::Number(val)))
                            }
                            None => {}
                        }
                    } else if c.is_alphabetic() || c == '_' {
                        let value = self.init_identifier();
                        match value {
                            Some((token_type, val)) => {
                                self.add_token(token_type, Some(Primitive::String(val)))
                            }
                            None => {}
                        }
                    } else {
                        panic!("Error in line {}: Unexpected Character {:?} ", self.line, c);
                    }
                }
            };
        }

        fn scan_tokens(self: &mut Self) -> Vec<Token> {
            while !self.is_at_end() {
                self.start = self.current;
                self.scan_token();
            }

            self.tokens.push(Token {
                token_type: TokenType::Eof,
                lexme: Some("".to_string()),
                literal: None,
                line: self.line,
            });

            self.tokens.clone()
        }
    }
}
