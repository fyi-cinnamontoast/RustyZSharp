use crate::util::Span;

#[derive(Debug, Clone)]
pub struct Expr {
    pub kind: ExprKind,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum ExprKind {
    Global(Box<Expr>),

    Block(Vec<Expr>),

    FuncDef(Box<Expr>, Vec<Expr>, Box<Expr>),
    VarDef(Box<Expr>, Box<Expr>, Box<Expr>),

    Name(String),

    IntLit(i64),
    FloatLit(f64),
    BoolLit(bool),
    StrLit(String),

    Call(Box<Expr>, Vec<Expr>),
}
