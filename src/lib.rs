extern crate ctrlc;

pub mod components;
pub mod interface;
pub mod terminal;

pub use interface::Interface;
pub use crossterm::event::KeyCode;

/// Initialize rustier.
pub fn init() {
    terminal::enable_raw_mode();
    terminal::enter_alternate_screen();
    terminal::clear();
    terminal::move_cursor(0, 0);
    terminal::hide_cursor();
    terminal::enable_mouse();
}

/// Clean and quit.
pub fn quit() {
    terminal::clear();
    terminal::disable_mouse();
    terminal::show_cursor();
    terminal::leave_alternate_screen();
    terminal::disable_raw_mode();
}
