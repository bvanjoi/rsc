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
    pub(crate) tokens: [Token; 2],
}

impl State {
    pub fn new(input: String) -> Self {
        Self {
            pos: 0,
            cur_line: 1,
            line_start: 0,
            input: input.chars().collect(),
            tokens: [Token::eof(), Token::eof()],
        }
    }

    pub(super) fn cur_pos(&self) -> Pos {
        Pos::new(self.pos, self.cur_line, self.pos - self.line_start)
    }

    pub fn parse(&mut self) -> SResult<Program> {
        let start = self.cur_pos();
        self.next_token()?;
        let program = self.parse_top_level(start)?;
        Ok(program)
    }

    pub(super) fn unexpected<T>(&self, token: &Token) -> SResult<T> {
        // TODO: lines
        let err = SError::new(
            token.get_start().pos,
            SyntaxError::UnexpectedToken(token.clone()),
        );
        Err(err)
    }
}
