use crate::ast::AST;
use crate::lexer::{Span, Token};

pub struct ParserError<'a> {
    pub origin: &'a Parser<'a>,
    pub msg: String,
    pub token: Token,
    pub span: Span,
}

pub struct Parser<'a> {
    code: &'a str,
    result: AST,
    errors: Vec<ParserError<'a>>,
    tokens: Vec<Token>,
    pos: usize,
}