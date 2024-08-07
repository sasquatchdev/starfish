use crate::common::{flag::Flag, text::span::Span};

use super::{token::{Token, TokenKind}, Lexer};

macro_rules! single {
    ($self:expr, $char:expr, $kind:expr) => {
        Some(Token::new(Span::new_single($self.position()), $char.to_string(), $kind, crate::common::flag::Flag::None))
    };
}

macro_rules! meta {
    ($self:expr, $flag:expr) => {
        Some(Token::new_meta(Span::new_single($self.position()), $flag))
    };
}

impl Lexer {
    pub fn lex(mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        
        while let Some(_) = self.peek() {
            let token = self.lex_token();
            if let Some(token) = token {
                tokens.push(token);
            } 

            self.advance();
        }

        tokens
    }

    pub fn lex_token(&mut self) -> Option<Token> {
        let start = self.position();
        let single = Span::new_single(start);
        match self.peek().unwrap() {
            ' ' | '\t' | '\n' => None,
            '+' => single!(self, '+', TokenKind::Plus),
            '-' => single!(self, '-', TokenKind::Minus),
            '*' => single!(self, '*', TokenKind::Star),
            '/' => single!(self, '/', TokenKind::Slash),
            '0'..='9' => self.lex_unsigned(),
            _ => meta!(self, Flag::Error(String::from("Unexpected Token")))
        }
    }

    pub fn lex_unsigned(&mut self) -> Option<Token> {
        let start = self.position();
        let mut lexeme = String::new();

        while let Some(c) = self.peek() {
            if c.is_numeric() || c == '.' {
                lexeme.push(c);
                self.advance();
            } else {
                break;
            }
        }

        let dots = lexeme.matches('.').count();
        if dots > 1 {
            return meta!(self, Flag::Error(String::from("Invalid Number: too many dots")));
        }

        if dots == 1 {
            let float = lexeme.parse::<f64>();
            if let Ok(float) = float {
                return Some(Token::new(Span::new(start, self.position()), lexeme, TokenKind::Floating(float), Flag::None));
            } else {
                return meta!(self, Flag::Error(String::from("Invalid Number: failed to parse (float)")));
            }
        }

        let unsigned = lexeme.parse::<u64>();
        if let Ok(unsigned) = unsigned {
            return Some(Token::new(Span::new(start, self.position()), lexeme, TokenKind::Unsigned(unsigned), Flag::None));
        } else {
            return meta!(self, Flag::Error(String::from("Invalid Number: failed to parse (u-int)")));
        }
    }
}
