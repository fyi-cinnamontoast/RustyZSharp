use std::io::stdout;
use std::ops::{Range};
use std::slice::SliceIndex;
use crate::ast::{Expr, ExprKind};
use crate::lexer::{Token, TokenKind};
use crate::util::{Span};

#[derive(Clone)]
pub struct ParserError {
    pub msg: String,
    pub pos: Span,
}

pub struct Parser<'a> {
    code: &'a str,
    result: Vec<Expr>,
    errors: Vec<ParserError>,
    tokens: Vec<Token>,
    pos: usize,

    span_stack: Vec<Span>
}

impl<'a> Parser<'a> {
    pub fn new(code: &'a str, tokens: Vec<Token>) -> Self {
        Self {
            code,
            result: vec![],
            errors: vec![],
            tokens,
            pos: 0usize,
            span_stack: vec![]
        }
    }

    pub fn parse(&mut self) {
        if self.result.len() > 0 || self.errors.len() > 0 { return; }

        while self.pos < self.tokens.len() {
            let Some(def) = self.parse_define() else { continue; };
            self.result.push(def);
        }
    }

    fn parse_define(&mut self) -> Option<Expr> {
        let Some(tok) =
            self.require(&[ TokenKind::Ident, TokenKind::KwGlobal, TokenKind::KwFunc ])
            else { todo!() };
        match tok.kind {
            TokenKind::Ident => { self.parse_var() }
            TokenKind::KwGlobal => {
                self.begin();
                self.next();
                let Some(var) = self.parse_var() else { todo!() };
                Some(Expr {
                    kind: ExprKind::Global(Box::new(var)),
                    span: self.end()
                })
            }
            TokenKind::KwFunc => { self.parse_func() }
            _ => unreachable!()
        }
    }

    fn parse_block(&mut self) -> Option<Expr> {
        self.begin();
        self.require_and_next(&[ TokenKind::LBracket ]);
        todo!();
    }

    fn parse_func(&mut self) -> Option<Expr> {
        self.begin();
        if self.require_and_next(&[ TokenKind::KwFunc ]).is_none() {
            todo!()
        }
        let Some(name) = self.parse_name() else { todo!() };
        todo!()
    }

    fn parse_var(&mut self) -> Option<Expr> {
        self.begin();
        let Some(r#type) =
            self.parse_ident()
            else { todo!() };
        let Some(name) =
            self.parse_name()
            else { todo!() };
        self.require_and_next(&[ TokenKind::EqualSign ]);
        let Some(val) =
            self.parse_expr()
            else { todo!() };
        self.require_and_next(&[ TokenKind::Newline ]);
        Some(Expr {
            kind: ExprKind::VarDef(Box::new(name), Box::new(r#type), Box::new(val)),
            span: self.end()
        })
    }

    fn parse_expr(&mut self) -> Option<Expr> {
        let Some(tok) =
            self.matches(&[
                TokenKind::IntLit, TokenKind::FloatLit, TokenKind::BoolLit, TokenKind::StrLit
            ])
            else { todo!() };
        match tok.kind {
            TokenKind::IntLit |
            TokenKind::FloatLit |
            TokenKind::BoolLit |
            TokenKind::StrLit => self.parse_atom(),
            _ => unreachable!()
        }
    }

    fn parse_atom(&mut self) -> Option<Expr> {
        self.begin();
        let Some(atom) =
            self.require_and_next(&[
                TokenKind::IntLit, TokenKind::FloatLit, TokenKind::BoolLit, TokenKind::StrLit
            ])
            else { todo!() };
        match atom.kind {
            TokenKind::IntLit => {
                if let Ok(v) = atom.val.parse() {
                    Some(Expr {
                        kind: ExprKind::IntLit(v),
                        span: self.end()
                    })
                }
                else { todo!() }
            }
            TokenKind::FloatLit => {
                if let Ok(v) = atom.val.parse() {
                    Some(Expr {
                        kind: ExprKind::FloatLit(v),
                        span: self.end()
                    })
                }
                else { todo!() }
            }
            TokenKind::BoolLit => {
                Some(Expr {
                    kind: ExprKind::BoolLit(
                        match atom.val.as_str() {
                            "True" => true,
                            "False" => false,
                            _ => unreachable!()
                        }
                    ),
                    span: self.end()
                })
            }
            TokenKind::StrLit => Some(Expr {
                kind: ExprKind::StrLit(atom.val[1..atom.val.len()-1].to_string()),
                span: self.end()
            }),
            _ => unreachable!()
        }
    }

    fn parse_name(&mut self) -> Option<Expr> {
        self.begin();
        let Some(first) =
            self.require_and_next(&[ TokenKind::Ident ])
            else { todo!() };
        let mut res = vec![ first.val ];
        while self.matches(&[ TokenKind::Dot ]).is_some() {
            self.next();
            let Some(ident) =
                self.require_and_next(&[ TokenKind::Ident ])
                else {todo!()};
            res.push(ident.val);
        }
        Some(Expr {
            kind: ExprKind::Name(res.join(".")),
            span: self.end()
        })
    }

    fn parse_ident(&mut self) -> Option<Expr> {
        self.begin();
        let Some(ident) =
            self.require_and_next(&[ TokenKind::Ident ])
            else { todo!() };
        Some(Expr {
            kind: ExprKind::Name(ident.val),
            span: self.end()
        })
    }

    fn get(&self, index: usize) -> Option<Token> {
        match self.tokens.get(index) {
            Some(token) => Some(token.clone()),
            None => None
        }
    }

    fn current(&mut self) -> Option<Token> {
        match self.get(self.pos) {
            Some(token) => Some(token),
            None => { None }
        }
    }

    fn next(&mut self) -> Option<Token> {
        match self.current() {
            Some(token) => {
                self.pos += 1;
                if let Some(last) = self.span_stack.last_mut() {
                    last.hi = token.span.hi;
                }
                Some(token)
            },
            None => None
        }
    }

    #[inline(always)]
    fn rollback(&mut self, offset: usize) { self.pos -= offset; }
    #[inline(always)]
    fn rollback1(&mut self) { self.rollback(1); }

    fn matches(&mut self, expected: &'a [TokenKind]) -> Option<Token> {
        let Some(token) = self.current() else { return None; };
        for expect in expected {
            if token.kind == *expect {
                return Some(token);
            }
        }
        return None;
    }

    fn skip_until(&mut self, expect: &'a [ TokenKind ]) -> Option<Token> {
        while self.matches(expect).is_none() && self.current().is_some() {
            self.next();
        }
        return self.current();
    }

    fn skip_while(&mut self, expect: &'a [ TokenKind ]) -> Option<Token> {
        while self.matches(expect).is_some() {
            self.next();
        }
        return self.current();
    }

    fn require(&mut self, expected: &'a [TokenKind]) -> Option<Token> {
        match self.matches(expected) {
            Some(token) => Some(token),
            None => {
                let current = if let Some(tok) = self.current() { tok.kind.to_string() } else { "EOF".to_string() };
                self.error(
                    format!(
                        "Expected `{}`, found `{}`",
                        expected.iter().map(|kind| format!("{}", kind)).collect::<Vec<String>>().join("`, `"),
                        current,
                    ),
                    self.pos..self.pos
                );
                None
            }
        }
    }

    fn require_or(&mut self, expected: &'a [TokenKind], cb: fn()) {
        if self.require(expected).is_none() {
            cb()
        }
    }

    fn require_and_next(&mut self, expected: &'a [TokenKind]) -> Option<Token> {
        if let Some(token) = self.require(expected) {
            self.next();
            Some(token)
        }
        else { None }
    }

    fn error(&mut self, msg: String, span: Range<usize>) {
        let low = self.get(span.start).unwrap();
        let high = self.get(span.end).unwrap();
        self.errors.push(ParserError {
            msg,
            pos: low.span | high.span
        });
    }

    fn begin(&mut self) {
        let Some(current) = self.current() else { return; };
        self.span_stack.push(current.span.clone())
    }

    fn end(&mut self) -> Span {
        let Some(res) = self.span_stack.pop() else { panic!() };
        if let Some(last) = self.span_stack.last_mut() {
            last.hi = res.hi;
        }
        res
    }

    pub fn result(&self) -> Result<Vec<Expr>, Vec<ParserError>> {
        if self.errors.is_empty() { Ok(self.result.clone()) }
        else { Err(self.errors.clone()) }
    }
}