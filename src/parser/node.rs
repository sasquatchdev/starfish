use crate::common::{flag::Flag, text::{location::Location, span::Span}};

#[derive(Debug, Clone)]
pub struct Node {
    span: Span,
    flag: Flag,
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

    Meta
}

impl Node {
    pub fn new(span: Span, flag: Flag, kind: NodeKind) -> Self {
        Self {
            span,
            flag,
            kind,
        }
    }

    pub fn new_simple(span: Span, kind: NodeKind) -> Self {
        Self::new(span, Flag::None, kind)
    }

    pub fn new_meta(span: Span, flag: Flag) -> Self {
        Self::new(span, flag, NodeKind::Meta)
    }

    pub fn span(&self) -> &Span {
        &self.span
    }
    pub fn flag(&self) -> &Flag {
        &self.flag
    }
    pub fn kind(&self) -> &NodeKind {
        &self.kind
    }
}
