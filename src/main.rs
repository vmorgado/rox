#![allow(dead_code, unused_variables, unused_imports, unused_assignments)]
mod ast;
mod environment;
mod interpreter;
mod parser;
mod printer;
mod scanner;
mod utils;
mod visitor;
extern crate clap;
use crate::interpreter::Interpreter;
use crate::parser::Parser;
use crate::printer::Printer;
use crate::scanner::{Scanner, TokenScanner};
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
    let mut parser = Parser::new(tokens);
    let statements = parser.parse();
    // let printer = Printer::new();
    // // can print result of printer to get ast printed
    // printer.print(expression.clone());
    let itp = Interpreter::new();
    itp.interpret(statements.clone());
}

fn read_file(file_path: &str) -> String {
    println!("Starting executing : {}", file_path);

    let result = match fs::read_to_string(file_path) {
        Ok(content) => content,
        _error => panic!("File could not be read"),
    };

    result
}

#[cfg(test)]
mod tests {
    use crate::ast::{AbstractExpr, Binary, Literal, Primitive, Token, TokenType};
    use crate::printer::Printer;
    #[test]
    fn print_ast() {
        let expression = Box::new(AbstractExpr::Binary(Binary {
            operator: Box::new(Token {
                token_type: TokenType::Plus,
                lexme: Some("+".to_string()),
                literal: None,
                line: 1,
            }),
            left: Box::new(AbstractExpr::Literal(Literal {
                value: Box::new(Primitive::Number(2.)),
            })),
            right: Box::new(AbstractExpr::Literal(Literal {
                value: Box::new(Primitive::Number(2.)),
            })),
        }));

        let mut printer = Printer::new();
        let result = printer.print(expression);
        assert_eq!(result, "(+ 2 2)".to_string());
    }
}
