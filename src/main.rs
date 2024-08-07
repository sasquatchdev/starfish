#![allow(unused)]

use common::text::Source;
use lexer::Lexer;
use parser::Parser;

mod common;
mod lexer;
mod parser;

fn main() {
    let text = "1 + 10 - 1".to_string(); // Should equal 10
    let file = "main.sf".to_string();

    let source = (file, text);
    let tokens = Lexer::new(source.clone()).lex();
    let nodes = Parser::new(source.clone(), tokens).parse();

    for node in nodes {
        println!("{:#?}", node.kind());
    }
}
