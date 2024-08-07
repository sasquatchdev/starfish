use crate::{common::flag::Flag, lexer::token::TokenKind};

use super::{node::{Node, NodeKind}, Parser};

impl Parser {
    pub fn parse(mut self) -> Vec<Node> {
        let mut nodes = Vec::new();

        while let Some(_) = self.peek() {
            let node = self.parse_expr();
            if let Some(node) = node {
                nodes.push(node);
            }

            self.advance();
        }

        nodes
    }

    pub fn parse_expr(&mut self) -> Option<Node> {
        self.parse_primary()
    }

    pub fn parse_primary(&mut self) -> Option<Node> {
        let current = self.peek()?;
        Some(match current.kind() {
            TokenKind::Floating(value) => Node::new_simple(self.current(), current.lexeme(), NodeKind::Floating(value)),
            TokenKind::Unsigned(value) => Node::new_simple(self.current(), current.lexeme(), NodeKind::Unsigned(value)),
            TokenKind::Signed(value) => Node::new_simple(self.current(), current.lexeme(), NodeKind::Signed(value)),
            _ => Node::new_meta(self.current(), Flag::Error(String::from("Unexpected Token")))
        })
    }
}
