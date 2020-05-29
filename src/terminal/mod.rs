//!
//! Terminal interface.
//!
use std::io::{stdout, Write};
use crossterm::{event, execute, terminal, cursor};

/// Clear the terminal.
pub fn clear() {
    execute!(stdout(), terminal::Clear(terminal::ClearType::All)).unwrap();
}

/// Flush the output buffer.
pub fn flush() {
    stdout().flush().unwrap();
}

/// Enable raw mode
pub fn enable_raw_mode() {
    terminal::enable_raw_mode().unwrap();
}

/// Disable raw mode.
pub fn disable_raw_mode() {
    terminal::disable_raw_mode().unwrap();
}

/// Enable mouse support.
pub fn enable_mouse() {
    execute!(stdout(), event::EnableMouseCapture).unwrap();
}

/// Disable mouse support.
pub fn disable_mouse() {
    execute!(stdout(), event::DisableMouseCapture).unwrap();
}

/// Enter the alternate screen.
pub fn enter_alternate_screen() {
    execute!(stdout(), terminal::EnterAlternateScreen).unwrap();
}

/// Leave the alternate screen.
pub fn leave_alternate_screen() {
    execute!(stdout(), terminal::LeaveAlternateScreen).unwrap();
}

/// Hide the cursor.
pub fn hide_cursor() {
    execute!(stdout(), cursor::Hide).unwrap();
}

/// Move the cursor to the given position.
pub fn move_cursor(x: u16, y: u16) {
    execute!(stdout(), cursor::MoveTo(x, y)).unwrap();
}

/// Show the cursor.
pub fn show_cursor() {
    execute!(stdout(), cursor::Show).unwrap();
}

/// Get the terminal size.
pub fn size() -> (u16, u16) {
    terminal::size().unwrap()
}
