use editor_action::{Action, SingleLineDocumentAction};
use editor_document::SingleLineDocument;
use editor_terminal::{TermRect, TermSlice};
use editor_theme::Theme;

pub struct CommandBar {
    document: SingleLineDocument,
}

impl CommandBar {
    pub fn new() -> Self {
        Self {
            document: SingleLineDocument::new(),
        }
    }

    pub fn draw(&mut self, theme: &Theme, mut term: TermSlice) {
        self.draw_suggestions(
            theme,
            term.slice(TermRect::new(
                (0, 0),
                (term.rect().width(), term.rect().heigth().saturating_sub(1)),
            )),
        );

        self.draw_line(
            theme,
            term.slice(TermRect::new(
                (0, term.rect().heigth().saturating_sub(1)),
                (term.rect().width(), 1),
            )),
        );
    }

    fn draw_suggestions(&self, theme: &Theme, mut term: TermSlice) {
        if self.document.line().chars().any(|ch| ch.is_whitespace()) {
            return;
        }

        term.set_background_color(theme.command_suggestion_background);
        term.set_text_color(theme.command_suggestion_text);

        let suggestions = Action::fuzzy_ordered(self.document.line());

        for y in 0..term.rect().heigth() {
            term.write_to(
                (0, y),
                &suggestions[y as usize]
                    .chars()
                    .chain(std::iter::repeat(' '))
                    .take(term.rect().width() as usize)
                    .collect::<String>(),
            );
        }
    }

    fn draw_line(&self, theme: &Theme, mut term: TermSlice) {
        term.set_background_color(theme.command_bar_background);

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
    }

    pub fn validate(&mut self) -> Option<Action> {
        let action = Action::parse(&self.document.line())
            .and_then(|action| action.is_public().then_some(action));
        self.document.clear();
        action
    }

    pub fn cancel(&mut self) {
        self.document.clear();
    }

    pub fn handle_action(&mut self, document_action: SingleLineDocumentAction) {
        self.document.handle_action(document_action);
    }
}
