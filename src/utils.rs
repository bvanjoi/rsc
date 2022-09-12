#[derive(Clone, Debug)]
pub struct Pos {
    pub line: usize,
    pub column: usize,
    pub pos: usize,
}

impl Pos {
    pub fn new(pos: usize, line: usize, column: usize) -> Self {
        Self { pos, line, column }
    }
}

#[derive(Clone, Debug)]
pub struct Loc {
    start: Pos,
    end: Option<Pos>,
}

impl Loc {
    pub(crate) fn new(start: Pos, end: Pos) -> Self {
        Self {
            start,
            end: Some(end),
        }
    }

    pub(crate) fn get_start(&self) -> &Pos {
        &self.start
    }

    pub(crate) fn get_end(&self) -> &Pos {
        self.end.as_ref().unwrap()
    }
}
