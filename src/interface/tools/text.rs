//!
//! A tool that writes text on a canvas.
//!

use crate::interface::{Tool, Canvas, Cell, Coordinate};

pub struct Text<'a> {
    content: &'a str,
    start: Coordinate
}

impl<'a> Text<'a> {
    /// Create a new text.
    pub fn new() -> Self {
        Text {
            content: "",
            start: Coordinate::new(0, 0),
        }
    }

    /// Set the text position on the canvas.
    pub fn at(mut self, start: Coordinate) -> Self {
        self.start = start;
        self
    }

    /// Set the text content.
    pub fn content(&'a mut self, string: &'a str) -> &'a Self {
        self.content = string;
        self
    }
}

impl<'a> Tool for Text<'a> {
    /// Apply the tool on the canvas.
    fn apply<'b>(&self, canvas: &'b mut Canvas) -> &'b Canvas {
        let mut offset = 0;
        for character in self.content.chars() {
            let cell = Cell::new(character);
            canvas.put(cell, Coordinate::new(self.start.x + offset, self.start.y));
            offset += 1;
        }
        canvas
    }
}
