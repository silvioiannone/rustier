extern crate ctrlc;

pub mod components;
pub mod interface;
pub mod terminal;

pub use terminal::Terminal;
pub use interface::Interface;

use crossterm::event::{Event,  KeyCode};

/// Initialize rustier.
pub fn init() -> Terminal {
    init_screen()
}

/// Initialize the screen.
fn init_screen() -> Terminal {
    let mut terminal = Terminal::init();

    terminal.enable_raw_mode();
    terminal.enter_alternate_screen();
    terminal.clear();
    terminal.move_cursor(0, 0);
    terminal.hide_cursor();
    terminal.enable_mouse();

    terminal.event_handler.register_key(
        KeyCode::Esc,
        Box::new(|_event: Event| {
            let mut terminal = Terminal::init();
            terminal.clear();
            terminal.disable_mouse();
            terminal.show_cursor();
            terminal.leave_alternate_screen();
            terminal.disable_raw_mode();
            std::process::exit(0);
        })
    );

    terminal
}
