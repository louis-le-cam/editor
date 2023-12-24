use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

use editor_terminal::{Color, Event, KeyCode, KeyEventKind, TermRect, TermSlice, TermVec};
use glam::{u16vec2, U64Vec2};

pub struct TextEditor {
    lines: Vec<String>,
    cursor: U64Vec2,
    offset: U64Vec2,
}
impl TextEditor {
    pub fn from_path(path: impl AsRef<Path>) -> Self {
        Self {
            lines: io::BufReader::new(File::open(path).unwrap())
                .lines()
                .collect::<Result<_, _>>()
                .unwrap(),
            cursor: (0, 0).into(),
            offset: (0, 0).into(),
        }
    }

    pub fn draw(&mut self, mut term: TermSlice) {
        let gutter_width = (self.lines.len().checked_ilog10().unwrap_or(0) + 3) as u16;

        self.draw_gutter(term.slice(TermRect::new((0, 0), (gutter_width, term.rect().heigth()))));

        self.draw_code(term.slice(TermRect::new(
            (gutter_width, 0),
            term.rect().size.saturating_sub(u16vec2(gutter_width, 0)),
        )));
    }

    fn draw_gutter(&mut self, mut term: TermSlice) {
        let size = term.rect().size;

        for y in 0..size.y {
            let line = self.offset.y + y as u64;

            let line_number = match line {
                _ if line < self.lines.len() as u64 => {
                    format!(
                        " {: >width$} ",
                        line + 1,
                        width = size.x.saturating_sub(2) as usize
                    )
                }
                _ if line == self.lines.len() as u64 => {
                    format!(
                        " {: >width$} ",
                        '~',
                        width = size.x.saturating_sub(2) as usize
                    )
                }
                _ => " ".repeat(size.x as usize),
            };

            term.write_to((0, y), &line_number);
        }
    }

    fn draw_code(&mut self, mut term: TermSlice) {
        let size = term.rect().size;

        for y in 0..size.y {
            let line = self
                .lines
                .get(y as usize + self.offset.y as usize)
                .map(|line| line.as_str())
                .unwrap_or("");

            let mut chars = line
                .chars()
                .skip(self.offset.x as usize)
                .chain(" ".chars().cycle());

            if y as i64 == self.cursor.y as i64 - self.offset.y as i64 {
                let before_cursor_len = self.cursor.x.saturating_sub(self.offset.x) as usize;

                if self.cursor.x >= self.offset.x {
                    let before_cursor = (0..before_cursor_len)
                        .filter_map(|_| chars.next())
                        .collect::<String>();

                    term.write_to((0, y), &before_cursor);

                    let at_cursor = chars.next().map(|ch| ch.to_string()).unwrap();

                    term.set_background_color(Color::Red);
                    term.write_to((before_cursor_len as u16, y), &at_cursor);
                    term.set_background_color(Color::Reset);

                    let after_cursor = chars
                        .take((size.x as usize).saturating_sub(before_cursor_len + 1))
                        .collect::<String>();

                    term.write_to((before_cursor_len as u16 + 1, y), &after_cursor);
                } else {
                    term.write_to((0, y), &chars.take(size.x as usize).collect::<String>());
                }
            } else {
                term.write_to((0, y), &chars.take(size.x as usize).collect::<String>());
            }
        }
    }

    pub fn event(&mut self, term: TermSlice, event: &Event) {
        match event {
            Event::Key(key) => {
                if matches!(key.kind, KeyEventKind::Press | KeyEventKind::Repeat) {
                    if matches!(
                        key.code,
                        KeyCode::Left
                            | KeyCode::Right
                            | KeyCode::Up
                            | KeyCode::Down
                            | KeyCode::Char('h')
                            | KeyCode::Char('j')
                            | KeyCode::Char('k')
                            | KeyCode::Char('l')
                    ) {
                        match key.code {
                            KeyCode::Char('h') | KeyCode::Left => {
                                self.cursor.x = self.cursor.x.saturating_sub(1)
                            }
                            KeyCode::Char('j') | KeyCode::Down => {
                                self.cursor.y = self.cursor.y.saturating_add(1)
                            }
                            KeyCode::Char('k') | KeyCode::Up => {
                                self.cursor.y = self.cursor.y.saturating_sub(1)
                            }
                            KeyCode::Char('l') | KeyCode::Right => {
                                self.cursor.x = self.cursor.x.saturating_add(1)
                            }
                            _ => {}
                        }

                        let gutter_width =
                            (self.lines.len().checked_ilog10().unwrap_or(0) + 3) as u16;

                        self.update_offset(
                            term.rect().size.saturating_sub(u16vec2(gutter_width, 0)),
                        );

                        self.draw(term);
                    }
                }
            }
            _ => {}
        }
    }

    /// Update `self.offset` if `self.cursor` is near edges
    fn update_offset(&mut self, size: TermVec) {
        if self.cursor.x + 7 > self.offset.x + size.x as u64 {
            self.offset.x = (self.cursor.x + 7).saturating_sub(size.x as u64);
        }

        if self.cursor.y + 4 > self.offset.y + size.y as u64 {
            self.offset.y = (self.cursor.y + 4).saturating_sub(size.y as u64);
        }

        if self.cursor.x < self.offset.x + 5 {
            self.offset.x = self.cursor.x.saturating_sub(5);
        }

        if self.cursor.y < self.offset.y + 4 {
            self.offset.y = self.cursor.y.saturating_sub(4);
        }
    }
}
