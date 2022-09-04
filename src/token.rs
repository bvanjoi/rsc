use super::state::{SResult, State};
use super::utils::Pos;
use crate::utils::Loc;

#[derive(Clone, Debug)]
pub enum TokenType {
    Eof,
    Int32(String),
    Plus,
    Minus,
}

#[derive(Clone, Debug)]
pub struct Token {
    pub loc: Loc,
    pub r#type: TokenType,
}

impl Token {
    pub fn eof() -> Token {
        Token {
            loc: Loc::new(Pos::new(0, 0), Pos::new(0, 0)),
            r#type: TokenType::Eof,
        }
    }
}

impl State {
    fn finish_token(&mut self, start: Pos, r#type: TokenType) -> SResult<()> {
        let end = self.cur_pos();
        self.cur_token = Token {
            loc: Loc::new(start, end),
            r#type,
        };
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

        {
            if start.column == 0 {
                self.p.push(format!("mov ${}, %rax", num))
            } else if matches!(self.last_token.as_ref().unwrap().r#type, TokenType::Minus) {
                self.p.push(format!("sub ${}, %rax", num))
            } else if matches!(self.last_token.as_ref().unwrap().r#type, TokenType::Plus) {
                self.p.push(format!("add ${}, %rax", num))
            }
        }

        self.finish_token(start, TokenType::Int32(num))
    }

    pub(super) fn next(&mut self) -> SResult<()> {
        self.last_token = Some(self.cur_token.clone());
        self.next_token()
    }

    pub fn next_token(&mut self) -> SResult<()> {
        let start = self.cur_pos();
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
}
