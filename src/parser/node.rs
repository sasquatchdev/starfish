use crate::common::{flag::Flag, text::{location::Location, span::Span}};

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

    Meta
}

impl Node {
    pub fn new(span: Span, flag: Flag, lexeme: String, kind: NodeKind) -> Self {
        Self {
            span,
            flag,
            lexeme,
            kind,
        }
    }

    pub fn new_simple(span: Span, lexeme: String, kind: NodeKind) -> Self {
        Self::new(span, Flag::None, lexeme, kind)
    }

    pub fn new_meta(location: Location, flag: Flag) -> Self {
        Self::new(Span::new_single(location), flag, String::new(), NodeKind::Meta)
    }

    pub fn span(&self) -> &Span {
        &self.span
    }
    pub fn flag(&self) -> &Flag {
        &self.flag
    }
    pub fn lexeme(&self) -> &str {
        &self.lexeme
    }
    pub fn kind(&self) -> &NodeKind {
        &self.kind
    }
}
