//!
//! Terminal interface.
//!

use std::io::{stdout, Stdout, Write};
use crossterm::{execute, terminal, cursor};

/// A terminal.
pub struct Terminal {
    /// Output buffer.
    pub output: Stdout
}

impl Terminal {
    /// Init the terminal
    pub fn init() -> Terminal {
        Terminal {
            output: stdout()
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
