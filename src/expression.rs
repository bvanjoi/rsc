use crate::{
    ast::*,
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
        let start = self.cur_token_start();
        let left = self.parse_operations()?;
        let tt = self.cur_token().get_type();
        if tt.assign() {
            self.next()?;
            let right = self.parse_maybe_assign()?;
            let loc = self.finish_loc(start);
            // expr to left
            let left = match left {
                Expr::Ident(expr) => Box::new(LeftVal::Ident(expr)),
                Expr::Deref(expr) => Box::new(LeftVal::Deref(expr)),
                _ => {
                    return Err(SError::new(
                        left.loc().get_start().pos,
                        SyntaxError::CastWrong,
                    ))
                }
            };
            Ok(Expr::Assign(AssignExpr {
                loc,
                left,
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
        let token = self.cur_token().clone();
        let tt = token.get_type().clone();
        if let Some(prec) = tt.prec() {
            // prec:
            // high    low
            //  1      15
            if prec >= min_prec {
                return Ok(left);
            }
            self.next()?;
            let right_start = self.cur_token_start();
            let right_expr = self.parse_maybe_unary()?;
            let right = Box::new(self.parse_operation(right_expr, right_start, prec)?);
            let loc = self.finish_loc(left_start.clone());
            let left = Box::new(left);
            let expr = match tt {
                TokenType::Plus => self.add_binary(left, token, right, loc)?,
                TokenType::Minus => self.sub_binary(left, token, right, loc)?,
                _ => BinaryExpr {
                    left,
                    right,
                    op: tt.binary_op(),
                    loc,
                },
            };
            self.parse_operation(Expr::Binary(expr), left_start, min_prec)
        } else {
            Ok(left)
        }
    }

    fn add_binary(
        &self,
        left: Box<Expr>,
        token: Token,
        right: Box<Expr>,
        loc: Loc,
    ) -> SResult<BinaryExpr> {
        match (contain_addr(&left), contain_addr(&right)) {
            // ptr1 + ptr2
            (true, true) => Err(SError::new(
                token.get_start().pos,
                SyntaxError::UnexpectedToken(token),
            )),
            // num1 + num2
            (false, false) => Ok(BinaryExpr {
                left,
                right,
                op: BinaryOp::Add,
                loc,
            }),
            // ptr + num
            (true, false) => Ok(BinaryExpr {
                left,
                op: BinaryOp::AddrAdd(BinaryAddrPos::Left),
                right,
                loc,
            }),
            // num + ptr
            (false, true) => Ok(BinaryExpr {
                left,
                op: BinaryOp::AddrAdd(BinaryAddrPos::Right),
                right,
                loc,
            }),
        }
    }

    fn sub_binary(
        &self,
        left: Box<Expr>,
        token: Token,
        right: Box<Expr>,
        loc: Loc,
    ) -> SResult<BinaryExpr> {
        match (contain_addr(&left), contain_addr(&right)) {
            // num1 - num2
            (false, false) => Ok(BinaryExpr {
                loc,
                left,
                op: BinaryOp::Sub,
                right,
            }),
            // ptr - num
            (true, false) => Ok(BinaryExpr {
                loc,
                left,
                op: BinaryOp::AddrSub(BinaryAddrPos::Left),
                right,
            }),
            // ptr - ptr
            (true, true) => Ok(BinaryExpr {
                loc,
                left,
                op: BinaryOp::AddrSub(BinaryAddrPos::Both),
                right,
            }),
            // num - ptr
            (false, true) => Err(SError::new(
                token.get_start().pos,
                SyntaxError::UnexpectedToken(token),
            )),
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
        } else if tt.eq(&TokenType::And) {
            self.next()?;
            let argument = self.parse_maybe_unary()?;
            let loc = self.finish_loc(start);
            Expr::Addr(AddrExpr {
                loc,
                argument: Box::new(argument),
            })
        } else if tt.eq(&TokenType::Star) {
            self.next()?;
            let argument = self.parse_maybe_unary()?;
            let loc = self.finish_loc(start);
            Expr::Deref(DerefExpr {
                loc,
                argument: Box::new(argument),
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
        let start = self.cur_token_start();
        let tt = self.cur_token().get_type();
        let name = match tt {
            TokenType::Name(name) => name.to_string(),
            _ => unreachable!(),
        };
        self.next()?;
        let offset = self.locals.offset(&name);
        let expr = IdentExpr {
            loc: self.finish_loc(start),
            name,
            offset,
        };
        Ok(expr)
    }

    fn parse_literal(&mut self, tt: TokenType) -> SResult<Lit> {
        let start = self.cur_token_start();
        self.next()?;

        let literal = match tt {
            TokenType::Int32(num) => {
                let loc = self.finish_loc(start);
                Lit::Int32(Int32Lit { loc, num })
            }
            _ => unreachable!(),
        };
        Ok(literal)
    }

    pub(super) fn parse_paren_expr(&mut self) -> SResult<Expr> {
        self.expect(&TokenType::ParenL)?;
        let expr = self.parse_expression()?;
        self.expect(&TokenType::ParenR)?;
        Ok(expr)
    }
}

fn contain_addr(expr: &Expr) -> bool {
    if expr.is_addr() {
        true
    } else {
        expr.as_binary()
            .map(|bin| &bin.op)
            .and_then(|op| match op {
                BinaryOp::AddrAdd(pos) | BinaryOp::AddrSub(pos) => Some(pos),
                _ => None,
            })
            .map(|pos| matches!(pos, BinaryAddrPos::Left | BinaryAddrPos::Right))
            .unwrap_or_default()
    }
}
