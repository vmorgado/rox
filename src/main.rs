#[warn(dead_code, unused_variables, unused_assignments)]
#[macro_use]

mod utils;
mod ast;
mod scanner;
extern crate clap;

use crate::ast::ast::{Binary, Grouping, Literal, Primitive, Printer, Token, TokenType, Unary};
use crate::scanner::scanner::{Scanner, TokenScanner};
use clap::{App, ArgMatches, SubCommand};
use std::fs;

fn build_clap_matches() -> ArgMatches {
    App::new("rox")
        .version("0.0.1-alpha")
        .author("Vitor Morgado <vitor.morgado@gmx.de>")
        .about(" awesome things")
        .args_from_usage("[FILE] 'Entrypoint file to run'")
        .subcommand(
            SubCommand::with_name("test")
                .about("controls testing features")
                .arg_from_usage("-v --verbose 'Print test information verbosely'"),
        )
        .get_matches()
}
fn main() {
    let matches = build_clap_matches();
    match matches.value_of("FILE") {
        Some(file_path) => run(&read_file(file_path)),
        None => repl(),
    };
}

fn repl() {
    println!("No file was specified, starting REPL...");
}

fn run(statement: &str) {
    let mut scanner: TokenScanner = Scanner::new(statement);
    let tokens = scanner.scan_tokens();
    println!("{:?}", tokens);

    let expression = Binary {
        operator: Token {
            token_type: TokenType::Star,
            lexme: Some("*".to_string()),
            literal: None,
            line: 1,
        },
        left: Box::new(Unary {
            operator: Token {
                token_type: TokenType::Minus,
                lexme: Some("-".to_string()),
                literal: None,
                line: 1,
            },
            right: Box::new(Literal {
                value: Box::new(Primitive::Number(123.00)),
            }),
        }),
        right: Box::new(Grouping {
            expression: Box::new(Literal {
                value: Box::new(Primitive::Number(45.67)),
            }),
        }),
    };

    let printer = Printer::new();
    println!("{}", printer.print(&expression));
}

fn read_file(file_path: &str) -> String {
    println!("Starting executing : {}", file_path);

    let result = match fs::read_to_string(file_path) {
        Ok(content) => content,
        _error => panic!("File could not be read"),
    };

    result
}
