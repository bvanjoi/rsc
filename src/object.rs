use std::collections::HashMap;

pub type Offset = usize;

#[derive(Debug)]
pub struct Object {
    content: HashMap<String, Offset>,
}

impl Object {
    pub fn new() -> Self {
        Self {
            content: Default::default(),
        }
    }

    pub fn offset(&mut self, name: &str) -> Offset {
        if let Some(offset) = self.content.get(name) {
            offset.clone()
        } else {
            let offset = self.content.len();
            self.content.insert(name.to_string(), offset);
            offset
        }
    }

    pub fn size(&self) -> usize {
        self.content.len()
    }
}
