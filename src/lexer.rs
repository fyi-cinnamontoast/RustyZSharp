use std::cmp::{max, min};
use std::fmt;
use std::fmt::{Display, Formatter};
use crate::util::{Span};

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Err,

    Whitespace,
    Newline,

    KwFunc,
    KwGlobal,
    KwWhile,
    KwIf,
    KwReturn,

    Ident,

    IntLit,
    FloatLit,
    BoolLit,
    StrLit,

    EqualSign,
    AddEquals,
    SubEquals,
    MulEquals,
    DivEquals,

    Plus,
    Minus,
    Star,
    Slash,

    Dot,
    Comma,

    LParen,
    RParen,
    LBracket,
    RBracket,
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            TokenKind::Whitespace => "whitespace",
            TokenKind::Newline => "newline",
            TokenKind::KwFunc => "func",
            TokenKind::KwGlobal => "global",
            TokenKind::KwWhile => "while",
            TokenKind::KwIf => "if",
            TokenKind::Ident => "identifier",
            TokenKind::IntLit => "int literal",
            TokenKind::FloatLit => "float literal",
            TokenKind::BoolLit => "bool literal",
            TokenKind::StrLit => "string literal",
            TokenKind::EqualSign => "=",
            TokenKind::AddEquals => "+=",
            TokenKind::SubEquals => "-=",
            TokenKind::MulEquals => "*=",
            TokenKind::DivEquals => "/=",
            TokenKind::Plus => "+",
            TokenKind::Minus => "-",
            TokenKind::Star => "*",
            TokenKind::Slash => "/",
            TokenKind::Dot => ".",
            TokenKind::Comma => ",",
            TokenKind::LParen => "(",
            TokenKind::RParen => ")",
            TokenKind::LBracket => "{",
            TokenKind::RBracket => "}",
            _ => unreachable!(),
        })
    }
}

plex::lexer! {
    fn next_token(_text: 'a) -> TokenKind;

    r"[ \t]+" => TokenKind::Whitespace,
    r"(\r\n)|([\n\r])" => TokenKind::Newline,

    r"[a-zA-Z_][a-zA-Z0-9_]*" => TokenKind::Ident,

    r"[0-9]+" => TokenKind::IntLit,
    r"[0-9]+\.[0-9]+" => TokenKind::FloatLit,
    r"True|False" => TokenKind::BoolLit,
    r#"["][^"]*["]"# => TokenKind::StrLit,

    r"=" => TokenKind::EqualSign,
    r"\+=" => TokenKind::AddEquals,
    r"-=" => TokenKind::SubEquals,
    r"\*=" => TokenKind::MulEquals,
    r"/=" => TokenKind::DivEquals,

    r"\+" => TokenKind::Plus,
    r"-" => TokenKind::Minus,
    r"\*" => TokenKind::Star,
    r"/" => TokenKind::Slash,

    r"\." => TokenKind::Dot,
    r"," => TokenKind::Comma,

    r"\(" => TokenKind::LParen,
    r"\)" => TokenKind::RParen,
    r"{" => TokenKind::LBracket,
    r"}" => TokenKind::RBracket,

    r"." => TokenKind::Err
}

#[derive(Debug, Clone)]
pub struct Token {
    pub val: String,
    pub kind: TokenKind,
    pub span: Span,
}

#[derive(Clone)]
pub struct LexerError {
    pub val: String,
    pub span: Span
}

pub struct Lexer<'a> {
    code: &'a str,
    remaining: &'a str,
    result: Vec<Token>,
    errors: Vec<LexerError>,
    pos: Span,
    chr: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(s: &'a str) -> Self {
        Self {
            code: s,
            remaining: s,
            result: vec![],
            errors: vec![],
            pos: Span { lo: 0usize, hi: 0usize },
            chr: 0usize,
        }
    }

    pub fn parse(&mut self) {
        if self.result.len() > 0 || self.errors.len() > 0 { return; }
        while self.next() { }
    }

    fn next(&mut self) -> bool {
        loop {
            if let Some((kind, new_remaining)) = next_token(self.remaining) {
                let len = self.remaining.len() - new_remaining.len();
                let mut val = self.remaining[..len].to_string();
                let mut pos = self.pos.clone();
                self.pos.hi += len;
                self.chr += len;
                self.remaining = new_remaining;
                let kind = match kind.clone() {
                    TokenKind::Whitespace => { continue },
                    TokenKind::Newline => {
                        self.pos.lo += 1;
                        self.pos.hi = 0;
                        if let Some(last) = self.result.last() {
                            match last.kind {
                                TokenKind::Newline |
                                TokenKind::LBracket |
                                TokenKind::RBracket |
                                TokenKind::LParen |
                                TokenKind::RParen |
                                TokenKind::EqualSign |
                                TokenKind::AddEquals |
                                TokenKind::SubEquals |
                                TokenKind::MulEquals |
                                TokenKind::DivEquals => continue,
                                _ => { kind }
                            }
                        } else { continue }
                    },
                    TokenKind::Ident => {
                        match val.as_str() {
                            "global" => TokenKind::KwGlobal,
                            "func" => TokenKind::KwFunc,
                            "if" => TokenKind::KwIf,
                            "while" => TokenKind::KwWhile,
                            "return" => TokenKind::KwReturn,
                            _ => kind
                        }
                    },
                    TokenKind::Err => {
                        if let Some(last) = self.errors.last().clone() {
                            if last.span.hi + last.val.len() == pos.hi {
                                val = format!("{}{}", last.val, val);
                                pos.hi = last.span.hi;
                                self.errors.remove(self.errors.len() - 1);
                            }
                        }
                        let err = LexerError {
                            val,
                            span: pos
                        };
                        self.errors.push(err.clone());
                        continue;
                    }
                    _ => { kind },
                };
                self.result.push(Token {
                    val,
                    kind,
                    span: Span { lo: self.chr - len, hi: self.chr, },
                });
                return true;
            } else {
                return false;
            };
        }
    }

    pub fn result(&self) -> Result<Vec<Token>, Vec<LexerError>> {
        if self.errors.is_empty() { Ok(self.result.clone()) }
        else { Err(self.errors.clone()) }
    }
}