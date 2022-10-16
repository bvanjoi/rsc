use crate::{
    expression::Expr,
    state::{SResult, State},
    token::TokenType,
    utils::{Loc, Pos},
};

fn align(offset: usize, align: usize) -> usize {
    (offset + align - 1) / align * align
}

impl State {
    pub(crate) fn parse_top_level(&mut self, start: Pos) -> SResult<Program> {
        let mut body = vec![];

        while !self.cur_token().is_eof() {
            let stmt = self.parse_statement()?;
            body.push(stmt);
        }
        let stack_size = align(self.locals.size() * 8, 16);
        Ok(Program {
            loc: Loc::new(start, self.cur_pos()),
            body,
            stack_size,
        })
    }

    // TODO: scope
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
            TokenType::For => Stmt::For(self.parse_for_statement()?),
            TokenType::While => Stmt::While(self.parse_while_statement()?),
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

    fn parse_while_statement(&mut self) -> SResult<WhileStmt> {
        let start = self.cur_token_start();
        self.next()?;
        self.expect(&TokenType::ParenL)?;
        let test = self.parse_expression()?;
        self.expect(&TokenType::ParenR)?;
        let body = Box::new(self.parse_statement()?);
        let loc = self.finish_loc(start);
        Ok(WhileStmt { loc, test, body })
    }

    fn parse_for_statement(&mut self) -> SResult<ForStmt> {
        let start = self.cur_token_start();
        self.next()?;
        self.expect(&TokenType::ParenL)?;
        let init = if matches!(self.cur_token().get_type(), &TokenType::Semi) {
            None
        } else {
            Some(self.parse_expression()?)
        };
        self.expect(&TokenType::Semi)?;
        let test = if matches!(self.cur_token().get_type(), &TokenType::Semi) {
            None
        } else {
            Some(self.parse_expression()?)
        };
        self.expect(&TokenType::Semi)?;
        let update = if matches!(self.cur_token().get_type(), &TokenType::ParenR) {
            None
        } else {
            Some(self.parse_expression()?)
        };
        self.expect(&TokenType::ParenR)?;
        let body = Box::new(self.parse_statement()?);
        Ok(ForStmt {
            loc: self.finish_loc(start),
            init,
            test,
            update,
            body,
        })
    }

    fn parse_block(&mut self) -> SResult<BlockStmt> {
        let mut body = vec![];
        let start = self.cur_token_start();
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
        let start = self.cur_token_start();
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
        let start = self.cur_token_start();
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
