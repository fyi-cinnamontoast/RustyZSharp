use std::process::exit;
use crate::lexer::{Lexer, Span, Token};
use crate::parser::Parser;

pub struct Program<'a> {
    filename: &'a str,
}

impl<'a> Program<'a> {
    pub fn new(filename: &'a str) -> Self {
        Self {
            filename
        }
    }

    pub fn exec(&mut self, s: &'a str) {
        let ast = self.parse(s);
    }

    fn tokenize(&self, s: &'a str) -> Vec<Token> {
        let mut lexer = Lexer::new(s);
        lexer.parse();
        let Ok(tokens) = lexer.result() else {
            let errs = lexer.result().unwrap_err();
            let lines: Vec<&str> = s.split("\n").collect();
            println!("--> {}", self.filename);
            for error in errs {
                let line = lines.get(error.span.ll).unwrap_or_else(|| {
                    panic!("Line is out of bounds!");
                }).to_string();
                self.error(
                    format!("Unknown token \"{}\"", error.val),
                    error.span.ll, line,
                    Span { ll: error.span.hc, hc: error.span.hc + error.val.len() }
                );
            }
            exit(-1);
        };
        tokens
    }

    fn error(&self, msg: String, line_index: usize, line: String, span: Span) {
        println!(
            "{} at {}:{}-{}\n| {}\n| {}{}",
            msg,
            line_index + 1, span.ll + 1, span.hc + 1,
            line,
            " ".repeat(span.ll), "~".repeat(span.hc - span.ll)
        );
    }

    fn parse(&self, s: &'a str) {
        let tokens = self.tokenize(s);
        let mut parser = Parser::new(s, tokens);
        parser.parse();

        let Ok(ast) = parser.result() else {
            let errs = parser.result().unwrap_err();
            let lines: Vec<&str> = s.split("\n").collect();
            println!("--> {}", self.filename);
            for error in errs {
                let (ln, span) = error.pos;
                let line = lines.get(ln).unwrap_or_else(|| {
                    panic!("Line is out of bounds!");
                }).to_string();
                self.error(error.msg, ln, line, span);
            }
            exit(-1);
        };
        for expr in ast {
            println!("{:?}", expr);
        }
    }
}