use crate::common::{flag::Flag, text::span::Span};

#[derive(Debug, Clone)]
pub struct Node {
    span: Span,
    flag: Flag,
    lexeme: String,
    kind: NodeKind,
}

#[derive(Debug, Clone)]
pub enum NodeKind {
}
