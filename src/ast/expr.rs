use crate::{object::Offset, token::TokenType, utils::Loc};

use super::Lit;

#[derive(Debug)]
pub enum Expr {
    Binary(BinaryExpr),
    Literal(Lit),
    Unary(UnaryExpr),
    Assign(AssignExpr),
    Ident(IdentExpr),
    Deref(DerefExpr),
    Addr(AddrExpr),
}

impl Expr {
    pub fn is_addr(&self) -> bool {
        matches!(self, Expr::Addr(_))
    }

    pub fn as_binary(&self) -> Option<&BinaryExpr> {
        match self {
            Expr::Binary(expr) => Some(expr),
            _ => None,
        }
    }

    // pub fn as_lit(&self) -> Option<&Lit> {
    //     match self {
    //         Expr::Literal(lit) => Some(lit),
    //         _ => None,
    //     }
    // }

    // pub fn as_ident(&self) -> Option<&IdentExpr> {
    //     match self {
    //         Expr::Ident(expr) => Some(expr),
    //         _ => None,
    //     }
    // }

    // pub fn as_addr(&self) -> Option<&AddrExpr> {
    //     match self {
    //         Expr::Addr(expr) => Some(expr),
    //         _ => None,
    //     }
    // }

    // pub fn as_deref(&self) -> Option<&DerefExpr> {
    //     match self {
    //         Expr::Deref(expr) => Some(expr),
    //         _ => None,
    //     }
    // }

    pub fn loc(&self) -> Loc {
        match self {
            Expr::Binary(expr) => expr.loc.clone(),
            Expr::Literal(expr) => expr.loc(),
            Expr::Unary(expr) => expr.loc.clone(),
            Expr::Assign(expr) => expr.loc.clone(),
            Expr::Ident(expr) => expr.loc.clone(),
            Expr::Deref(expr) => expr.loc.clone(),
            Expr::Addr(expr) => expr.loc.clone(),
        }
    }
}

#[derive(Debug)]
pub struct IdentExpr {
    pub loc: Loc,
    pub name: String,
    pub offset: Offset,
}

#[derive(Debug)]
pub enum LeftVal {
    Ident(IdentExpr),
    Deref(DerefExpr),
}

#[derive(Debug)]
pub struct AssignExpr {
    pub loc: Loc,
    // TODO: left_val
    pub left: Box<LeftVal>,
    pub right: Box<Expr>,
}

#[derive(Debug)]
pub struct BinaryExpr {
    pub loc: Loc,
    pub left: Box<Expr>,
    pub op: BinaryOp,
    pub right: Box<Expr>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BinaryAddrPos {
    /// ptr + num
    /// ptr - num
    Left,
    /// num + ptr
    Right,
    /// ptr - ptr,
    Both,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BinaryOp {
    Add,
    Sub,
    AddrAdd(BinaryAddrPos),
    AddrSub(BinaryAddrPos),
    Mul,
    Div,
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Great,
    GreatEqual,
}

#[derive(Debug)]
pub struct UnaryExpr {
    pub loc: Loc,
    pub op: TokenType,
    pub argument: Box<Expr>,
    pub prefix: bool,
}

#[derive(Debug)]
pub struct DerefExpr {
    pub loc: Loc,
    pub argument: Box<Expr>,
}

#[derive(Debug)]
pub struct AddrExpr {
    pub loc: Loc,
    pub argument: Box<Expr>,
}
