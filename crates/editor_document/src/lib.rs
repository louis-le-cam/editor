mod action;

use std::{
    fs::{self, File},
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
};

use action::Action;
use glam::{u64vec2, U64Vec2};
use log::error;

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

    pub fn execute(&mut self, action: &Action) {
        action.execute(self);
    }

    pub fn write(&mut self) {
        if !self.dirty {
            return;
        }

        if let Err(err) = fs::write(&self.path, self.lines.join("\n")) {
            error!(
                "Failed to write document to {}, {:?}",
                self.path.display(),
                err
            );
        } else {
            self.dirty = false;
        };
    }

    fn true_cursor_x(&self) -> u64 {
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

/// Cursor movement
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
            if self.cursor.y >= self.lines.len() as u64 {
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
        let true_cursor = self.true_cursor();

        let line = self.get_line_mut(true_cursor.y as usize);

        let index = line
            .char_indices()
            .nth(true_cursor.x as usize)
            .map(|(i, _)| i)
            .unwrap_or(line.len());

        line.insert(index, ch);
        self.cursor.x = true_cursor.x + 1;

        self.dirty = true;
    }

    pub fn delete_before(&mut self) {
        self.move_left();

        let true_cursor = self.true_cursor();

        let line = self.get_line_mut(true_cursor.y as usize);

        if true_cursor.x != line.chars().count() as u64 {
            line.remove(true_cursor.x as usize);
            self.dirty = true;
        } else {
            if let Some(after_cursor) = (true_cursor.y + 1 < self.lines.len() as u64)
                .then(|| self.lines.remove(true_cursor.y as usize + 1))
            {
                self.lines
                    .get_mut(true_cursor.y as usize)
                    .map(|line| line.push_str(&after_cursor));
                self.dirty = true;
            }
        }
    }

    pub fn insert_line_before_cursor(&mut self) {
        let true_cursor = self.true_cursor();

        let line = self.get_line_mut(true_cursor.y as usize);

        let after_cursor = line.split_off(
            line.char_indices()
                .nth(true_cursor.x as usize)
                .map(|(i, _)| i)
                .unwrap_or(line.len()),
        );

        self.lines.insert(true_cursor.y as usize + 1, after_cursor);

        self.move_right();
    }

    fn get_line_mut(&mut self, index: usize) -> &mut String {
        if self.lines.len() <= index {
            self.lines.extend(
                std::iter::repeat(String::new())
                    .take((self.cursor.y as usize + 1).saturating_sub(self.lines.len())),
            );
        }

        &mut self.lines[index]
    }
}
