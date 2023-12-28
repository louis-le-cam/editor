use std::{
    fs::{self, File},
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
};

use editor_action::DocumentActionHandler;
use log::error;

use crate::selection::Selection;

pub struct Document {
    path: PathBuf,
    lines: Vec<String>,
    selection: Selection,
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
            selection: Selection::new(),
            dirty: false,
        }
    }

    fn get_line_mut(&mut self, index: usize) -> &mut String {
        if index < self.lines.len() {
            self.lines.extend(
                std::iter::repeat(String::new()).take(index.saturating_sub(self.lines.len())),
            );
        }

        &mut self.lines[index]
    }

    pub fn selection(&self) -> ((usize, usize), (usize, usize)) {
        (
            self.selection.true_start(&self.lines),
            self.selection.true_end(&self.lines),
        )
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

impl DocumentActionHandler for Document {
    fn move_left(&mut self) {
        self.selection.move_left(&self.lines);
    }

    fn move_right(&mut self) {
        self.selection.move_right(&self.lines);
    }

    fn move_up(&mut self) {
        self.selection.move_up();
    }

    fn move_down(&mut self) {
        self.selection.move_down(&self.lines);
    }

    fn insert(&mut self, ch: char) {
        let true_start = self.selection.true_start(&self.lines);

        let line = self.get_line_mut(true_start.1);

        let index = line
            .char_indices()
            .nth(true_start.0)
            .map(|(i, _)| i)
            .unwrap_or(line.len());

        line.insert(index, ch);

        self.selection.move_right(&self.lines);

        self.dirty = true;
    }

    fn delete_before(&mut self) {
        self.selection.move_left(&self.lines);

        let true_start = self.selection.true_start(&self.lines);

        let line = self.get_line_mut(true_start.1);

        if true_start.0 != line.chars().count() {
            line.remove(true_start.0);
            self.dirty = true;
        } else {
            if let Some(after_cursor) =
                (true_start.1 + 1 < self.lines.len()).then(|| self.lines.remove(true_start.1 + 1))
            {
                self.lines
                    .get_mut(true_start.1)
                    .map(|line| line.push_str(&after_cursor));
                self.dirty = true;
            }
        }
    }

    fn insert_line_before_cursor(&mut self) {
        let true_start = self.selection.true_start(&self.lines);

        let line = self.get_line_mut(true_start.1);

        let after_cursor = line.split_off(
            line.char_indices()
                .nth(true_start.0)
                .map(|(i, _)| i)
                .unwrap_or(line.len()),
        );

        self.lines.insert(true_start.1 + 1, after_cursor);

        self.move_right();

        self.dirty = true;
    }

    fn write(&mut self) {
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
}
