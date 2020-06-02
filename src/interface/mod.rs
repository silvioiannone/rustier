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
use std::io::{stdout, Write};

/// An interface containing all the components that are displayed on the screen.
pub struct Interface {
    components: Vec<Box<dyn Component>>
}

/// Interface's implementation.
impl Interface {
    /// Create a new interface.
    pub fn new() -> Self {
        Self {
            components: vec![]
        }
    }

    /// Add a component.
    pub fn add(&mut self, component: Box<dyn Component>) -> &mut Self {
        self.components.push(component);
        self
    }

    /// Present the canvas on the screen.
    pub fn present(&mut self) {
        let canvas = &mut self.make_canvas();

        for component in self.components.iter() {
            component.draw(canvas);
        }

        // Copy the canvas into the terminal's buffer.
        self.reset();
        write!(stdout(), "{}", canvas).unwrap();
        Terminal::flush();
    }

    /// Reset and prepare for a new frame.
    fn reset(&mut self) {
        Terminal::move_cursor(0, 0);
    }

    /// Create a canvas where a component can be drawn.
    fn make_canvas(&self) -> Canvas {
        Canvas::default()
    }
}

