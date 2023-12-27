use editor_action::{Action, Command, DocumentAction};
use editor_mode::Mode;
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
}

impl Inputs {
    pub fn key_event(&self, key_event: &KeyEvent, mode: &Mode) -> Option<Action> {
        if key_event.kind == KeyEventKind::Release {
            return None;
        }

        match mode {
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
                    (KeyModifiers::NONE | KeyModifiers::SHIFT, KeyCode::Char(ch)) => {
                        Some(DocumentAction::Insert(ch).into())
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
            (Left, NONE, DocumentAction::MoveLeft),
            (Right, NONE, DocumentAction::MoveRight),
            (Up, NONE, DocumentAction::MoveUp),
            (Down, NONE, DocumentAction::MoveDown),
            (Char('h'), NONE, DocumentAction::MoveLeft),
            (Char('l'), NONE, DocumentAction::MoveRight),
            (Char('k'), NONE, DocumentAction::MoveUp),
            (Char('j'), NONE, DocumentAction::MoveDown),
            (Char('i'), NONE, Command::EnterInsertMode),
            (Char('s'), CONTROL, DocumentAction::Write),
            (Char('c'), CONTROL, Command::Quit),
        );

        let insert = keybinds!(
            (Char('h'), CONTROL, DocumentAction::DeleteBefore),
            (Char('j'), CONTROL, DocumentAction::InsertLineBeforeCursor),
            (Esc, NONE, Command::EnterNormalMode),
        );

        Self { normal, insert }
    }
}
