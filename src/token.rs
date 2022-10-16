use super::state::{SResult, State};
use super::utils::Pos;
use crate::error::{SError, SyntaxError};
use crate::utils::Loc;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TokenType {
    Eof,
    Int32(String),
    Plus,
    Minus,
    Star,
    Slash,
    ParenL,
    ParenR,
    BraceL,
    BraceR,
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Great,
    GreatEqual,
    Semi,
    Assign,
    And,
    Name(String),
    // keyword
    If,
    Else,
    Return,
    For,
    While,
}

impl TokenType {
    /// https://en.cppreference.com/w/c/language/operator_precedence
    pub const fn prec(&self) -> Option<u16> {
        use TokenType::*;
        match self {
            // high       low
            //  1          15
            Plus | Minus => Some(4),
            Star | Slash => Some(3),
            Equal | NotEqual | Less | LessEqual | Great | GreatEqual => Some(7),
            _ => None,
        }
    }

    pub const fn prefix(&self) -> bool {
        use TokenType::*;
        matches!(self, Plus | Minus)
    }

    pub const fn assign(&self) -> bool {
        use TokenType::*;
        matches!(self, Assign)
    }
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
        matches!(self.r#type, TokenType::Eof)
    }

    pub const fn eof() -> Self {
        Token {
            loc: None,
            r#type: TokenType::Eof,
        }
    }
}

impl State {
    fn finish_token(&mut self, start: Pos, r#type: TokenType) -> SResult<()> {
        let end = self.cur_pos();
        let token = Token {
            loc: Some(Loc::new(start, end)),
            r#type,
        };

        self.tokens[0] = token;
        Ok(())
    }

    pub(super) fn next_token(&mut self) -> SResult<()> {
        let start = self.cur_pos();
        self.skip_space()?;
        if self.pos >= self.input.len() {
            self.finish_token(start, TokenType::Eof)
        } else {
            let char = self.input[self.pos];
            match char {
                '0'..='9' => self.read_number(),
                '&' => self.read_and(),
                '+' => self.read_plus(),
                '-' => self.read_minus(),
                '*' => self.read_star(),
                '/' => self.read_slash(),
                '(' => {
                    self.pos += 1;
                    self.finish_token(start, TokenType::ParenL)
                }
                ')' => {
                    self.pos += 1;
                    self.finish_token(start, TokenType::ParenR)
                }
                '{' => {
                    self.pos += 1;
                    self.finish_token(start, TokenType::BraceL)
                }
                '}' => {
                    self.pos += 1;
                    self.finish_token(start, TokenType::BraceR)
                }
                '=' => self.read_equal(),
                '!' => self.read_excl(),
                '<' => self.read_less(),
                '>' => self.read_greater(),
                ';' => {
                    self.pos += 1;
                    self.finish_token(start, TokenType::Semi)
                }
                _ => self.read_word(),
            }
        }
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

    fn is_valid_start(char: &char) -> bool {
        (&'a'..=&'z').contains(&char) || (&'A'..=&'Z').contains(&char) || char == &'_'
    }

    fn is_valid(char: &char) -> bool {
        Self::is_valid_start(char) || (&'0'..=&'9').contains(&char)
    }

    fn read_word(&mut self) -> SResult<()> {
        let start = self.cur_pos();
        let mut str = String::new();
        while self.pos < self.input.len() {
            let char = self.input[self.pos];
            if (self.pos == start.pos && Self::is_valid_start(&char)) || Self::is_valid(&char) {
                str.push(char);
            } else {
                break;
            }
            self.pos += 1;
        }
        let tt = self
            .is_keyword(&str)
            .cloned()
            .unwrap_or(TokenType::Name(str));
        self.finish_token(start, tt)
    }

    fn is_keyword(&self, str: &str) -> Option<&TokenType> {
        self.keywords.get(str)
    }

    fn read_equal(&mut self) -> SResult<()> {
        let start = self.cur_pos();
        if let Some(&char) = self.input.get(self.pos + 1) {
            if char == '=' {
                self.pos += 2;
                return self.finish_token(start, TokenType::Equal);
            }
        }
        self.pos += 1;
        self.finish_token(start, TokenType::Assign)
    }

    fn read_excl(&mut self) -> SResult<()> {
        let start = self.cur_pos();
        if let Some(&char) = self.input.get(self.pos + 1) {
            if char == '=' {
                self.pos += 2;
                return self.finish_token(start, TokenType::NotEqual);
            }
        }
        self.pos += 1;
        Err(SError::new(self.pos + 1, SyntaxError::UnexpectedChar))
    }

    fn read_and(&mut self) -> SResult<()> {
        let start = self.cur_pos();
        self.pos += 1;
        self.finish_token(start, TokenType::And)
    }

    fn read_less(&mut self) -> SResult<()> {
        let start = self.cur_pos();
        if let Some(&char) = self.input.get(self.pos + 1) {
            if char == '=' {
                self.pos += 2;
                return self.finish_token(start, TokenType::LessEqual);
            }
        }
        self.pos += 1;
        self.finish_token(start, TokenType::Less)
    }

    fn read_greater(&mut self) -> SResult<()> {
        let start = self.cur_pos();
        if let Some(&char) = self.input.get(self.pos + 1) {
            if char == '=' {
                self.pos += 2;
                return self.finish_token(start, TokenType::GreatEqual);
            }
        }
        self.pos += 1;
        self.finish_token(start, TokenType::Great)
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

    fn read_star(&mut self) -> SResult<()> {
        let start = self.cur_pos();
        // TODO: *=
        self.pos += 1;
        self.finish_token(start, TokenType::Star)
    }

    fn read_slash(&mut self) -> SResult<()> {
        let start = self.cur_pos();
        // TODO: /=
        self.pos += 1;
        self.finish_token(start, TokenType::Slash)
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
