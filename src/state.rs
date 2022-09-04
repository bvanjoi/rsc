use crate::{statement::Program, token::Token};

use super::utils::*;

#[derive(Debug)]
pub struct StateError {}

pub type SResult<T> = Result<T, StateError>;

pub struct State {
    pub(crate) pos: usize,
    cur_line: usize,
    line_start: usize,
    pub(crate) input: Vec<char>,
    pub(crate) cur_token: Token,
    pub(crate) last_token: Option<Token>,
    pub p: Vec<String>,
}

impl State {
    pub fn new(input: String) -> Self {
        Self {
            cur_token: Token::eof(),
            pos: 0,
            cur_line: 1,
            line_start: 0,
            input: input.chars().collect(),
            last_token: None,
            p: vec![],
        }
    }

    pub fn cur_pos(&self) -> Pos {
        Pos::new(self.cur_line, self.pos - self.line_start)
    }

    pub fn parse(&mut self) -> SResult<Program> {
        let start = self.cur_pos();
        self.next()?;
        self.parse_top_level(start)
    }
}
