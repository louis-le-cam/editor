use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
};

use glam::{u64vec2, U64Vec2};

pub struct Document {
    path: PathBuf,
    lines: Vec<String>,
    cursor: U64Vec2,
    dirty: bool,
}

impl Document {
    pub fn from_path(path: PathBuf) -> Self {
        let lines = match File::open(&path) {
            Ok(file) => BufReader::new(file)
                .lines()
                .collect::<Result<_, _>>()
                .unwrap_or(Vec::new()),
            Err(_) => Vec::new(),
        };

        Self {
            lines,
            path,
            cursor: u64vec2(0, 0),
            dirty: false,
        }
    }

    pub fn true_cursor_x(&self) -> u64 {
        self.lines
            .get(self.cursor.y as usize)
            .map(|line| self.cursor.x.min(line.len() as u64))
            .unwrap_or(0)
    }

    pub fn cursor(&self) -> U64Vec2 {
        self.cursor
    }

    pub fn true_cursor(&self) -> U64Vec2 {
        u64vec2(self.true_cursor_x(), self.cursor.y)
    }

    pub fn path(&self) -> &Path {
        self.path.as_path()
    }

    pub fn lines(&self) -> &Vec<String> {
        &self.lines
    }

    pub fn dirty(&self) -> bool {
        self.dirty
    }
}

/// Movement
impl Document {
    pub fn move_left(&mut self) {
        let true_cursor_x = self.true_cursor_x();

        if true_cursor_x == 0 {
            if self.cursor.y != 0 {
                self.cursor.y -= 1;
                self.cursor.x = self
                    .lines
                    .get(self.cursor.y as usize)
                    .map(String::len)
                    .unwrap_or(0) as u64;
            }
        } else {
            self.cursor.x = true_cursor_x.saturating_sub(1);
        }
    }

    pub fn move_right(&mut self) {
        if self.cursor.x
            >= self
                .lines
                .get(self.cursor.y as usize)
                .map(String::len)
                .unwrap_or(0) as u64
        {
            self.cursor.x = 0;
            if self.cursor.y > self.lines.len() as u64 {
                self.cursor.y = self.lines.len() as u64;
            } else {
                self.cursor.y = self.cursor.y.saturating_add(1);
            }
        } else {
            self.cursor.x = self.cursor.x.saturating_add(1)
        }
    }

    pub fn move_up(&mut self) {
        self.cursor.y = self.cursor.y.saturating_sub(1)
    }

    pub fn move_down(&mut self) {
        if self.cursor.y >= self.lines.len() as u64 {
            self.cursor.y = self.lines.len() as u64;
        } else {
            self.cursor.y = self.cursor.y.saturating_add(1);
        }
    }
}

/// Editing
impl Document {
    pub fn insert(&mut self, ch: char) {
        let true_cursor_x = self.true_cursor_x();

        if let Some(line) = self.lines.get_mut(self.cursor.y as usize) {
            let index = line
                .char_indices()
                .nth(true_cursor_x as usize)
                .map(|(i, _)| i)
                .unwrap_or(line.len());

            self.cursor.x = true_cursor_x + 1;
            line.insert(index, ch);

            self.dirty = true;
        }
    }
}
