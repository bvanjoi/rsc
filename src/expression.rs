use crate::{
    state::{SResult, State},
    token::TokenType,
    utils::{Loc, Pos},
};

#[derive(Debug)]
pub enum Expr {
    Binary(BinaryExpr),
    Literal(Literal),
}

#[derive(Debug)]
pub struct BinaryExpr {
    pub loc: Loc,
    pub left: Box<Expr>,
    pub op: TokenType,
    pub right: Box<Expr>,
}

#[derive(Debug)]
pub enum Literal {
    Int32(Int32Literal),
}

#[derive(Debug)]
pub struct Int32Literal {
    pub loc: Loc,
    pub num: String,
}

impl State {
    pub fn cur_token_start(&self) -> Pos {
        self.cur_token.loc.get_start().clone()
    }

    pub fn finish_loc(&self, start: Pos) -> Loc {
        let end = self.last_token.as_ref().unwrap().loc.get_end().clone();
        Loc::new(start, end)
    }

    pub fn parse_expression(&mut self) -> SResult<Expr> {
        // let start = self.cur_token_start();
        self.parse_operations()
    }

    fn parse_operations(&mut self) -> SResult<Expr> {
        let start = self.cur_token_start();
        let expr = self.parse_atom()?;
        self.parse_operation(expr, start)
    }

    fn parse_operation(&mut self, left: Expr, left_start: Pos) -> SResult<Expr> {
        let tt = self.cur_token.r#type.clone();
        if !matches!(tt, TokenType::Minus | TokenType::Plus) {
            return Ok(left);
        }
        self.next()?;
        let start = self.cur_token_start();
        let expr = self.parse_atom()?;
        // TODO: precedence
        let right = self.parse_operation(expr, start)?;
        let loc = self.finish_loc(left_start);
        let expr = Expr::Binary(BinaryExpr {
            left: Box::new(left),
            right: Box::new(right),
            op: tt,
            loc,
        });
        Ok(expr)
    }

    fn parse_atom(&mut self) -> SResult<Expr> {
        let tt = self.cur_token.r#type.clone();
        let literal = self.parse_literal(tt)?;
        Ok(Expr::Literal(literal))
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
}
