extern crate ctrlc;

mod event_handler;
mod interface;

pub mod components;
pub mod terminal;

pub use crossterm::event::KeyCode;

use crate::{
    event_handler::EventHandler,
    interface::Interface,
    terminal::Terminal
};

/// The rustier TUI.
pub struct Rustier {
    pub event_handler: EventHandler,
    pub interface: Interface
}

impl Rustier {
    /// Initialize rustier.
    pub fn init() -> Rustier {
        Terminal::enable_raw_mode();
        Terminal::enter_alternate_screen();
        Terminal::clear();
        Terminal::move_cursor(0, 0);
        Terminal::hide_cursor();
        Terminal::enable_mouse();

        Rustier {
            event_handler: EventHandler::new(),
            interface: Interface::new()
        }
    }

    /// Clean and quit.
    pub fn quit() {
        Terminal::clear();
        Terminal::disable_mouse();
        Terminal::show_cursor();
        Terminal::leave_alternate_screen();
        Terminal::disable_raw_mode();
    }

    /// Draw the interface.
    pub fn draw(&mut self) {
        loop {
            self.event_handler.handle();
            self.interface.present();
        }
    }
}
