#![allow(unused)]

use common::text::Source;
use lexer::Lexer;
use parser::Parser;

mod common;
mod lexer;
mod parser;

fn main() {
    let text = "1 + 1 - 2.3 * 4 / 5".to_string(); // Should equal 0.16
    let file = "main.sf".to_string();

    let source = (file, text);
    let tokens = Lexer::new(source.clone()).lex();
    let nodes = Parser::new(source.clone(), tokens).parse();

    for node in nodes {
        println!("{:?}", node);
    }
}
