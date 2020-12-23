use std::{fs, io, path::PathBuf};
use crate::editor::row::Row;

#[derive(Debug)]
pub struct OpenFile {
    rows: Vec<Row>,
    path: PathBuf,
}

impl OpenFile {

    pub fn new<P: Into<PathBuf>>(path: P) -> io::Result<Self> {
        let p = path.into();
        let file = fs::read_to_string(&p)?;
        let mut rows = Vec::new();
        for line in file.lines() {
            rows.push(Row::from(line));
        }
        Ok ( Self { rows, path: p } )
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
        Self::new("/home/chrisp/div/srv/src/app.rs").unwrap()
    }
}
