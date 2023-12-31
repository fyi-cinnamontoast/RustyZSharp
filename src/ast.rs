use crate::lexer::Span;

pub type Block = Vec<Expr>;

#[derive(Debug, Clone)]
pub struct Expr {
    pub kind: ExprKind,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum ExprKind {
    Global(Box<Expr>),

    FuncDef(Vec<Expr>, Vec<String>, Block),
    VarDef(Box<Expr>, String, Box<Expr>),

    Name(String),

    IntLit(i64),
    FloatLit(f64),
    StrLit(String),
}
