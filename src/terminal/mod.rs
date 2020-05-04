//!
//! Terminal interface.
//!

pub mod event_handler;

pub use self::event_handler::EventHandler;

use std::io::{stdout, Stdout, Write};
use crossterm::{event, execute, terminal, cursor};

/// A terminal.
pub struct Terminal {
    /// Output buffer.
    pub output: Stdout,
    pub event_handler: EventHandler
}

impl Terminal {
    /// Init the terminal
    pub fn init() -> Terminal {
        Terminal {
            output: stdout(),
            event_handler: EventHandler::new()
        }
    }

    /// Clear the terminal.
    pub fn clear(&mut self) {
        execute!(self.output, terminal::Clear(terminal::ClearType::All)).unwrap();
    }

    /// Flush the output buffer.
    pub fn flush(&mut self) {
        self.output.flush().unwrap();
    }

    /// Enable raw mode
    pub fn enable_raw_mode(&mut self) {
        terminal::enable_raw_mode().unwrap();
    }

    /// Disable raw mode.
    pub fn disable_raw_mode(&mut self) {
        terminal::disable_raw_mode().unwrap();
    }

    /// Enable mouse support.
    pub fn enable_mouse(&mut self) {
        execute!(self.output, event::EnableMouseCapture).unwrap();
    }

    /// Disable mouse support.
    pub fn disable_mouse(&mut self) {
        execute!(self.output, event::DisableMouseCapture).unwrap();
    }

    /// Enter the alternate screen.
    pub fn enter_alternate_screen(&mut self) {
        execute!(self.output, terminal::EnterAlternateScreen).unwrap();
    }

    /// Leave the alternate screen.
    pub fn leave_alternate_screen(&mut self) {
        execute!(self.output, terminal::LeaveAlternateScreen).unwrap();
    }

    /// Hide the cursor.
    pub fn hide_cursor(&mut self) {
        execute!(self.output, cursor::Hide).unwrap();
    }

    /// Move the cursor to the given position.
    pub fn move_cursor(&mut self, x: u16, y: u16) {
        execute!(self.output, cursor::MoveTo(x, y)).unwrap();
    }

    /// Show the cursor.
    pub fn show_cursor(&mut self) {
        execute!(self.output, cursor::Show).unwrap();
    }

    /// Get the terminal size.
    pub fn size() -> (u16, u16) {
        terminal::size().unwrap()
    }
}
