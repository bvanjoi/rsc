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
    pub(crate) fn parse_top_level(&mut self, start: Pos) -> SResult<Program> {
        let mut body = vec![];
        while !self.cur_token().is_eof() {
            let stmt = self.parse_statement()?;
            body.push(stmt);
        }
        Ok(Program {
            loc: Loc::new(start, self.cur_pos()),
            body,
        })
    }

    fn parse_statement(&mut self) -> SResult<Stmt> {
        let start = self.cur_token_start();
        let expr = self.parse_expression()?;
        self.expect(&TokenType::Semi)?;
        let stmt = Stmt::Expr(ExprStmt {
            loc: self.finish_loc(start),
            expr,
        });
        Ok(stmt)
    }

    pub(crate) fn next(&mut self) -> SResult<()> {
        self.tokens[1] = self.cur_token().clone();
        self.next_token()
    }

    pub(crate) fn expect(&mut self, expected: &TokenType) -> SResult<()> {
        let token = self.cur_token();
        let actual = token.get_type();

        if actual != expected {
            self.unexpected(token)
        } else {
            self.next()
        }
    }
}
