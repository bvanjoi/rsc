use crate::{
    error::{SError, SyntaxError},
    statement::Program,
    token::Token,
};

use super::utils::*;

pub type SResult<T> = Result<T, SError>;

pub struct State {
    pub(crate) pos: usize,
    cur_line: usize,
    line_start: usize,
    pub(crate) input: Vec<char>,
    pub(crate) tokens: Vec<Token>,
}

impl State {
    pub fn new(input: String) -> Self {
        Self {
            pos: 0,
            cur_line: 1,
            line_start: 0,
            input: input.chars().collect(),
            tokens: vec![],
        }
    }

    pub(super) fn cur_pos(&self) -> Pos {
        Pos::new(self.pos, self.cur_line, self.pos - self.line_start)
    }

    pub fn parse(&mut self) -> SResult<(Program, Vec<Token>)> {
        let start = self.cur_pos();
        self.next()?;
        let program = self.parse_top_level(start)?;
        Ok((program, self.tokens.clone()))
    }

    pub(super) fn unexpected(&self, token: &Token) -> SResult<()> {
        let err = SError::new(
            token.get_start().clone(),
            SyntaxError::UnexpectedToken(token.clone()),
        );
        Err(err)
    }
}
