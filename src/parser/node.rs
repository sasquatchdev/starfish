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
    Floating(f64),
    Unsigned(u64),
    Signed(i64),

    BinaryExpr {
        left: Box<Node>,
        right: Box<Node>,
        operator: String,
    },
}
