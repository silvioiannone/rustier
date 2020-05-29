extern crate ctrlc;

pub mod components;
pub mod interface;
pub mod terminal;

pub use interface::Interface;
pub use crossterm::event::KeyCode;

use terminal::Terminal;

/// The rustier TUI.
pub struct Rustier {}

impl Rustier {
    /// Initialize rustier.
    pub fn init() {
        Terminal::enable_raw_mode();
        Terminal::enter_alternate_screen();
        Terminal::clear();
        Terminal::move_cursor(0, 0);
        Terminal::hide_cursor();
        Terminal::enable_mouse();
    }

    /// Clean and quit.
    pub fn quit() {
        Terminal::clear();
        Terminal::disable_mouse();
        Terminal::show_cursor();
        Terminal::leave_alternate_screen();
        Terminal::disable_raw_mode();
    }
}
