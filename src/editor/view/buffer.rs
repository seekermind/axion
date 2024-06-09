use std::{fs::read_to_string, io::Error};

use super::line::Line;

#[derive(Default)]
pub struct Buffer {
   pub lines: Vec<Line>,
}

impl Buffer {
    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }
    pub fn load(file_name: &str) -> Result<Self, Error> {
        let content = read_to_string(file_name)?;
        let mut lines = Vec::new();
        for line in content.lines() {
            lines.push(Line::from(line));
        }
        Ok(Self { lines })
    }
}
