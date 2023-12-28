mod command_bar;
mod editor;

use command_bar::CommandBar;
use editor_action::{Action, Command, CommandHandler};
use editor_input::Inputs;
use editor_mode::{Focused, Mode};
use editor_terminal::{Event, Term, TermRect};
use editor_theme::Theme;
use glam::u16vec2;
use log::warn;

use crate::editor::Editor;

pub struct App {
    should_quit: bool,
    mode: Mode,
    focused: Focused,
    term: Term,
    theme: Theme,
    inputs: Inputs,
    editor: Editor,
    command_bar: CommandBar,
}

impl App {
    pub fn new() -> Self {
        Self {
            should_quit: false,
            mode: Mode::Normal,
            focused: Focused::Editor,
            term: Term::new(),
            theme: Theme::default(),
            inputs: Inputs::default(),
            editor: Editor::from_path("log.txt".into()),
            command_bar: CommandBar::new(),
        }
    }

    pub fn run(mut self) {
        self.draw();
        self.term.flush();

        while !self.should_quit {
            if let Ok(event) = self.term.wait_for_event() {
                self.handle_event(&event);

                self.term.flush();
            };
        }
    }

    fn draw(&mut self) {
        self.draw_editor();
        self.draw_command_bar();
    }

    fn draw_editor(&mut self) {
        self.editor
            .draw(&self.theme, self.term.slice(self.editor_rect()), &self.mode);
    }

    fn draw_command_bar(&mut self) {
        self.command_bar.draw(
            &self.theme,
            self.focused,
            self.term.slice(self.command_bar_rect()),
        );
    }

    fn editor_rect(&self) -> TermRect {
        TermRect::new((0, 0), self.term.size().saturating_sub(u16vec2(0, 1)))
    }

    fn command_bar_rect(&self) -> TermRect {
        TermRect::new(
            (0, self.term.size().y.saturating_sub(1)),
            u16vec2(self.term.size().x, 1),
        )
    }

    fn handle_event(&mut self, event: &Event) {
        match event {
            Event::Key(key_event) => {
                self.execute(&self.inputs.key_event(&key_event, &self.focused, &self.mode))
            }
            Event::Resize(_, _) => self.draw(),
            _ => {}
        }
    }

    fn execute(&mut self, action: &Option<Action>) {
        match action {
            Some(Action::Command(command)) => command.execute(self),
            Some(Action::Document(document_action)) => match self.focused {
                Focused::Editor => self.editor.execute(
                    &self.theme,
                    self.term.slice(self.editor_rect()),
                    &self.mode,
                    &document_action,
                ),
                Focused::CommandBar => self.command_bar.execute(
                    &self.theme,
                    self.focused,
                    self.term.slice(self.command_bar_rect()),
                    document_action,
                ),
            },
            None => {}
        }
    }
}

impl CommandHandler for App {
    fn quit(&mut self) {
        self.should_quit = true;
    }

    fn enter_normal_mode(&mut self) {
        self.mode = Mode::Normal;
        self.draw_editor();
    }

    fn enter_insert_mode(&mut self) {
        self.mode = Mode::Insert;
        self.draw_editor();
    }

    fn enter_selection_mode(&mut self) {
        self.mode = Mode::Selection;
        self.draw_editor();
    }

    fn focus_editor(&mut self) {
        self.focused = Focused::Editor;
        self.draw();
    }

    fn focus_command_bar(&mut self) {
        self.focused = Focused::CommandBar;
        self.draw();
    }

    fn validate(&mut self) {
        match self.focused {
            Focused::Editor => {
                warn!("Validate command does nothing when editor is focused")
            }
            Focused::CommandBar => {
                self.execute(&Some(Command::FocusEditor.into()));
                let command = self.command_bar.validate();
                self.execute(&command.map(Into::into));
            }
        }
    }

    fn cancel(&mut self) {
        match self.focused {
            Focused::Editor => {
                warn!("Cancel command does nothing when editor is focused")
            }
            Focused::CommandBar => {
                self.execute(&Some(Command::FocusEditor.into()));
                self.command_bar.cancel();
            }
        }
    }
}
