use crate::common::{flag::Flag, text::span::Span};

#[derive(Debug, Clone)]
pub struct Token {
    span: Span,
    lexeme: String,
    kind: TokenKind,
    flag: Flag,
}

#[derive(Debug, Clone)]
pub enum TokenKind {
    Floating(f64),
    Unsigned(u64),
    Signed(i64),

    Plus, Minus, Slash, Star,
    
    Meta
}

impl Token {
    pub fn new(span: Span, lexeme: String, kind: TokenKind, flag: Flag) -> Self {
        Token { span, lexeme, kind, flag }
    }
    
    pub fn new_meta(span: Span, flag: Flag) -> Self {
        Token { span, flag, lexeme: String::new(), kind: TokenKind::Meta }
    }

    pub fn span(&self) -> Span {
        self.span.clone()
    }

    pub fn lexeme(&self) -> String {
        self.lexeme.clone()
    }

    pub fn kind(&self) -> TokenKind {
        self.kind.clone()
    }

    pub fn flag(&self) -> Flag {
        self.flag.clone()
    }
}
