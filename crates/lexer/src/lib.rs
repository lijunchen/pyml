use std::fmt::{Debug, Write};

use logos::{Logos, Span};
use text_size::{TextRange, TextSize};

#[cfg(test)]
mod tests;

#[derive(Clone)]
pub struct Token<'a> {
    pub kind: TokenKind,
    pub text: &'a str,
    pub range: TextRange,
}

impl Debug for Token<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char('{')?;
        f.write_str("kind: ")?;
        f.write_str(&format!("{:?}", self.kind))?;
        f.write_str(", ")?;
        f.write_str("text: ")?;
        f.write_str(&format!("{:?}", &self.text))?;
        f.write_char('}')
    }
}

pub fn lex(input: &str) -> Vec<Token> {
    let lexer = Lexer::new(input);
    let toks: Vec<Token> = lexer.collect();
    toks
}

pub struct Lexer<'a> {
    inner: logos::Lexer<'a, TokenKind>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            inner: TokenKind::lexer(input),
        }
    }
}

fn range_from_span(span: Span) -> TextRange {
    let std::ops::Range { start, end } = span;
    let start = TextSize::try_from(start).unwrap();
    let end = TextSize::try_from(end).unwrap();
    TextRange::new(start, end)
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let kind = self.inner.next()?;
        if let Ok(kind) = kind {
            let text = self.inner.slice();
            Some(Self::Item {
                kind,
                text,
                range: range_from_span(self.inner.span()),
            })
        } else {
            let text = self.inner.slice();
            Some(Self::Item {
                kind: TokenKind::Error,
                text,
                range: range_from_span(self.inner.span()),
            })
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Logos)]
pub enum TokenKind {
    #[token("(")]
    LParen,

    #[token(")")]
    RParen,

    #[token("{")]
    LBrace,

    #[token("}")]
    RBrace,

    #[token("[")]
    LBracket,

    #[token("]")]
    RBracket,

    #[token("=")]
    Eq,

    #[token(";")]
    Semi,

    #[token(",")]
    Comma,

    #[token(":")]
    Colon,

    #[token("->")]
    Arrow,

    #[token("=>")]
    FatArrow,

    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token("*")]
    Star,

    #[token("/")]
    Slash,

    #[token(".")]
    Dot,

    #[token("fn")]
    FnKeyword,

    #[token("enum")]
    EnumKeyword,

    #[token("match")]
    MatchKeyword,

    #[token("if")]
    IfKeyword,

    #[token("else")]
    ElseKeyword,

    #[token("let")]
    LetKeyword,

    #[token("in")]
    InKeyword,

    #[token("return")]
    ReturnKeyword,

    #[token("true")]
    TrueKeyword,

    #[token("false")]
    FalseKeyword,

    #[token("_")]
    WildcardKeyword,

    #[token("Unit")]
    UnitKeyword,

    #[token("Bool")]
    BoolKeyword,

    #[token("Int")]
    IntKeyword,

    #[regex("[a-z][A-Za-z_0-9]*")]
    Lident,

    #[regex("[A-Z][A-Za-z_0-9]*")]
    Uident,

    #[regex("[0-9]+")]
    Int32,

    #[regex(r#""([^"\\\x00-\x1F]|\\(["\\bnfrt/]|u[a-fA-F0-9]{4}))*""#)]
    Str,

    #[regex("[ \n]+")]
    Whitespace,

    #[regex("//.*")]
    Comment,

    Error,
    Eof,
}

impl TokenKind {
    pub fn is_trivia(self) -> bool {
        matches!(self, Self::Whitespace | Self::Comment)
    }
}

impl std::fmt::Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::LParen => "(",
            Self::RParen => ")",
            Self::LBrace => "{",
            Self::RBrace => "}",
            Self::LBracket => "[",
            Self::RBracket => "]",
            Self::Eq => "=",
            Self::Semi => ";",
            Self::Comma => ",",
            Self::Colon => ":",
            Self::Arrow => "->",
            Self::FatArrow => "=>",
            Self::Plus => "+",
            Self::Minus => "-",
            Self::Star => "*",
            Self::Slash => "/",
            Self::Dot => ".",
            Self::EnumKeyword => "enum",
            Self::FnKeyword => "fn",
            Self::IfKeyword => "if",
            Self::MatchKeyword => "match",
            Self::ElseKeyword => "else",
            Self::LetKeyword => "let",
            Self::InKeyword => "in",
            Self::ReturnKeyword => "return",
            Self::TrueKeyword => "true",
            Self::FalseKeyword => "false",
            Self::WildcardKeyword => "_",
            Self::UnitKeyword => "Unit",
            Self::BoolKeyword => "Bool",
            Self::IntKeyword => "Int",
            Self::Lident => "lident",
            Self::Uident => "uident",
            Self::Int32 => "int",
            Self::Str => "str",
            Self::Whitespace => "whitespace",
            Self::Comment => "comment",
            Self::Error => "unknown token",
            Self::Eof => "end of file",
        })
    }
}
