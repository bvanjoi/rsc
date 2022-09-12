use crate::{token::Token, utils::Pos};

#[derive(Debug)]
pub struct SError {
    inner: (Pos, SyntaxError),
}

impl SError {
    pub fn new(pos: Pos, error: SyntaxError) -> Self {
        Self {
            inner: (pos, error),
        }
    }
}

#[derive(Debug)]
pub enum SyntaxError {
    UnexpectedToken(Token),
}
