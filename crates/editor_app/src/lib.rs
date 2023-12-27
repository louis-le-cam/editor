mod editor;

use editor_action::{Action, CommandHandler};
use editor_input::Inputs;
use editor_mode::Mode;
use editor_terminal::{Event, Term, TermRect};
use editor_theme::Theme;

use crate::editor::Editor;

pub struct App {
    should_quit: bool,
    mode: Mode,
    term: Term,
    theme: Theme,
    inputs: Inputs,
    editor: Editor,
}

impl App {
    pub fn new() -> Self {
        Self {
            should_quit: false,
            mode: Mode::Normal,
            term: Term::new(),
            theme: Theme::default(),
            inputs: Inputs::default(),
            editor: Editor::from_path("log.txt".into()),
        }
    }

    pub fn run(mut self) {
        self.draw_editor();
        self.term.flush();

        while !self.should_quit {
            if let Ok(event) = self.term.wait_for_event() {
                self.handle_event(&event);

                self.term.flush();
            };
        }
    }

    fn draw_editor(&mut self) {
        let term_size = self.term.size();
        self.editor.draw(
            &self.theme,
            self.term.slice(TermRect::new((0, 0), term_size)),
            &self.mode,
        );
    }

    fn handle_event(&mut self, event: &Event) {
        match event {
            Event::Key(key_event) => self.execute(&self.inputs.key_event(&key_event, &self.mode)),
            Event::Resize(_, _) => self.draw_editor(),
            _ => {}
        }
    }

    fn execute(&mut self, action: &Option<Action>) {
        match action {
            Some(Action::Command(command)) => command.execute(self),
            Some(Action::Document(document_action)) => {
                let term_size = self.term.size();

                self.editor.execute(
                    &self.theme,
                    self.term.slice(TermRect::new((0, 0), term_size)),
                    &self.mode,
                    &document_action,
                )
            }
            None => {}
        }
    }
}

impl CommandHandler for App {
    fn quit(&mut self) {
        self.should_quit = true;
    }

    fn enter_insert_mode(&mut self) {
        self.mode = Mode::Insert;
        self.draw_editor();
    }

    fn enter_normal_mode(&mut self) {
        self.mode = Mode::Normal;
        self.draw_editor();
    }
}
