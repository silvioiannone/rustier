extern crate ctrlc;

pub mod components;
pub mod interface;
pub mod terminal;

use terminal::Terminal;

/// Initialize rustier.
pub fn init() -> Terminal {
    let mut terminal = Terminal::init();

    terminal.clear();
    terminal.move_cursor(0, 0);
    terminal.hide_cursor();

    // Handle the <CTRL + C> shortcut.
    ctrlc::set_handler(|| {
        let mut terminal = Terminal::init();
        terminal.clear();
        terminal.show_cursor();
        std::process::exit(0);
    })
        .unwrap();

    terminal
}
