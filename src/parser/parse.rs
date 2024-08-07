use crate::{common::{flag::Flag, text::span::Span}, lexer::token::TokenKind};

use super::{node::{Node, NodeKind}, Parser};

impl Parser {
    pub fn parse(mut self) -> Vec<Node> {
        let mut nodes = Vec::new();

        while let Some(_) = self.peek() {
            let node = self.parse_expr();
            if let Some(node) = node {
                nodes.push(node);
            }
        }

        nodes
    }

    pub fn parse_expr(&mut self) -> Option<Node> {
        self.parse_binary()
    }

    pub fn parse_binary(&mut self) -> Option<Node> {
        let mut left = self.parse_primary()?;
        
        while (
            self.peek().is_some() && (
            self.peek()?.kind() == TokenKind::Plus  ||
            self.peek()?.kind() == TokenKind::Minus ||
            self.peek()?.kind() == TokenKind::Star  ||
            self.peek()?.kind() == TokenKind::Slash
        )) {
            let operator = self.pop()?;
            let right = self.parse_primary()?;

            let span = Span::new(left.span().start(), right.span().end());

            left = Node::new_simple(span, NodeKind::BinaryExpr {
                left: Box::new(left),
                right: Box::new(right.clone()),
                operator: operator.lexeme(),
            });
        }

        Some(left)
    }

    pub fn parse_primary(&mut self) -> Option<Node> {
        let current = self.peek()?;
        let node = Some(match current.kind() {
            TokenKind::Floating(value) => Node::new_simple(self.current(), NodeKind::Floating(value)),
            TokenKind::Unsigned(value) => Node::new_simple(self.current(), NodeKind::Unsigned(value)),
            TokenKind::Signed(value) => Node::new_simple(self.current(), NodeKind::Signed(value)),
            _ => Node::new_meta(self.current(), Flag::Error(String::from("Unexpected Token")))
        });

        self.advance();
        node
    }
}
