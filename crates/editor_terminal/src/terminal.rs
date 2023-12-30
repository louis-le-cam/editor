use std::{
    io::{stdout, Stdout, Write},
    time::Duration,
};

use crossterm::{
    cursor,
    event::{self, Event},
    terminal::{
        disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    ExecutableCommand, QueueableCommand,
};
use log::error;

use crate::{TermRect, TermSlice, TermVec};

/// Exposes some terminal apis for user interface purposes
///
/// Writing to the terminal is possible through [`TermSlice`], see [`Term::slice`]
///
/// Enables `raw mode` and `alternate screen` on creation and disables them on drop
///
/// # Examples:
/// ```
/// # use editor_terminal::{Term, Event, KeyCode, KeyModifiers};
/// let mut term = Term::new();
///
/// while let Ok(event) = term.wait_for_event() {
///     match event {
///         Event::Key(key) => {
///             if key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL) {
///                 break;
///             }
///         }
///         _ => {}
///     }
/// }
/// ```
pub struct Term {
    stdout: Stdout,
    size: TermVec,
}

impl Term {
    /// Create a new `Terminal` enabling terminal `raw mode` and `alternate screen`
    #[must_use]
    pub fn new() -> Self {
        let mut term = Self {
            stdout: stdout(),
            size: match crossterm::terminal::size() {
                Ok(size) => size.into(),
                Err(err) => {
                    error!("Failed to get terminal size, {:?}", err);
                    (0, 0).into()
                }
            },
        };

        if let Err(err) = enable_raw_mode() {
            error!("Failed to enable raw mode, {:?}", err);
        }
        if let Err(err) = term.stdout.queue(EnterAlternateScreen) {
            error!("Failed to enter alternate screen, {:?}", err);
        }
        if let Err(err) = term.stdout.queue(cursor::Hide) {
            error!("Failed to hide the terminal cursor, {:?}", err);
        }

        term.flush();

        term
    }

    /// Create a slice over a portion of the terminal
    ///
    /// Slices allow writing to the terminal
    ///
    /// See [`TermSlice`]
    #[must_use]
    pub fn slice(&mut self, rect: TermRect) -> TermSlice {
        TermSlice::new(rect)
    }

    /// Flush the terminal having the effect of displaying
    /// the commands executed since the last flush
    pub fn flush(&mut self) {
        if let Err(err) = self.stdout.flush() {
            error!("Failed to flush terminal, {:?}", err);
        };
    }

    /// Get the size of the terminal in character count
    ///
    /// The size is cached so this doesn't require any io
    #[must_use]
    pub fn size(&self) -> TermVec {
        self.size
    }

    /// Clear the terminal screen effectively writing spaces to all chars of the terminal
    /// The action will only take effect after flushing the terminal, see [`Term::flush`]
    pub fn clear(&mut self) {
        if let Err(err) = self.stdout.queue(Clear(ClearType::All)) {
            error!("Failed to clear terminal screen, {:?}", err);
        }
    }
}

impl Term {
    /// Get a terminal event if available
    #[must_use]
    pub fn event(&mut self) -> Option<Result<Event, ()>> {
        self.event_available().then(|| self.wait_for_event())
    }

    /// Wait for the next terminal event and return it
    #[must_use]
    pub fn wait_for_event(&mut self) -> Result<Event, ()> {
        match event::read() {
            Ok(event) => {
                self.process_event(&event);
                Ok(event)
            }
            Err(err) => {
                error!("Failed to read terminal event, {:?}", err);
                Err(())
            }
        }
    }

    #[must_use]
    fn event_available(&mut self) -> bool {
        match event::poll(Duration::ZERO) {
            Ok(event_available) => event_available,
            Err(err) => {
                error!("Failed to poll terminal event, {:?}", err);
                false
            }
        }
    }

    fn process_event(&mut self, event: &Event) {
        match event {
            Event::Resize(x, y) => self.size = (*x, *y).into(),
            _ => {}
        };
    }
}

impl Drop for Term {
    fn drop(&mut self) {
        if let Err(err) = self.stdout.execute(LeaveAlternateScreen) {
            error!("Failed to leave alternate screen: {:?}", err);
        }
        if let Err(err) = disable_raw_mode() {
            error!("Failed to disable raw mode: {:?}", err);
        };
        if let Err(err) = self.stdout.queue(cursor::Show) {
            error!("Failed to show the terminal cursor, {:?}", err);
        }
    }
}
