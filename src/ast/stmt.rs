use crate::{ast::Expr, utils::Loc};

#[derive(Debug)]
pub enum Stmt {
    Expr(ExprStmt),
    Return(ReturnStmt),
    Block(BlockStmt),
    Empty(EmptyStmt),
    If(IfStmt),
    For(ForStmt),
    While(WhileStmt),
}

#[derive(Debug)]
pub struct ForStmt {
    pub loc: Loc,
    pub init: Option<Expr>,
    pub test: Option<Expr>,
    pub update: Option<Expr>,
    pub body: Box<Stmt>,
}

#[derive(Debug)]
pub struct WhileStmt {
    pub loc: Loc,
    pub test: Expr,
    pub body: Box<Stmt>,
}

#[derive(Debug)]
pub struct IfStmt {
    pub loc: Loc,
    pub test: Expr,
    pub consequent: Box<Stmt>,
    pub alternate: Option<Box<Stmt>>,
}

#[derive(Debug)]
pub struct EmptyStmt {
    pub loc: Loc,
}

#[derive(Debug)]
pub struct BlockStmt {
    pub loc: Loc,
    pub body: Vec<Stmt>,
}

#[derive(Debug)]
pub struct ExprStmt {
    pub loc: Loc,
    pub expr: Expr,
}

#[derive(Debug)]
pub struct ReturnStmt {
    pub loc: Loc,
    pub argument: Option<Expr>,
}

#[derive(Debug)]
pub struct Program {
    pub loc: Loc,
    pub body: Vec<Stmt>,
    pub stack_size: usize,
}
