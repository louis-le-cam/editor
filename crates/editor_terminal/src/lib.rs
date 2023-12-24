mod rect;
mod slice;
mod terminal;
mod vector;

pub use crossterm::{
    event::{
        Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers, MediaKeyCode,
        ModifierKeyCode, MouseButton, MouseEvent, MouseEventKind,
    },
    style::Color,
};

pub use crate::{rect::TermRect, slice::TermSlice, terminal::Term, vector::TermVec};
