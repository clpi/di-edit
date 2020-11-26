use std::cmp;
use unicode_segmentation::UnicodeSegmentation;

pub struct Row {
    row: String,
    len: usize,
}

impl Row {

    pub fn render(&self, start: usize, end: usize) -> String {
        let end = cmp::min(end as usize, self.row.len() as usize) as usize;
        let start = cmp::min(start as usize, end) as usize;
        self.row.get(start..end).unwrap_or_default().to_string();
        let mut res = String::new();
        for grapheme in self.row[..]
            .graphemes(true)
            .skip(start)
            .take(end-start)
        {
            if grapheme == "\t" {
                res.push_str(" ");
            } else {
                res.push_str(grapheme);
            }
        }
        res
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0_usize
    }

    pub fn update_len(&mut self) {
        self.len = self.row[..].graphemes(true).count();
    }
}

impl From<&str> for Row {

    fn from(string: &str) -> Self {
        let mut row = Self {
            row: String::from(string), len: 0
        };
        row.update_len();
        row
    }

}
