use crate::common::text::span::Span;

#[derive(Debug, Clone)]
pub struct Token {
    span: Span,
    lexeme: String,
    kind: TokenKind,
    flag: TokenFlag,
}

#[derive(Debug, Clone)]
pub enum TokenKind {
    Floating(f64),
    Unsigned(u64),
    Signed(i64),

    Plus, Minus, Slash, Star,
    
    Meta
}

#[derive(Debug, Clone)]
pub enum TokenFlag {
    None,
    Info(String),
    Warn(String),
    Error(String),
    Fatal(String),
}

impl Token {
    pub fn new(span: Span, lexeme: String, kind: TokenKind, flag: TokenFlag) -> Self {
        Token { span, lexeme, kind, flag }
    }
    
    pub fn new_meta(span: Span, flag: TokenFlag) -> Self {
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

    pub fn flag(&self) -> TokenFlag {
        self.flag.clone()
    }
}
