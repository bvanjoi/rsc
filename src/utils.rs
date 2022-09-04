#[derive(Clone, Debug)]
pub struct Pos {
    pub line: usize,
    pub column: usize,
}

impl Pos {
    pub fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }
}

#[derive(Clone, Debug)]
pub struct Loc {
    start: Pos,
    end: Option<Pos>,
}

impl Loc {
    pub fn new(start: Pos, end: Pos) -> Self {
        Self {
            start,
            end: Some(end),
        }
    }

    pub fn get_start(&self) -> &Pos {
        &self.start
    }

    pub fn get_end(&self) -> &Pos {
        self.end.as_ref().unwrap()
    }
}
