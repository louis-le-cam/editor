mod editor;

use editor::Editor;
use editor_terminal::{Event, KeyCode, KeyModifiers, Term, TermRect};
use editor_theme::Theme;

pub struct App {
    term: Term,
    theme: Theme,
    editor: Editor,
}

impl App {
    pub fn new() -> Self {
        Self {
            term: Term::new(),
            theme: Theme::default(),
            editor: Editor::from_path("log.txt".into()),
        }
    }

    pub fn run(mut self) {
        let term_size = self.term.size();
        self.editor.draw(
            &self.theme,
            self.term.slice(TermRect::new((0, 0), term_size)),
        );
        self.term.flush();

        while let Ok(event) = self.term.wait_for_event() {
            let term_size = self.term.size();

            match event {
                Event::Key(key) => {
                    if key.code == KeyCode::Char('c')
                        && key.modifiers.contains(KeyModifiers::CONTROL)
                    {
                        break;
                    }
                }
                Event::Resize(_, _) => {
                    self.editor.draw(
                        &self.theme,
                        self.term.slice(TermRect::new((0, 0), term_size)),
                    );
                }
                _ => {}
            }

            self.editor.event(
                &self.theme,
                self.term.slice(TermRect::new((0, 0), term_size)),
                &event,
            );
            self.term.flush();
        }
    }
}
