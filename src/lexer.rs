#[derive(Debug, Clone)]
pub enum TokenKind {
    Err,

    Whitespace,
    Newline,

    KwFunc,
    KwGlobal,
    KwWhile,
    KwIf,

    Ident,

    IntLit,
    FloatLit,
    BoolLit,
    StrLit,

    Equals,
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

plex::lexer! {
    fn next_token(text: 'a) -> TokenKind;

    r"[ \t]+" => TokenKind::Whitespace,
    r"[\n\r]" => TokenKind::Newline,

    r"func" => TokenKind::KwFunc,
    r"global" => TokenKind::KwGlobal,
    r"while" => TokenKind::KwWhile,
    r"if" => TokenKind::KwIf,

    r"[a-zA-Z_][a-zA-Z0-9_]*" => TokenKind::Ident,

    r"[+-]?[0-9]+" => TokenKind::IntLit,
    r"[+-]?[0-9]+\.[0-9]+" => TokenKind::FloatLit,
    r"True|False" => TokenKind::BoolLit,
    r#"["][^"]*["]"# => TokenKind::StrLit,

    r"=" => TokenKind::Equals,
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
pub struct Span {
    ll: usize,          // Low | Line
    hc: usize,          // High | Column
}

#[derive(Debug, Clone)]
pub struct TokenPos {
    chr: Span,
    ln: Span,
    col: Span,
}

#[derive(Debug, Clone)]
pub struct Token {
    val: String,
    kind: TokenKind,
    pos: TokenPos,
}

pub struct Lexer<'a> {
    code: &'a str,
    remaining: &'a str,
    result: Vec<Token>,
    errors: Vec<(String, Span)>,
    pos: Span,
}

impl<'a> Lexer<'a> {
    pub fn new(s: &'a str) -> Self {
        Self {
            code: s,
            remaining: s,
            result: vec![],
            errors: vec![],
            pos: Span { ll: 0usize, hc: 0usize },
        }
    }

    pub fn parse(&mut self) {
        while let Some(token) = self.next() {
            if token.is_ok() { self.result.push(token.unwrap()); }
            else {
                if let Some(mut last) = self.errors.last() {
                    let (last_msg, last_span) = last;
                    let (msg, mut span) = token.clone().unwrap_err();
                    if last_span.ll + 1 == span.ll {
                        span.hc += 1;
                        self.errors.insert(
                            self.errors.len() - 1,
                            (msg, span)
                        );
                        continue;
                    }
                }
                self.errors.push(token.unwrap_err());
            }
        }
    }

    fn next(&mut self) -> Option<Result<Token, (String, Span)>> {
        loop {
            let token = if let Some((kind, new_remaining)) = next_token(self.remaining) {
                let lo = self.code.len() - self.remaining.len();
                let hi = self.code.len() - new_remaining.len();
                let len = self.remaining.len() - new_remaining.len();
                self.remaining = new_remaining;
                Token {
                    val: self.code[lo..hi].to_string(),
                    kind,
                    pos: TokenPos {
                        chr: Span { ll: lo, hc: hi, },
                        ln: Span {
                            ll: self.pos.ll,
                            hc: self.pos.ll,
                        },
                        col: Span {
                            ll: self.pos.hc,
                            hc: self.pos.hc + len,
                        }
                    }
                }
            } else {
                return None;
            };
            match token.kind {
                TokenKind::Err => {
                    return Some(Err(("Unexpected token!".to_string(), token.pos.chr)));
                }
                TokenKind::Whitespace => { continue; }
                TokenKind::Newline => {
                    self.pos.ll += 1;
                    self.pos.hc = 1;
                    if let Some(last) = self.result.last() {
                        match last.kind {
                            TokenKind::Newline |
                            TokenKind::LBracket |
                            TokenKind::RBracket => {
                                continue;
                            }
                            _ => {}
                        }
                    } else { continue; }
                }
                _ => {  }
            }
            return Some(Ok(token));
        }
    }

    pub fn result(&self) -> Result<Vec<Token>, Vec<(String, Span)>> {
        if self.errors.is_empty() { Ok(self.result.clone()) }
        else { Err(self.errors.clone()) }
    }
}