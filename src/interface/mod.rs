//!
//! TUI interface.
//!

///
/// TODO: the interface is composed by layers. Drawing a component on a layer
///       will reduce the size of the available layer.
///
///       Each component can also define one or more areas where a component can
///       be placed.
///

pub mod canvas;
pub mod cell;
pub mod coordinate;
pub mod tools;

pub use self::cell::Cell;
pub use self::canvas::Canvas;
pub use self::coordinate::Coordinate;
pub use self::tools::Tool;

use crate::components::Component;
use crate::terminal::Terminal;

use std::io::Write;

/// An interface containing all the components that are displayed on the screen.
pub struct Interface {
    components: Vec<Box<dyn Component>>,
    terminal: Terminal
}

/// Interface's implementation.
impl Interface {
    /// Create a new interface.
    pub fn new(terminal: Terminal) -> Self {
        Self {
            components: vec![],
            terminal
        }
    }

    /// Add a component.
    pub fn add(&mut self, component: Box<dyn Component>) {
        self.components.push(component);
    }

    /// Draw the interface.
    pub fn draw(&mut self) {
        let canvas = &mut self.make_canvas();

        for component in self.components.iter() {
            component.draw(canvas);
        }

        // Copy the canvas into the terminal's buffer.
        self.reset();
        write!(self.terminal.output, "{}", canvas).unwrap();
        self.terminal.flush();
    }

    /// Reset and prepare for a new frame.
    fn reset(&mut self) {
        &self.terminal.clear();
        &self.terminal.move_cursor(0, 0);
    }

    /// Create a canvas where a component can be drawn.
    fn make_canvas(&self) -> Canvas {
        Canvas::default()
    }
}

