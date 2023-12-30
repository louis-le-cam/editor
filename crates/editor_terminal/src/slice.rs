use std::io::{stdout, Stdout, Write};

use crossterm::{
    cursor::MoveTo,
    style::{Color, SetBackgroundColor, SetForegroundColor, SetUnderlineColor},
    QueueableCommand,
};
use glam::U16Vec2;
use log::error;

use crate::TermRect;

/// Represent a rectangular region of terminal
/// Writing to this slice will be offseted by the position of the rect and restricted by its size
pub struct TermSlice {
    stdout: Stdout,
    rect: TermRect,
}
impl TermSlice {
    #[must_use]
    pub(crate) fn new(rect: TermRect) -> Self {
        Self {
            stdout: stdout(),
            rect,
        }
    }

    #[must_use]
    /// Create a new slice from this one
    /// `rect` is relative to the slice rect
    pub fn slice(&mut self, mut rect: TermRect) -> Self {
        rect.pos += self.rect.pos;
        rect.size = rect.size.min(self.rect.size.saturating_sub(rect.pos));

        Self {
            stdout: stdout(),
            rect,
        }
    }

    #[must_use]
    /// Get the rect restricting the [`TermSlice`]
    pub fn rect(&self) -> TermRect {
        self.rect
    }

    /// Write the string to the given position position in the terminal (offseted by [`TermSlice::rect`])
    /// The action will only take effect after flushing the terminal, see [`Term::flush`]
    pub fn write_to(&mut self, pos: impl Into<U16Vec2>, str: &str) {
        let pos: U16Vec2 = pos.into();

        if pos.y >= self.rect.heigth() {
            return;
        }

        self.move_to(self.rect.pos + pos);
        self.write(
            &str.chars()
                .take(self.rect.width().saturating_sub(pos.x) as usize)
                .collect::<String>(),
        );
    }

    /// Move the terminal cursor to the specified position
    /// The action will only take effect after flushing the terminal, see [`Term::flush`]
    fn move_to(&mut self, pos: U16Vec2) {
        let pos: U16Vec2 = pos.into();
        if let Err(err) = self.stdout.queue(MoveTo(pos.x, pos.y)) {
            error!("Failed to move terminal cursor: {:?}", err);
        }
    }

    /// Write the string to the current terminal cursor position
    /// The action will only take effect after flushing the terminal, see [`Term::flush`]
    fn write(&mut self, str: &str) {
        if let Err(err) = self.stdout.write_all(str.as_bytes()) {
            error!("Failed to write to terminal, {:?}", err);
        }
    }
}

impl TermSlice {
    /// Set the text color the next write, see [`Term::write_to`]
    pub fn set_text_color(&mut self, color: Color) {
        if let Err(err) = self.stdout.queue(SetForegroundColor(color)) {
            error!("Failed to set text color, {:?}", err);
        }
    }
    /// Set the background color the next write, see [`Term::write_to`]
    pub fn set_background_color(&mut self, color: Color) {
        if let Err(err) = self.stdout.queue(SetBackgroundColor(color)) {
            error!("Failed to set background color, {:?}", err);
        }
    }
    /// Set the underline color the next write, see [`Term::write_to`]
    pub fn set_underline_color(&mut self, color: Color) {
        if let Err(err) = self.stdout.queue(SetUnderlineColor(color)) {
            error!("Failed to set background color, {:?}", err);
        }
    }

    /// Reset the text color to the default one for the next write, see [`Term::write_to`]
    pub fn reset_text_color(&mut self) {
        self.set_text_color(Color::Reset);
    }
    /// Reset the background color to the default one for the next write, see [`Term::write_to`]
    pub fn reset_background_color(&mut self) {
        self.set_background_color(Color::Reset);
    }
    /// Reset the underline color to the default one for the next write, see [`Term::write_to`]
    pub fn reset_underline_color(&mut self) {
        self.set_underline_color(Color::Reset);
    }
}

impl Clone for TermSlice {
    fn clone(&self) -> Self {
        Self {
            stdout: stdout(),
            rect: self.rect,
        }
    }
}
