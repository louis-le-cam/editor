use editor_action::{Command, DocumentAction};
use editor_document::SingleLineDocument;
use editor_mode::Focused;
use editor_terminal::TermSlice;
use editor_theme::Theme;
use log::warn;

pub struct CommandBar {
    document: SingleLineDocument,
}

impl CommandBar {
    pub fn new() -> Self {
        Self {
            document: SingleLineDocument::new(),
        }
    }

    pub fn draw(&mut self, theme: &Theme, focused: Focused, mut term: TermSlice) {
        term.set_background_color(theme.command_bar_background);

        if focused == Focused::CommandBar {
            let text = format!(
                ":{:<width$}",
                self.document.line(),
                width = (term.rect().width() as usize).saturating_sub(1)
            );

            let cursor_index = text
                .char_indices()
                .nth(self.document.cursor() + 1)
                .map(|(i, _)| i)
                .unwrap_or(self.document.line().len());

            let (before_cursor, after) = text.split_at(cursor_index);
            let before_cursor_count = before_cursor.chars().count() as u16;
            let (at_cursor, after_cursor) = after.split_at(
                after
                    .char_indices()
                    .nth(1)
                    .map(|(i, _)| i)
                    .unwrap_or(after.len()),
            );

            term.set_text_color(theme.command_bar_text);
            term.write_to((0, 0), before_cursor);

            if at_cursor.len() == 0 {
                return;
            }

            term.set_background_color(theme.cursor);
            term.set_text_color(theme.command_bar_background);
            term.write_to((before_cursor_count, 0), at_cursor);

            if after_cursor.len() == 0 {
                return;
            }

            term.set_background_color(theme.command_bar_background);
            term.set_text_color(theme.command_bar_text);
            term.write_to((before_cursor_count + 1, 0), after_cursor);
        } else {
            term.write_to((0, 0), &" ".repeat(term.rect().width() as usize));
        }
    }

    pub fn validate(&mut self) -> Option<Command> {
        let command = Command::from_str(&self.document.line());
        self.document.clear();
        command
    }

    pub fn cancel(&mut self) {
        self.document.clear();
    }

    pub(crate) fn execute(
        &mut self,
        theme: &Theme,
        focused: Focused,
        term: TermSlice,
        document_action: &DocumentAction,
    ) {
        document_action.execute(&mut self.document);
        self.draw(theme, focused, term);
    }
}
