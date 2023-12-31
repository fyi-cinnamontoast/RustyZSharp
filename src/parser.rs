use std::ops::{Add, Range};
use std::slice::SliceIndex;
use crate::ast::{Block, Expr, ExprKind};
use crate::lexer::{LexerError, Span, Token, TokenKind};

#[derive(Clone)]
pub struct ParserError {
    pub msg: String,
    pub pos: (usize, Span), // Line, Column(High, Low)
}

pub struct Parser<'a> {
    code: &'a str,
    result: Block,
    errors: Vec<ParserError>,
    tokens: Vec<Token>,
    pos: usize,
}

impl<'a> Parser<'a> {
    pub fn new(code: &'a str, tokens: Vec<Token>) -> Self {
        Self {
            code,
            result: vec![],
            errors: vec![],
            tokens,
            pos: 0usize
        }
    }

    pub fn parse(&mut self) {
        if self.result.len() > 0 || self.errors.len() > 0 { return; }

        while self.pos < self.tokens.len() - 1 {
            let Some(def) = self.parse_define() else { continue; };
            self.result.push(def);
        }
    }

    pub fn parse_define(&mut self) -> Option<Expr> {
        let is_var: bool;
        if !self.matches(&[ TokenKind::KwFunc, TokenKind::Ident ]) {
            let mut found = self.current().val;
            let start = self.pos;
            self.next();
            while !self.matches(&[ TokenKind::Ident ]) {
                let tok = self.next();
                found += tok.val.as_str();
            }
            let end = self.pos - 1;
            while !self.matches(&[ TokenKind::Newline ]) {
                self.next();
            }
            let msg = format!(r#"Expected "func", "global" or an identifier found "{}""#, found);
            self.error(msg, start..end);
            return None;
        } else {
            let tok = self.current();
            is_var = match tok.kind {
                TokenKind::KwFunc => false,
                TokenKind::Ident => true,
                _ => unreachable!()
            };
        }
        if is_var {
            let var_type = self.next();
            let name = self.parse_name();
            self.require(&[ TokenKind::Equals ]);
            let val = self.parse_term();
            self.require(&[ TokenKind::Newline ]);
            Some(Expr {
                kind: ExprKind::VarDef(Box::new(name), var_type.val, Box::new(val.clone())),
                span: Span{ ll: var_type.pos.chr.ll, hc: val.span.hc }
            })
        }
        else {
            let name = self.parse_complex_name();
            todo!();
        }
    }

    pub fn parse_var_def() {

    }

    pub fn parse_term(&mut self) -> Expr {
        if self.matches(&[
            TokenKind::IntLit, TokenKind::FloatLit, TokenKind::StrLit
        ]) {
            self.parse_atom()
        }
        else {
            todo!()
        }
    }

    pub fn parse_atom(&mut self) -> Expr {
        let token = self.next();
        match token.kind {
            TokenKind::IntLit => Expr {
                kind: ExprKind::IntLit(token.val.parse().unwrap()),
                span: token.pos.chr
            },
            TokenKind::FloatLit => Expr {
                kind: ExprKind::FloatLit(token.val.parse().unwrap()),
                span: token.pos.chr
            },
            TokenKind::StrLit => Expr {
                kind: ExprKind::StrLit(token.val[1..token.val.len()-1].to_string()),
                span: token.pos.chr
            },
            _ => unreachable!()
        }
    }

    pub fn parse_name(&mut self) -> Expr {
        let Some(name) = self.match_current(&[ TokenKind::Ident ]) else {
            todo!()
        };
        self.next();
        Expr {
            kind: ExprKind::Name(name.val.clone()),
            span: name.pos.chr.clone()
        }
    }

    pub fn parse_complex_name(&mut self) -> Vec<Expr> {
        let mut name: Vec<Expr> = vec![];
        let first = self.parse_name();
        name.push(first);
        while self.matches(&[ TokenKind::Dot ]) {
            let next = self.parse_name();
            name.push(next);
            self.next();
        }
        self.rollback1();
        name
    }

    pub fn get(&self, index: usize) -> Token {
        self.tokens.get(index).unwrap().clone()
    }

    pub fn current(&self) -> Token {
        self.get(self.pos)
    }

    pub fn next(&mut self) -> Token {
        let token = self.current();
        self.pos += 1;
        return token;
    }

    #[inline(always)]
    pub fn rollback(&mut self, offset: usize) { self.pos -= offset; }
    #[inline(always)]
    pub fn rollback1(&mut self) { self.rollback(1); }

    pub fn matches(&self, expected: &'a [TokenKind]) -> bool {
        let token = self.current();
        for expect in expected {
            if token.kind == *expect {
                return true;
            }
        }
        return false;
    }

    pub fn match_current(&self, expected: &'a [TokenKind]) -> Option<Token> {
        if self.matches(expected) { Some(self.current()) }
        else { None }
    }

    pub fn require(&mut self, expected: &'a [TokenKind]) {
        if !self.matches(expected) {
            let current = self.current();
            self.error(
                format!(
                    "Unexpected token `{}`, required `{}`",
                    current.val,
                    expected.iter().map(|kind| format!("{}", kind)).collect::<Vec<String>>().join("`, `")
                ),
                self.pos..self.pos
            );
        }
        else { self.next(); }
    }

    pub fn error(&mut self, msg: String, span: Range<usize>) {
        let low = self.get(span.start);
        let high = self.get(span.end);
        self.errors.push(ParserError {
            msg,
            pos: (low.pos.ln.hc, Span{ ll: low.pos.col.ll, hc: high.pos.col.hc })
        });
    }

    pub fn result(&self) -> Result<Block, Vec<ParserError>> {
        if self.errors.is_empty() { Ok(self.result.clone()) }
        else { Err(self.errors.clone()) }
    }
}