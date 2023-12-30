mod command_bar;
mod editor;

use command_bar::CommandBar;
use editor_action::Action;
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
            .draw(&self.theme, self.term.slice(self.editor_rect()), self.mode);
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
                if let Some(action) = self.inputs.key_event(&key_event, self.focused, self.mode) {
                    self.handle_action(action);
                }
            }
            Event::Resize(_, _) => self.draw(),
            _ => {}
        }
    }

    fn handle_action(&mut self, action: Action) {
        use editor_action::{Action::*, DocumentAction::*};

        match action {
            Document(action) => match self.focused {
                Focused::Editor => self.editor.handle_action(
                    &self.theme,
                    self.term.slice(self.editor_rect()),
                    self.mode,
                    action,
                ),
                Focused::CommandBar => match action {
                    SingleLine(action) => self.command_bar.handle_action(
                        &self.theme,
                        self.focused,
                        self.term.slice(self.command_bar_rect()),
                        action,
                    ),
                    action => {
                        warn!("Unexpected multiline document action ({:?}) used while command bar focused (ignored)", action);
                    }
                },
            },
            Quit => self.should_quit = true,
            Validate => match self.focused {
                Focused::Editor => {
                    warn!("Validate command does nothing when editor is focused")
                }
                Focused::CommandBar => {
                    self.handle_action(Action::FocusEditor.into());
                    if let Some(command) = self.command_bar.validate() {
                        self.handle_action(command);
                    }
                }
            },
            Cancel => match self.focused {
                Focused::Editor => {
                    warn!("Cancel command does nothing when editor is focused")
                }
                Focused::CommandBar => {
                    self.handle_action(Action::FocusEditor.into());
                    self.command_bar.cancel();
                }
            },
            EnterNormalMode => {
                self.mode = Mode::Normal;
                self.draw_editor();
            }
            EnterInsertMode => {
                self.mode = Mode::Insert;
                self.draw_editor();
            }
            EnterSelectionMode => {
                self.mode = Mode::Selection;
                self.draw_editor();
            }
            FocusCommandBar => {
                self.focused = Focused::CommandBar;
                self.draw();
            }
            FocusEditor => {
                self.focused = Focused::Editor;
                self.draw();
            }
        }
    }
}
