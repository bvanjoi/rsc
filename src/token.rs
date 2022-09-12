use super::state::{SResult, State};
use super::utils::Pos;
use crate::utils::Loc;

#[derive(Clone, Debug, PartialEq)]
pub enum TokenType {
    Eof,
    Int32(String),
    Plus,
    Minus,
}

#[derive(Clone, Debug)]
pub struct Token {
    loc: Option<Loc>,
    r#type: TokenType,
}

impl Token {
    pub fn get_start(&self) -> Pos {
        self.loc.as_ref().unwrap().get_start().clone()
    }

    pub fn get_end(&self) -> Pos {
        self.loc.as_ref().unwrap().get_end().clone()
    }

    pub fn get_type(&self) -> &TokenType {
        &self.r#type
    }

    pub fn is_eof(&self) -> bool {
        self.r#type.eq(&TokenType::Eof)
    }
}

impl State {
    fn finish_token(&mut self, start: Pos, r#type: TokenType) -> SResult<()> {
        let end = self.cur_pos();
        let token = Token {
            loc: Some(Loc::new(start, end)),
            r#type,
        };

        self.tokens.push(token);
        // self.cur_token = token;
        Ok(())
    }

    fn read_number(&mut self) -> SResult<()> {
        let start = self.cur_pos();
        let mut num = String::new();
        while self.pos < self.input.len() {
            let char = self.input[self.pos];
            if ('0'..='9').contains(&char) {
                num.push(char);
                self.pos += 1;
            } else {
                break;
            }
        }

        self.finish_token(start, TokenType::Int32(num))
    }

    pub(super) fn next(&mut self) -> SResult<()> {
        let start = self.cur_pos();
        self.skip_space()?;
        if self.pos >= self.input.len() {
            self.finish_token(start, TokenType::Eof)
        } else {
            let char = self.input[self.pos];
            match char {
                '0'..='9' => self.read_number(),
                '+' => self.read_plus(),
                '-' => self.read_minus(),
                _ => unreachable!(),
            }
        }
    }

    fn read_plus(&mut self) -> SResult<()> {
        let start = self.cur_pos();
        // TODO: +=, ++a, a++,
        self.pos += 1;
        self.finish_token(start, TokenType::Plus)
    }

    fn read_minus(&mut self) -> SResult<()> {
        let start = self.cur_pos();
        // TODO: -=, --a, a--,
        self.pos += 1;
        self.finish_token(start, TokenType::Minus)
    }

    #[inline]
    fn skip_space(&mut self) -> SResult<()> {
        while self.pos < self.input.len() {
            let ch = self.input[self.pos];
            if ch == ' ' {
                self.pos += 1;
            } else {
                break;
            }
        }
        Ok(())
    }
}
