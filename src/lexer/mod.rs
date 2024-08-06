use crate::common::text::{location::Location, Source};

pub mod token;
pub mod lex;

pub struct Lexer {
    source: Source,
    position: Location,
}

impl Lexer {
    pub fn new(source: Source) -> Self {
        Lexer {
            source: source.clone(),
            position: Location::new(source, 0),
        }
    }

    pub fn source(&self) -> Source {
        self.source.clone()
    }

    pub fn position(&self) -> Location {
        self.position.clone()
    }

    pub fn chars(&self) -> Vec<char> {
        self.source.1.chars().collect()
    }

    pub fn peek(&self) -> Option<char> {
        self.chars().get(self.position.index()).copied()
    }

    pub fn advance(&mut self) -> bool {
        if self.position.is_eof() { return false; }
        self.position.advance();
        true
    }

    pub fn pop(&mut self) -> Option<char> {
        let current = self.peek();
        self.advance();
        current
    }
}
