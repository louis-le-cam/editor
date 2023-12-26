mod editor;

use editor_action::CommandHandler;
use editor_terminal::{Event, KeyCode, KeyModifiers, Term, TermRect};
use editor_theme::Theme;

use crate::editor::Editor;

pub struct App {
    should_quit: bool,
    term: Term,
    theme: Theme,
    editor: Editor,
}

impl App {
    pub fn new() -> Self {
        Self {
            should_quit: false,
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

        while !self.should_quit {
            let Ok(event) = self.term.wait_for_event() else {
                continue;
            };

            let term_size = self.term.size();

            match event {
                Event::Key(key) => {
                    if key.code == KeyCode::Char('c')
                        && key.modifiers.contains(KeyModifiers::CONTROL)
                    {
                        self.quit();
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

impl CommandHandler for App {
    fn quit(&mut self) {
        self.should_quit = true;
    }
}
