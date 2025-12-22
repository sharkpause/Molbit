mod token;
mod lexer;
mod parser;

use crate::token::print_token;
use crate::lexer::Lexer;
use crate::parser::Parser;

fn main() {
    let mut lexer = Lexer {
        input: String::from("return 67;\nreturn 0;"),
        index: 0
    };

    let tokens = lexer.tokenize().unwrap();

    println!("Tokens:");

    for token in &tokens {
        print_token(token);
    }

    let mut parser = Parser {
        tokens: tokens,
        index: 0
    };

    let statements = parser.parse_program().unwrap();
    println!("\nStatements:\n{:?}", statements);
}