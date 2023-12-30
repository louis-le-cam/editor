mod rect;
mod slice;
mod terminal;

pub use crossterm::{
    event::{
        Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers, MediaKeyCode,
        ModifierKeyCode, MouseButton, MouseEvent, MouseEventKind,
    },
    style::Color,
};

pub use crate::{rect::TermRect, slice::TermSlice, terminal::Term};
