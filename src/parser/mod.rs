use crate::{common::text::{location::Location, span::Span, Source}, lexer::{token::Token, Lexer}};

pub mod node;
pub mod parse;

pub struct Parser {
    source: Source,
    tokens: Vec<Token>,
    current: Span,
}

impl Parser {
    pub fn new(source: Source, tokens: Vec<Token>) -> Self {
        Self { source: source.clone(), tokens, current: Span::new_single(Location::new_start(source)) }
    }

    pub fn source(&self) -> Source {
        self.source.clone()
    }

    pub fn tokens(&self) -> Vec<Token> {
        self.tokens.clone()
    }

    pub fn current(&self) -> Span {
        self.current.clone()
    }

    pub fn peek(&self) -> Option<Token> {
        self.tokens.get(self.current.start().index()).cloned()
    }

    pub fn advance(&mut self) -> bool {
        if self.current.end().is_eof() { return false; }
        self.current = Span::new_single(self.current.end().clone());
        true
    }

    pub fn pop(&mut self) -> Option<Token> {
        let current = self.peek();
        self.advance();
        current
    }
}
