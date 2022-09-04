use crate::{
    expression::Expr,
    state::{SResult, State},
    token::TokenType,
    utils::{Loc, Pos},
};

#[derive(Debug)]
pub enum Stmt {
    Expr(ExprStmt),
}

#[derive(Debug)]
pub struct ExprStmt {
    pub loc: Loc,
    pub expr: Expr,
}

#[derive(Debug)]
pub struct Program {
    pub loc: Loc,
    pub body: Vec<Stmt>,
}

impl State {
    pub fn parse_top_level(&mut self, start: Pos) -> SResult<Program> {
        let mut body = vec![];
        while !matches!(self.cur_token.r#type, TokenType::Eof) {
            let stmt = self.parse_statement()?;
            body.push(stmt)
        }
        self.next()?;
        Ok(Program {
            loc: Loc::new(start, self.cur_pos()),
            body,
        })
    }

    pub fn parse_statement(&mut self) -> SResult<Stmt> {
        let start = self.cur_token_start();
        // TODO: match
        let stmt = {
            let expr = self.parse_expression()?;
            self.next()?;
            Stmt::Expr(ExprStmt {
                loc: self.finish_loc(start),
                expr,
            })
        };
        Ok(stmt)
    }
}
