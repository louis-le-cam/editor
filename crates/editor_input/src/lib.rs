use editor_mode::Mode;
use editor_terminal::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

pub enum Input {
    Nothing,
    MoveLeft,
    MoveRight,
    MoveUp,
    MoveDown,
    InsertChar(char),
    DeleteBefore,
    SetMode(Mode),
}
impl Input {
    pub fn from_key(key_event: &KeyEvent, mode: &Mode) -> Self {
        match mode {
            Mode::Normal => {
                if matches!(key_event.kind, KeyEventKind::Press | KeyEventKind::Repeat) {
                    match key_event.code {
                        KeyCode::Char('h') | KeyCode::Left => Self::MoveLeft,
                        KeyCode::Char('l') | KeyCode::Right => Self::MoveRight,
                        KeyCode::Char('k') | KeyCode::Up => Self::MoveUp,
                        KeyCode::Char('j') | KeyCode::Down => Self::MoveDown,
                        KeyCode::Char('i') => Self::SetMode(Mode::Insert),
                        _ => Self::Nothing,
                    }
                } else {
                    Self::Nothing
                }
            }
            Mode::Insert => {
                if matches!(key_event.kind, KeyEventKind::Press | KeyEventKind::Repeat) {
                    match key_event.code {
                        KeyCode::Char('h')
                            if key_event.modifiers.contains(KeyModifiers::CONTROL) =>
                        {
                            Input::DeleteBefore
                        }
                        KeyCode::Char(ch) => Input::InsertChar(ch),
                        KeyCode::Esc => Input::SetMode(Mode::Normal),
                        _ => Input::Nothing,
                    }
                } else {
                    Input::Nothing
                }
            }
        }
    }
}
