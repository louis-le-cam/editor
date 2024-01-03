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
            editor: Editor::new_scratch(),
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
        self.editor
            .draw(&self.theme, self.term.slice(self.editor_rect()), self.mode);
        if self.focused == Focused::CommandBar {
            self.command_bar
                .draw(&self.theme, self.term.slice(self.command_bar_rect()));
        }
    }

    fn editor_rect(&self) -> TermRect {
        if self.focused == Focused::CommandBar {
            TermRect::new((0, 0), self.term.size().saturating_sub(u16vec2(0, 1)))
        } else {
            TermRect::new((0, 0), self.term.size())
        }
    }

    fn command_bar_rect(&self) -> TermRect {
        let y = self.term.size().y.saturating_sub(6);

        TermRect::new(
            (0, y),
            u16vec2(self.term.size().x, self.term.size().y.saturating_sub(y)),
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
                Focused::Editor => {
                    self.editor.handle_action(action);
                    self.draw();
                }
                Focused::CommandBar => match action {
                    SingleLine(action) => {
                        self.command_bar.handle_action(action);
                        self.draw();
                    }
                    action => {
                        warn!("Unexpected multiline document action ({:?}) used while command bar focused (ignored)", action);
                    }
                },
            },
            Quit => self.should_quit = true,
            Open { path } => {
                self.editor = Editor::from_path(path.into());
                self.draw();
            }
            Redraw => self.draw(),
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
                self.draw();
            }
            EnterInsertMode => {
                self.mode = Mode::Insert;
                self.draw();
            }
            EnterSelectionMode => {
                self.mode = Mode::Selection;
                self.draw();
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
