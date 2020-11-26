use std::{io, fs};
use crate::editor::row::Row;

pub struct OpenFile {
    rows: Vec<Row>,

}

impl OpenFile {

    pub fn new(path: &str) -> io::Result<Self> {
        let file = fs::read_to_string(path)?;
        let mut rows = Vec::new();
        for line in file.lines() {
            rows.push(Row::from(line));
        }
        Ok ( Self { rows } )
    }

    pub fn get(&self, idx: usize) -> Option<&Row> {
        self.rows.get(idx as usize)
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }
}

impl Default for OpenFile {
    fn default() -> Self {
        // Self { rows: Vec::new() }
        Self::new("/home/chrisp/div/srv/src/app.rs").unwrap()
    }
}
