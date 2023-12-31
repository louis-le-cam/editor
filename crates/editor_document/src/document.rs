use std::{
    fs::{self, File},
    io::{BufRead, BufReader},
    path::PathBuf,
};

use editor_action::DocumentAction;
use log::error;

use crate::{selection::InternalSelection, Selection};

pub enum DocumentName {
    Scratch,
    Path(PathBuf),
}

pub struct Document {
    name: DocumentName,
    lines: Vec<String>,
    selection: InternalSelection,
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
            name: DocumentName::Path(path),
            lines,
            selection: InternalSelection::new(),
            dirty: false,
        }
    }

    pub fn new_scratch() -> Self {
        Self {
            name: DocumentName::Scratch,
            lines: Vec::new(),
            selection: InternalSelection::new(),
            dirty: false,
        }
    }

    fn get_line_mut(&mut self, index: usize) -> &mut String {
        if index >= self.lines.len() {
            self.lines.extend(
                std::iter::repeat(String::new()).take(index.saturating_sub(self.lines.len()) + 1),
            );
        }

        &mut self.lines[index]
    }

    pub fn selection(&self) -> Selection {
        self.selection.to_selection(&self.lines)
    }

    pub fn display_name(&self) -> String {
        match &self.name {
            DocumentName::Scratch => "[scratch]".to_string(),
            DocumentName::Path(path) => path.display().to_string(),
        }
    }

    pub fn lines(&self) -> &Vec<String> {
        &self.lines
    }

    pub fn get_line(&self, index: usize) -> Option<&str> {
        self.lines.get(index).map(|line| line.as_str())
    }

    pub fn dirty(&self) -> bool {
        self.dirty
    }

    pub fn handle_action(&mut self, action: DocumentAction) {
        use editor_action::{DocumentAction::*, SingleLineDocumentAction::*};

        match action {
            SingleLine(action) => match action {
                MoveLeft => self.selection.move_left(&self.lines),
                MoveRight => self.selection.move_right(&self.lines),
                Insert { char } => {
                    let true_start = self.selection.true_start(&self.lines);

                    let line = self.get_line_mut(true_start.1);

                    let index = line
                        .char_indices()
                        .nth(true_start.0)
                        .map(|(i, _)| i)
                        .unwrap_or(line.len());

                    line.insert(index, char);

                    if true_start.1 == self.selection.true_end(&self.lines).1 {
                        self.selection.move_selection_right(&self.lines);
                    } else {
                        self.selection.extend_start_right(&self.lines);
                    }

                    self.dirty = true;
                }
                DeleteBefore => {
                    self.selection.extend_start_left(&self.lines);
                    if self.selection.true_start(&self.lines).1
                        == self.selection.true_end(&self.lines).1
                    {
                        self.selection.extend_end_left(&self.lines);
                    }

                    let true_start = self.selection.true_start(&self.lines);

                    let line = self.get_line_mut(true_start.1);

                    if true_start.0 != line.chars().count() {
                        line.remove(true_start.0);
                        self.dirty = true;
                    } else {
                        if let Some(after_cursor) = (true_start.1 + 1 < self.lines.len())
                            .then(|| self.lines.remove(true_start.1 + 1))
                        {
                            self.selection.extend_end_up();
                            self.lines
                                .get_mut(true_start.1)
                                .map(|line| line.push_str(&after_cursor));
                            self.dirty = true;
                        }
                    }
                }
            },
            MoveUp => self.selection.move_up(),
            MoveDown => self.selection.move_down(&self.lines),

            ExtendEndLeft => self.selection.extend_end_left(&self.lines),
            ExtendEndRight => self.selection.extend_end_right(&self.lines),
            ExtendEndUp => self.selection.extend_end_up(),
            ExtendEndDown => self.selection.extend_end_down(&self.lines),

            ExtendStartLeft => self.selection.extend_start_left(&self.lines),
            ExtendStartRight => self.selection.extend_start_left(&self.lines),
            ExtendStartUp => self.selection.extend_start_left(&self.lines),
            ExtendStartDown => self.selection.extend_start_left(&self.lines),

            MoveSelectionLeft => self.selection.move_selection_left(&self.lines),
            MoveSelectionRight => self.selection.move_selection_right(&self.lines),
            MoveSelectionUp => self.selection.move_selection_up(),
            MoveSelectionDown => self.selection.move_selection_down(&self.lines),

            InsertLineBeforeCursor => {
                let true_start = self.selection.true_start(&self.lines);

                let line = self.get_line_mut(true_start.1);

                let after_cursor = line.split_off(
                    line.char_indices()
                        .nth(true_start.0)
                        .map(|(i, _)| i)
                        .unwrap_or(line.len()),
                );

                self.lines.insert(true_start.1 + 1, after_cursor);

                self.selection.extend_start_right(&self.lines);
                self.selection.extend_end_down(&self.lines);

                self.dirty = true;
            }
            Write => {
                if !self.dirty {
                    return;
                }

                match &self.name {
                    DocumentName::Scratch => {}
                    DocumentName::Path(path) => {
                        if let Err(err) = fs::write(&path, self.lines.join("\n")) {
                            error!("Failed to write document to {}, {:?}", path.display(), err);
                        } else {
                            self.dirty = false;
                        };
                    }
                }
            }
        }
    }
}
