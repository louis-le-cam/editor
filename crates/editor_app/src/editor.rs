use std::path::PathBuf;

use editor_action::DocumentAction;
use editor_document::Document;
use editor_mode::Mode;
use editor_terminal::{Color, TermRect, TermSlice, TermVec};
use editor_theme::Theme;
use glam::{u16vec2, U64Vec2};

pub struct Editor {
    document: Document,
    offset: U64Vec2,
}

impl Editor {
    pub fn from_path(path: PathBuf) -> Self {
        Self {
            document: Document::from_path(path),
            offset: (0, 0).into(),
        }
    }

    pub fn draw(&mut self, theme: &Theme, mut term: TermSlice, mode: &Mode) {
        let gutter_width = (number_width(self.document.lines().len() as u64) + 2) as u16;

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
            mode,
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

            if line == self.document.cursor().y {
                term.set_text_color(theme.gutter_current_line);
            } else {
                term.set_text_color(theme.gutter_line);
            }

            let line_number = match line {
                _ if line < self.document.lines().len() as u64 => {
                    format!(
                        " {: >width$} ",
                        line + 1,
                        width = size.x.saturating_sub(2) as usize
                    )
                }

                _ if line == self.document.lines().len() as u64 => {
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

    fn draw_infos(&mut self, theme: &Theme, mut term: TermSlice, mode: &Mode) {
        let mode_abreviation = mode.abreviation();

        let path = self.document.path().display().to_string();

        term.set_background_color(theme.code_info_background);
        term.set_text_color(theme.code_info_text);

        term.write_to(
            (0, 0),
            &format!(
                " {} {} {} {:>width$}:{} ",
                mode_abreviation,
                path,
                match self.document.dirty() {
                    true => "[+]",
                    false => "   ",
                },
                self.document.cursor().y + 1,
                self.document.cursor().x + 1,
                width = (term.rect().width() as usize).saturating_sub(
                    mode_abreviation.len()
                        + path.len()
                        + number_width(self.document.cursor().x + 1) as usize
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

        let true_cursor = self.document.true_cursor();

        for y in 0..size.y {
            let line = self
                .document
                .lines()
                .get(y as usize + self.offset.y as usize)
                .map(String::as_str)
                .unwrap_or("");

            let mut chars = line
                .chars()
                .skip(self.offset.x as usize)
                .chain(" ".chars().cycle());

            if y as i64 == true_cursor.y as i64 - self.offset.y as i64 {
                let before_cursor_len = true_cursor.x.saturating_sub(self.offset.x) as usize;

                if true_cursor.x >= self.offset.x {
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

    pub fn execute(
        &mut self,
        theme: &Theme,
        term: TermSlice,
        mode: &Mode,
        action: &DocumentAction,
    ) {
        action.execute(&mut self.document);

        self.update_offset(term.rect().size);
        self.draw(theme, term, mode);
    }

    /// Update `self.offset` if `self.document.cursor()` is near edges
    fn update_offset(&mut self, size: TermVec) {
        if self.document.cursor().x + 7 > self.offset.x + size.x as u64 {
            self.offset.x = (self.document.cursor().x + 7).saturating_sub(size.x as u64);
        }

        if self.document.cursor().y + 4 > self.offset.y + size.y as u64 {
            self.offset.y = (self.document.cursor().y + 4).saturating_sub(size.y as u64);
        }

        if self.document.cursor().x < self.offset.x + 5 {
            self.offset.x = self.document.cursor().x.saturating_sub(5);
        }

        if self.document.cursor().y < self.offset.y + 4 {
            self.offset.y = self.document.cursor().y.saturating_sub(4);
        }
    }
}

fn number_width(number: u64) -> u32 {
    number.checked_ilog10().unwrap_or(0) + 1
}
