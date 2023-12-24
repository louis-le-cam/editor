use editor_terminal::{Event, KeyCode, KeyModifiers, Term, TermRect};

use crate::text_editor::TextEditor;

pub struct App {
    term: Term,
    text_editor: TextEditor,
}

impl App {
    pub fn new() -> Self {
        Self {
            term: Term::new(),
            text_editor: TextEditor::from_path("log.txt"),
        }
    }

    pub fn run(mut self) {
        let term_size = self.term.size();
        self.text_editor
            .draw(self.term.slice(TermRect::new((0, 0), term_size)));
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
                    self.text_editor
                        .draw(self.term.slice(TermRect::new((0, 0), term_size)));
                }
                _ => {}
            }

            self.text_editor
                .event(self.term.slice(TermRect::new((0, 0), term_size)), &event);
            self.term.flush();
        }
    }
}
