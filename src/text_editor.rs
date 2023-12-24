use std::{
    fs::File,
    io::{self, BufRead},
    path::{Path, PathBuf},
};

use editor_terminal::{Color, Event, KeyCode, KeyEventKind, TermRect, TermSlice, TermVec};
use glam::{u16vec2, U64Vec2};

use crate::{mode::Mode, theme::Theme};

pub struct TextEditor {
    path: PathBuf,
    lines: Vec<String>,
    dirty: bool,
    mode: Mode,
    cursor: U64Vec2,
    offset: U64Vec2,
}
impl TextEditor {
    pub fn from_path(path: impl AsRef<Path>) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
            lines: io::BufReader::new(File::open(path).unwrap())
                .lines()
                .collect::<Result<_, _>>()
                .unwrap(),
            dirty: false,
            mode: Mode::Normal,
            cursor: (0, 0).into(),
            offset: (0, 0).into(),
        }
    }

    pub fn draw(&mut self, theme: &Theme, mut term: TermSlice) {
        let gutter_width = (number_width(self.lines.len() as u64) + 2) as u16;

        self.draw_gutter(
            theme,
            term.slice(TermRect::new(
                (0, 0),
                (gutter_width, term.rect().heigth().saturating_sub(1)),
            )),
        );

        self.draw_infos(
            theme,
            term.slice(TermRect::new(
                (0, term.rect().heigth().saturating_sub(1)),
                (term.rect().width(), 1),
            )),
        );

        self.draw_code(
            theme,
            term.slice(TermRect::new(
                (gutter_width, 0),
                term.rect().size.saturating_sub(u16vec2(gutter_width, 1)),
            )),
        );
    }

    fn draw_gutter(&mut self, theme: &Theme, mut term: TermSlice) {
        let size = term.rect().size;

        for y in 0..size.y {
            let line = self.offset.y + y as u64;

            if line == self.cursor.y {
                term.set_text_color(theme.gutter_current_line);
            } else {
                term.set_text_color(theme.gutter_line);
            }

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

            term.set_background_color(theme.gutter_background);
            term.write_to((0, y), &line_number);
            term.reset_text_color();
            term.reset_background_color();
        }
    }

    fn draw_infos(&mut self, theme: &Theme, mut term: TermSlice) {
        let mode_abreviation = self.mode.abreviation();

        let path = self.path.display().to_string();

        term.set_background_color(theme.code_info_background);
        term.set_text_color(theme.code_info_text);

        term.write_to(
            (0, 0),
            &format!(
                " {} {} {} {:>width$}:{} ",
                mode_abreviation,
                path,
                match self.dirty {
                    true => "[+]",
                    false => "   ",
                },
                self.cursor.y + 1,
                self.cursor.x + 1,
                width = (term.rect().width() as usize).saturating_sub(
                    mode_abreviation.len()
                        + path.len()
                        + number_width(self.cursor.x + 1) as usize
                        + 8
                )
            ),
        );

        term.reset_background_color();
        term.reset_text_color();
    }

    fn draw_code(&mut self, theme: &Theme, mut term: TermSlice) {
        let size = term.rect().size;

        term.set_background_color(theme.code_background);
        term.set_text_color(theme.code_text);

        let true_cursor_x = self.true_cursor_x();

        for y in 0..size.y {
            let line = self
                .lines
                .get(y as usize + self.offset.y as usize)
                .map(String::as_str)
                .unwrap_or("");

            let mut chars = line
                .chars()
                .skip(self.offset.x as usize)
                .chain(" ".chars().cycle());

            if y as i64 == self.cursor.y as i64 - self.offset.y as i64 {
                let before_cursor_len = true_cursor_x.saturating_sub(self.offset.x) as usize;

                if true_cursor_x >= self.offset.x {
                    let before_cursor = (0..before_cursor_len)
                        .filter_map(|_| chars.next())
                        .collect::<String>();

                    term.write_to((0, y), &before_cursor);

                    let at_cursor = chars.next().map(|ch| ch.to_string()).unwrap();

                    term.set_background_color(theme.cursor);
                    term.set_text_color(Color::Black);
                    term.write_to((before_cursor_len as u16, y), &at_cursor);
                    term.set_background_color(theme.code_background);
                    term.set_text_color(theme.code_text);

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

    pub fn event(&mut self, theme: &Theme, term: TermSlice, event: &Event) {
        let mut need_redraw = false;

        match event {
            Event::Key(key) => match self.mode {
                Mode::Normal => {
                    if matches!(key.kind, KeyEventKind::Press | KeyEventKind::Repeat) {
                        need_redraw = true;

                        match key.code {
                            KeyCode::Char('h') | KeyCode::Left => {
                                if self.cursor.x == 0 {
                                    if self.cursor.y != 0 {
                                        self.cursor.y -= 1;
                                        self.cursor.x = self
                                            .lines
                                            .get(self.cursor.y as usize)
                                            .map(String::len)
                                            .unwrap_or(0)
                                            as u64;
                                    }
                                } else {
                                    self.cursor.x -= 1;
                                }
                            }
                            KeyCode::Char('j') | KeyCode::Down => {
                                if self.cursor.y >= self.lines.len() as u64 {
                                    self.cursor.y = self.lines.len() as u64;
                                } else {
                                    self.cursor.y = self.cursor.y.saturating_add(1);
                                }
                            }
                            KeyCode::Char('k') | KeyCode::Up => {
                                self.cursor.y = self.cursor.y.saturating_sub(1)
                            }
                            KeyCode::Char('l') | KeyCode::Right => {
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
                            KeyCode::Char('i') => self.mode = Mode::Insert,
                            _ => need_redraw = false,
                        }

                        if need_redraw {
                            let gutter_width =
                                (self.lines.len().checked_ilog10().unwrap_or(0) + 3) as u16;

                            self.update_offset(
                                term.rect().size.saturating_sub(u16vec2(gutter_width, 0)),
                            );
                        }
                    }
                }
                Mode::Insert => {
                    if matches!(key.kind, KeyEventKind::Press | KeyEventKind::Repeat) {
                        need_redraw = true;

                        match key.code {
                            KeyCode::Char(ch) => {
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
                            KeyCode::Esc => {
                                self.mode = Mode::Normal;
                            }
                            _ => need_redraw = false,
                        }
                    }
                }
            },
            _ => {}
        }

        if need_redraw {
            self.draw(theme, term);
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

    fn true_cursor_x(&self) -> u64 {
        self.lines
            .get(self.cursor.y as usize)
            .map(|line| self.cursor.x.min(line.len() as u64))
            .unwrap_or(0)
    }
}

fn number_width(number: u64) -> u32 {
    number.checked_ilog10().unwrap_or(0) + 1
}
