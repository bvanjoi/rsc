use crate::{
    error::{SError, SyntaxError},
    state::{SResult, State},
    token::{Token, TokenType},
    utils::{Loc, Pos},
};

impl State {
    pub(crate) fn cur_token(&self) -> &Token {
        &self.tokens[0]
    }

    pub(crate) fn last_token(&self) -> &Token {
        &self.tokens[1]
    }

    pub(crate) fn cur_token_start(&self) -> Pos {
        self.cur_token().get_start()
    }

    pub fn finish_loc(&self, start: Pos) -> Loc {
        let end = self.last_token().get_end();
        Loc::new(start, end)
    }

    pub fn parse_expression(&mut self) -> SResult<Expr> {
        let _start = self.cur_token_start();
        self.parse_maybe_assign()
    }

    fn parse_maybe_assign(&mut self) -> SResult<Expr> {
        let start = self.cur_pos();
        let left = self.parse_operations()?;
        let tt = self.cur_token().get_type();
        if tt.assign() {
            self.next()?;
            let right = self.parse_maybe_assign()?;
            let loc = self.finish_loc(start);
            Ok(Expr::Assign(AssignExpr {
                loc,
                left: Box::new(left.as_ident()?),
                right: Box::new(right),
            }))
        } else {
            Ok(left)
        }
    }

    fn parse_operations(&mut self) -> SResult<Expr> {
        let start = self.cur_token_start();
        let expr = self.parse_maybe_unary()?;
        self.parse_operation(expr, start, 16)
    }

    fn parse_operation(&mut self, left: Expr, left_start: Pos, min_prec: u16) -> SResult<Expr> {
        let tt = self.cur_token().get_type().clone();
        if let Some(prec) = tt.prec() {
            // prec:
            // high    low
            //  1      15
            if prec > min_prec {
                return Ok(left);
            }
            self.next()?;
            let start = self.cur_token_start();
            let expr = self.parse_maybe_unary()?;
            let right = self.parse_operation(expr, start, prec)?;
            let loc = self.finish_loc(left_start);
            let expr = Expr::Binary(BinaryExpr {
                left: Box::new(left),
                right: Box::new(right),
                op: tt,
                loc,
            });
            Ok(expr)
        } else {
            Ok(left)
        }
    }

    fn parse_maybe_unary(&mut self) -> SResult<Expr> {
        let start = self.cur_token_start();
        let tt = self.cur_token().get_type().clone();
        let expr = if tt.prefix() {
            self.next()?;
            let argument = self.parse_maybe_unary()?;
            let loc = self.finish_loc(start);
            Expr::Unary(UnaryExpr {
                loc,
                op: tt,
                argument: Box::new(argument),
                prefix: true,
            })
        } else {
            self.parse_atom()?
        };
        Ok(expr)
    }

    fn parse_atom(&mut self) -> SResult<Expr> {
        let token = self.cur_token();
        let tt = token.get_type().clone();
        let expr = match &tt {
            TokenType::Name(_) => Expr::Ident(self.parse_ident()?),
            TokenType::Int32(_) => Expr::Literal(self.parse_literal(tt)?),
            TokenType::ParenL => self.parse_paren_expr()?,
            _ => self.unexpected(token)?,
        };
        Ok(expr)
    }

    fn parse_ident(&mut self) -> SResult<IdentExpr> {
        let start = self.cur_pos();
        let tt = self.cur_token().get_type();
        let name = match tt {
            TokenType::Name(name) => name.to_string(),
            _ => unreachable!(),
        };
        self.next()?;
        let expr = IdentExpr {
            loc: self.finish_loc(start),
            name,
        };
        Ok(expr)
    }

    fn parse_literal(&mut self, tt: TokenType) -> SResult<Literal> {
        let start = self.cur_token_start();
        self.next()?;

        let literal = match tt {
            TokenType::Int32(num) => {
                let loc = self.finish_loc(start);
                Literal::Int32(Int32Literal { loc, num })
            }
            _ => unreachable!(),
        };
        Ok(literal)
    }

    fn parse_paren_expr(&mut self) -> SResult<Expr> {
        self.expect(&TokenType::ParenL)?;
        let expr = self.parse_expression()?;
        self.expect(&TokenType::ParenR)?;
        Ok(expr)
    }
}

#[derive(Debug)]
pub enum Expr {
    Binary(BinaryExpr),
    Literal(Literal),
    Unary(UnaryExpr),
    Assign(AssignExpr),
    Ident(IdentExpr),
}

impl Expr {
    pub fn as_ident(self) -> SResult<IdentExpr> {
        match self {
            Expr::Ident(ident) => Ok(ident),
            _ => Err(SError::new(
                self.loc().get_start().pos,
                SyntaxError::CastWrong,
            )),
        }
    }

    pub fn loc(&self) -> Loc {
        match self {
            Expr::Binary(expr) => expr.loc.clone(),
            Expr::Literal(expr) => expr.loc(),
            Expr::Unary(expr) => expr.loc.clone(),
            Expr::Assign(expr) => expr.loc.clone(),
            Expr::Ident(expr) => expr.loc.clone(),
        }
    }
}

#[derive(Debug)]
pub struct IdentExpr {
    pub loc: Loc,
    pub name: String,
}

#[derive(Debug)]
pub struct AssignExpr {
    pub loc: Loc,
    // TODO: left_val
    pub left: Box<IdentExpr>,
    pub right: Box<Expr>,
}

#[derive(Debug)]
pub struct BinaryExpr {
    pub loc: Loc,
    pub left: Box<Expr>,
    pub op: TokenType,
    pub right: Box<Expr>,
}

#[derive(Debug)]
pub struct UnaryExpr {
    pub loc: Loc,
    pub op: TokenType,
    pub argument: Box<Expr>,
    pub prefix: bool,
}

#[derive(Debug)]
pub enum Literal {
    Int32(Int32Literal),
}

impl Literal {
    pub fn loc(&self) -> Loc {
        match self {
            Literal::Int32(lit) => lit.loc.clone(),
        }
    }
}

#[derive(Debug)]
pub struct Int32Literal {
    pub loc: Loc,
    pub num: String,
}
