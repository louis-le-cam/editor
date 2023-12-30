use editor_action::{Action, DocumentAction, SingleLineDocumentAction};
use editor_mode::{Focused, Mode};
use editor_terminal::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

#[derive(Debug)]
pub struct Input {
    key: KeyCode,
    modifier: KeyModifiers,
}

impl Input {
    pub fn new(key: KeyCode, modifier: KeyModifiers) -> Self {
        Self { key, modifier }
    }
}

pub struct Inputs {
    normal: Vec<(Input, Action)>,
    insert: Vec<(Input, Action)>,
    selection: Vec<(Input, Action)>,
    text_box: Vec<(Input, Action)>,
}

impl Inputs {
    pub fn key_event(&self, key_event: &KeyEvent, focused: Focused, mode: Mode) -> Option<Action> {
        if key_event.kind == KeyEventKind::Release {
            return None;
        }

        match focused {
            Focused::Editor => match mode {
                Mode::Normal => self
                    .normal
                    .iter()
                    .filter(|(input, _)| {
                        input.key == key_event.code && input.modifier == key_event.modifiers
                    })
                    .map(|(_, action)| action.clone())
                    .next(),
                Mode::Insert => self
                    .insert
                    .iter()
                    .filter(|(input, _)| {
                        input.key == key_event.code && input.modifier == key_event.modifiers
                    })
                    .map(|(_, action)| action.clone())
                    .next()
                    .or_else(|| match (key_event.modifiers, key_event.code) {
                        (KeyModifiers::NONE | KeyModifiers::SHIFT, KeyCode::Char(char)) => {
                            Some(SingleLineDocumentAction::Insert { char }.into())
                        }
                        _ => None,
                    }),
                Mode::Selection => self
                    .selection
                    .iter()
                    .filter(|(input, _)| {
                        input.key == key_event.code && input.modifier == key_event.modifiers
                    })
                    .map(|(_, action)| action.clone())
                    .next(),
            },
            Focused::CommandBar => self
                .text_box
                .iter()
                .filter(|(input, _)| {
                    input.key == key_event.code && input.modifier == key_event.modifiers
                })
                .map(|(_, action)| action.clone())
                .next()
                .or_else(|| match (key_event.modifiers, key_event.code) {
                    (KeyModifiers::NONE | KeyModifiers::SHIFT, KeyCode::Char(ch)) => {
                        Some(SingleLineDocumentAction::Insert { char: ch }.into())
                    }
                    _ => None,
                }),
        }
    }
}

impl Default for Inputs {
    fn default() -> Self {
        macro_rules! keybinds {
            ($(($key:expr, $modifiers:ident, $action:expr),)*) => {
                {
                    use editor_terminal::{KeyCode::*, KeyModifiers};

                    vec![
                        $((Input::new($key, KeyModifiers::$modifiers), Into::<Action>::into($action)),)*
                    ]
                }
            };
        }

        let normal = keybinds!(
            (Left, NONE, SingleLineDocumentAction::MoveLeft),
            (Right, NONE, SingleLineDocumentAction::MoveRight),
            (Up, NONE, DocumentAction::MoveUp),
            (Down, NONE, DocumentAction::MoveDown),
            (Char('h'), NONE, SingleLineDocumentAction::MoveLeft),
            (Char('l'), NONE, SingleLineDocumentAction::MoveRight),
            (Char('k'), NONE, DocumentAction::MoveUp),
            (Char('j'), NONE, DocumentAction::MoveDown),
            (Char('i'), NONE, Action::EnterInsertMode),
            (Char('v'), NONE, Action::EnterSelectionMode),
            (Char('s'), CONTROL, DocumentAction::Write),
            (Char(':'), NONE, Action::FocusCommandBar),
        );

        let insert = keybinds!(
            (Char('h'), CONTROL, SingleLineDocumentAction::DeleteBefore),
            (Backspace, NONE, SingleLineDocumentAction::DeleteBefore),
            (Char('j'), CONTROL, DocumentAction::InsertLineBeforeCursor),
            (Enter, CONTROL, DocumentAction::InsertLineBeforeCursor),
            (Esc, NONE, Action::EnterNormalMode),
        );

        let selection = keybinds!(
            (Left, NONE, DocumentAction::ExtendEndLeft),
            (Right, NONE, DocumentAction::ExtendEndRight),
            (Up, NONE, DocumentAction::ExtendEndUp),
            (Down, NONE, DocumentAction::ExtendEndDown),
            (Char('h'), NONE, DocumentAction::ExtendEndLeft),
            (Char('l'), NONE, DocumentAction::ExtendEndRight),
            (Char('k'), NONE, DocumentAction::ExtendEndUp),
            (Char('j'), NONE, DocumentAction::ExtendEndDown),
            (Char('i'), NONE, Action::EnterInsertMode),
            (Esc, NONE, Action::EnterNormalMode),
            (Char(':'), NONE, Action::FocusCommandBar),
        );

        let text_box = keybinds!(
            (Char('j'), CONTROL, Action::Validate),
            (Enter, NONE, Action::Validate),
            (Esc, NONE, Action::Cancel),
            (Left, NONE, SingleLineDocumentAction::MoveLeft),
            (Right, NONE, SingleLineDocumentAction::MoveRight),
            (Char('h'), CONTROL, SingleLineDocumentAction::DeleteBefore),
            (Backspace, NONE, SingleLineDocumentAction::DeleteBefore),
        );

        Self {
            normal,
            insert,
            selection,
            text_box,
        }
    }
}
