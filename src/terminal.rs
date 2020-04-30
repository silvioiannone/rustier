//!
//! Terminal interface.
//!

use std::io::{stdout, Write, Stdout};

macro_rules! terminal_write {
    ($self:ident, $($arg:tt)*) => {
        $self.output.write_fmt(format_args!($($arg)*)).unwrap();
        $self.output.flush().unwrap();
    }
}

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
        terminal_write!(self, "{}", termion::clear::All);
    }

    /// Flush the output buffer.
    pub fn flush(&mut self) {
        self.output.flush().unwrap();
    }

    /// Hide the cursor.
    pub fn hide_cursor(&mut self) {
        terminal_write!(self, "{}", termion::cursor::Hide);
    }

    /// Move the cursor to the given position.
    pub fn move_cursor(&mut self, x: usize, y: usize) {
        terminal_write!(
            self,
            "{}",
            termion::cursor::Goto((x + 1) as u16, (y + 1) as u16)
        );
    }

    /// Show the cursor.
    pub fn show_cursor(&mut self) {
        terminal_write!(self, "{}", termion::cursor::Show);
    }
}
