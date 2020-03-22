use crate::point::Point;
use std::ops::{Deref, DerefMut};
use unicode_width::UnicodeWidthChar;

/// A 2D string buffer
/// where you can insert a character to any cell on the 2D grid
/// each cell can be assigned with a string
/// taking into account utf8 code which can not be char
/// including but not limited to multi-width chars
pub struct StringBuffer(Vec<Vec<char>>);

impl Deref for StringBuffer {
    type Target = Vec<Vec<char>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for StringBuffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl StringBuffer {
    pub(in crate) fn new() -> Self {
        StringBuffer(vec![])
    }

    /// add rows to this buffer
    fn add_rows(&mut self, n: i32) {
        for _i in 0..n {
            self.push(vec![]);
        }
    }

    /// add columns to the specified row of this buffer
    fn add_column(&mut self, row: usize, n: i32) {
        for _i in 0..n {
            self[row].push(' ');
        }
    }

    /// x and y can also be negative
    pub fn add_char(&mut self, x: i32, y: i32, ch: char) {
        if x >= 0 && y >= 0 {
            if ch == '\0' {
                println!("skipping {}", ch);
            } else {
                let row_index = y as usize;
                let column_index = x as usize;
                let row_diff = y - self.len() as i32;
                if row_diff >= 0 {
                    self.add_rows(row_diff + 1);
                }
                let column = &self[y as usize];
                let column_diff = x - column.len() as i32;
                if column_diff >= 0 {
                    self.add_column(row_index as usize, column_diff + 1);
                }
                self[row_index][column_index] = ch;
            }
        }
    }

    pub fn add_str(&mut self, x: i32, y: i32, s: &str) {
        for (i, ch) in s.chars().enumerate() {
            self.add_char(x + i as i32, y, ch);
        }
    }
}

impl From<&str> for StringBuffer {
    fn from(s: &str) -> Self {
        let mut rows = vec![];
        for line in s.lines() {
            let mut row = vec![];
            for ch in line.chars() {
                row.push(ch);
                if let Some(width) = ch.width() {
                    for i in 1..width {
                        row.push('\0');
                    }
                }
            }
            rows.push(row);
        }

        StringBuffer(rows)
    }
}

impl ToString for StringBuffer {
    fn to_string(&self) -> String {
        let mut lines = vec![];
        for row in self.iter() {
            let row_contents: Vec<String> = row
                .iter()
                .filter(|ch| **ch != '\0')
                .map(ToString::to_string)
                .collect();
            let line = row_contents.join("").trim_end().to_string();
            lines.push(line);
        }
        lines.join("\n")
    }
}
