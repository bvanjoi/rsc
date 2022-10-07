use crate::token::Token;

type Pos = usize;
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
    UnexpectedChar,
    UnexpectedToken(Token),
    CastWrong,
}
