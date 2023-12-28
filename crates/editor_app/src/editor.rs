use std::path::PathBuf;

use editor_action::DocumentAction;
use editor_document::Document;
use editor_mode::Mode;
use editor_terminal::{Color, TermRect, TermSlice, TermVec};
use editor_theme::Theme;
use glam::u16vec2;

pub struct Editor {
    document: Document,
    offset: (usize, usize),
}

impl Editor {
    pub fn from_path(path: PathBuf) -> Self {
        Self {
            document: Document::from_path(path),
            offset: (0, 0).into(),
        }
    }

    pub fn draw(&mut self, theme: &Theme, mut term: TermSlice, mode: &Mode) {
        self.update_offset(mode, term.rect().size);

        let gutter_width = (number_width(self.document.lines().len()) + 2) as u16;

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

        let (start, _) = self.document.selection();

        for y in 0..size.y {
            let line = self.offset.1 + y as usize;

            if line == start.1 {
                term.set_text_color(theme.gutter_current_line);
            } else {
                term.set_text_color(theme.gutter_line);
            }

            let line_number = match line {
                _ if line < self.document.lines().len() => {
                    format!(
                        " {: >width$} ",
                        line + 1,
                        width = size.x.saturating_sub(2) as usize
                    )
                }

                _ if line == self.document.lines().len() => {
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
        }
    }

    fn draw_infos(&mut self, theme: &Theme, mut term: TermSlice, mode: &Mode) {
        let mode_abreviation = mode.abreviation();

        let path = self.document.path().display().to_string();
        let (start, _) = self.document.selection();

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
                start.1 + 1,
                start.0 + 1,
                width = (term.rect().width() as usize).saturating_sub(
                    mode_abreviation.len() + path.len() + number_width(start.0 + 1) + 9
                )
            ),
        );
    }

    fn draw_code(&mut self, theme: &Theme, mut term: TermSlice) {
        let size = term.rect().size;

        let (start, end) = self.document.selection();

        term.set_background_color(theme.code_background);
        term.set_text_color(theme.code_text);

        for y in 0..size.y {
            let line = self
                .document
                .lines()
                .get(y as usize + self.offset.1)
                .map(String::as_str)
                .unwrap_or("");

            let mut chars = line
                .chars()
                .skip(self.offset.0 as usize)
                .chain(" ".chars().cycle());

            let offseted_y = y as usize + self.offset.1;

            if (start.1.min(end.1)..start.1.max(end.1) + 1).contains(&offseted_y) {
                let before_selection_len = if offseted_y == start.1 {
                    start.0.saturating_sub(self.offset.0) as usize
                } else {
                    0
                };

                let under_selection_len = if offseted_y == end.1 {
                    (size.x as usize).saturating_sub(
                        before_selection_len
                            + ((size.x as usize)
                                .saturating_sub(end.0.saturating_sub(self.offset.0) + 1)),
                    )
                } else {
                    (size.x as usize).saturating_sub(before_selection_len)
                };

                let after_selection_len =
                    (size.x as usize).saturating_sub(before_selection_len + under_selection_len);

                if before_selection_len != 0 {
                    term.write_to(
                        (0, y),
                        &(0..before_selection_len)
                            .map(|_| chars.next().unwrap_or(' '))
                            .collect::<String>(),
                    );
                }

                if under_selection_len != 0 {
                    term.set_background_color(theme.cursor);
                    term.set_text_color(Color::Black);
                    term.write_to(
                        (before_selection_len as u16, y),
                        &(0..under_selection_len)
                            .map(|_| chars.next().unwrap_or(' '))
                            .collect::<String>(),
                    );
                    term.set_background_color(theme.code_background);
                    term.set_text_color(theme.code_text);
                }

                if after_selection_len != 0 {
                    term.write_to(
                        ((before_selection_len + under_selection_len) as u16, y),
                        &(0..after_selection_len)
                            .map(|_| chars.next().unwrap_or(' '))
                            .collect::<String>(),
                    );
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
        self.draw(theme, term, mode);
    }

    /// Update `self.offset` if `self.document.cursor()` is near edges
    fn update_offset(&mut self, mode: &Mode, size: TermVec) {
        let cursor = match mode {
            Mode::Normal | Mode::Selection => self.document.selection().1,
            Mode::Insert => self.document.selection().0,
        };

        if cursor.0 + 7 > self.offset.0 + size.x as usize {
            self.offset.0 = (cursor.0 + 7).saturating_sub(size.x as usize);
        }

        if cursor.1 + 4 > self.offset.1 + size.y as usize {
            self.offset.1 = (cursor.1 + 4).saturating_sub(size.y as usize);
        }

        if cursor.0 < self.offset.0 + 5 {
            self.offset.0 = cursor.0.saturating_sub(5);
        }

        if cursor.1 < self.offset.1 + 4 {
            self.offset.1 = cursor.1.saturating_sub(4);
        }
    }
}

fn number_width(number: usize) -> usize {
    number.checked_ilog10().unwrap_or(0) as usize + 1
}
