pub struct Buffer {
    pub file: Option<String>,
    pub lines: Vec<String>,
}

impl Buffer {
    pub fn from_file(file: Option<String>) -> Self {
        let lines = match &file {
            Some(file) => match std::fs::read_to_string(file) {
                Ok(content) => content.lines().map(|s| s.to_string()).collect(),
                Err(_) => vec![],
            },
            None => vec![],
        };

        Self { file, lines }
    }

    pub fn get(&self, line: usize) -> Option<String> {
        if line < self.lines.len() {
            Some(self.lines[line].clone())
        } else {
            None
        }
    }
    pub fn len(&self) -> usize {
        self.lines.len()
    }

    pub fn insert(&mut self, x: u16, y: u16, c: char) {
        if let Some(line) = self.lines.get_mut(y as usize) {
            (*line).insert(x as usize, c);
        }
    }

    pub fn remove(&mut self, x: u16, y: u16) {
        if let Some(line) = self.lines.get_mut(y as usize) {
            (*line).remove(x as usize);
        }
    }

    pub fn remove_line(&mut self, line: u16) {
        if self.len() > line as usize {
            self.lines.remove(line as usize);
        }
    }
}
