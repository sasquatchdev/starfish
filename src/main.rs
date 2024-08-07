#![allow(unused)]

use common::text::Source;
use lexer::Lexer;

mod common;
mod lexer;
mod parser;

fn main() {
    let text = "1 + 1 - 2.3 * 4 / 5".to_string(); // Should equal 0.16
    let file = "main.sf".to_string();

    let source = (file, text);
    let tokens = Lexer::new(source).lex();

    for token in tokens {
        println!("{:?}", token.kind());
    }
}
