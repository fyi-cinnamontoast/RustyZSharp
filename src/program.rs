use std::collections::HashMap;
use std::fmt::format;
use std::process::exit;
use crate::ast::{Expr, ExprKind};
use crate::environment::{Env, Member, Val};
use crate::lexer::{Lexer, Token, TokenKind};
use crate::parser::Parser;
use crate::util::Span;

pub struct Program<'a> {
    filename: &'a str,
    pub(crate) env: Env
}

impl<'a> Program<'a> {
    pub fn new(filename: &'a str) -> Self {
        Self {
            filename,
            env: vec![ HashMap::new() ]
        }
    }

    pub fn exec(&mut self, s: &'a str) {
        let mut lexer = Lexer::new(s);
        lexer.parse();
        let Ok(tokens) = lexer.result() else {
            panic!();
        };
        for token in tokens.clone() {
            print!("{} ", token.kind);
            match token.kind {
                TokenKind::Newline => { println!() },
                _ => {}
            }
        }
        println!();

        let mut parser = Parser::new(s, tokens);
        parser.parse();
        let Ok(ast) = parser.result() else {
            panic!();
        };
        for expr in ast {
            println!("{:?}", expr);
        }
    }
}