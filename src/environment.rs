use std::collections::HashMap;
use crate::ast::Expr;

#[derive(Debug, Clone)]
pub enum Val {
    None,
    String(String),
    Integer(i64),
    Float(f64),
    Bool(bool)
}

#[derive(Debug, Clone)]
pub enum Member {
    Var(String, Val),
    Func(String, Vec<String>, Expr)
}

pub type Scope = HashMap<String, Member>;
pub type Env = Vec<Scope>;