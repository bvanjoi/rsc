use crate::{
    expression::Expr,
    state::{SResult, State},
    token::TokenType,
    utils::{Loc, Pos},
};

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
        let tt = self.cur_token().get_type();
        let stmt = match tt {
            TokenType::Return => {
                let stmt = self.parse_return_statement()?;
                Stmt::Return(stmt)
            }
            TokenType::BraceL => {
                let stmt = self.parse_block()?;
                Stmt::Block(stmt)
            }
            TokenType::Semi => {
                self.next()?;
                let loc = self.finish_loc(start);
                Stmt::Empty(EmptyStmt { loc })
            }
            TokenType::If => Stmt::If(self.parse_if_statement()?),
            _ => {
                let expr = self.parse_expression()?;
                self.expect(&TokenType::Semi)?;
                Stmt::Expr(ExprStmt {
                    loc: self.finish_loc(start),
                    expr,
                })
            }
        };

        Ok(stmt)
    }

    fn parse_block(&mut self) -> SResult<BlockStmt> {
        let mut body = vec![];
        let start = self.cur_pos();
        self.expect(&TokenType::BraceL)?;
        loop {
            let tt = self.cur_token().get_type();
            if !matches!(tt, &TokenType::BraceR) {
                let stmt = self.parse_statement()?;
                body.push(stmt);
            } else {
                break;
            }
        }
        self.next()?;
        Ok(BlockStmt {
            loc: self.finish_loc(start),
            body,
        })
    }

    fn parse_if_statement(&mut self) -> SResult<IfStmt> {
        let start = self.cur_pos();
        self.next()?;
        let test = self.parse_paren_expr()?;
        let consequent = Box::new(self.parse_statement()?);

        let alternate = if self.eat(&TokenType::Else)? {
            Some(Box::new(self.parse_statement()?))
        } else {
            None
        };

        Ok(IfStmt {
            loc: self.finish_loc(start),
            test,
            consequent,
            alternate,
        })
    }

    fn parse_return_statement(&mut self) -> SResult<ReturnStmt> {
        let start = self.cur_pos();
        self.next()?;
        let stmt = if self.eat(&TokenType::Semi)? {
            ReturnStmt {
                loc: self.finish_loc(start),
                argument: None,
            }
        } else {
            let argument = self.parse_expression()?;
            self.expect(&TokenType::Semi)?;
            ReturnStmt {
                loc: self.finish_loc(start),
                argument: Some(argument),
            }
        };
        Ok(stmt)
    }

    pub(crate) fn next(&mut self) -> SResult<()> {
        self.tokens[1] = self.cur_token().clone();
        self.next_token()
    }

    pub(crate) fn eat(&mut self, expected: &TokenType) -> SResult<bool> {
        let token = self.cur_token();
        let actual = token.get_type();
        if actual == expected {
            self.next()?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub(crate) fn expect(&mut self, expected: &TokenType) -> SResult<()> {
        if self.eat(expected)? {
            Ok(())
        } else {
            let token = self.cur_token();
            self.unexpected(token)
        }
    }
}

#[derive(Debug)]
pub enum Stmt {
    Expr(ExprStmt),
    Return(ReturnStmt),
    Block(BlockStmt),
    Empty(EmptyStmt),
    If(IfStmt),
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
pub struct Program {
    pub loc: Loc,
    pub body: Vec<Stmt>,
}

#[derive(Debug)]
pub struct ReturnStmt {
    pub loc: Loc,
    pub argument: Option<Expr>,
}
