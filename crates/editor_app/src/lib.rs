mod editor;

use editor_action::{Action, CommandHandler};
use editor_input::Inputs;
use editor_mode::Mode;
use editor_terminal::{Event, Term, TermRect};
use editor_theme::Theme;

use crate::editor::Editor;

pub struct App {
    should_quit: bool,
    term: Term,
    theme: Theme,
    inputs: Inputs,
    editor: Editor,
}

impl App {
    pub fn new() -> Self {
        Self {
            should_quit: false,
            term: Term::new(),
            theme: Theme::default(),
            inputs: Inputs::default(),
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
                Event::Key(key_event) => {
                    match self.inputs.key_event(&key_event, &self.editor.mode) {
                        Some(Action::Command(command)) => command.execute(&mut self),
                        Some(Action::Document(document_action)) => self.editor.execute(
                            &self.theme,
                            self.term.slice(TermRect::new((0, 0), term_size)),
                            &document_action,
                        ),
                        None => {}
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

            self.term.flush();
        }
    }
}

impl CommandHandler for App {
    fn quit(&mut self) {
        self.should_quit = true;
    }

    fn enter_insert_mode(&mut self) {
        self.editor.mode = Mode::Insert;
        let term_size = self.term.size();
        self.editor.draw(
            &self.theme,
            self.term.slice(TermRect::new((0, 0), term_size)),
        );
    }

    fn enter_normal_mode(&mut self) {
        self.editor.mode = Mode::Normal;
        let term_size = self.term.size();
        self.editor.draw(
            &self.theme,
            self.term.slice(TermRect::new((0, 0), term_size)),
        );
    }
}
