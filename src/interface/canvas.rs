//!
//! Canvas.
//!

use std::fmt;
use std::fmt::{Write, Display, Formatter};

use crate::interface::Cell;
use crate::interface::Coordinate;
use crate::interface::tools::{Line, Tool, Rectangle, Text};
use crate::terminal::Terminal;

/// A canvas is where a component is allowed to be displayed.
#[derive(Debug)]
pub struct Canvas {
    height: u16,
    width: u16,
    buffer: Vec<Cell>
}

impl Canvas {
    /// Create a new canvas.
    pub fn new(width: u16, height: u16) -> Self {
        let buffer = Self::make_buffer(width, height);

        Canvas { width, height, buffer }
    }

    /// Insert the given cell in the canvas.
    pub fn put(&mut self, cell: Cell, coordinate: Coordinate) -> &Self {
        if coordinate.x >= self.width {
            panic!("The x coordinate falls outside the canvas.")
        }

        if coordinate.y >= self.height {
            panic!("The y coordinate falls outside the canvas.")
        }

        let index = (coordinate.y * self.width + coordinate.x) as usize;

        self.buffer[index] = cell;
        self
    }

    /// Get the height.
    pub fn height(&self) -> u16 {
        self.height
    }

    /// Get the width.
    pub fn width(&self) -> u16 {
        self.width
    }

    /// Create a Canvas buffer.
    fn make_buffer(width: u16, height: u16) -> Vec<Cell>
    {
        let mut vec = Vec::new();

        for _ in 0..height {
            for _ in 0..width {
                vec.push(Cell::new(' '));
            }
        }

        vec
    }

    /// Go over every single cell in the canvas.
    pub fn scan<F>(&mut self, mut scanner: F)
        where F: FnMut(&mut Self, u16, u16) {
        for x in 0..self.width() {
            for y in 0..self.height() {
                scanner(self, x, y);
            }
        }
    }

    /// Draw a line on the canvas.
    pub fn line(&mut self, start: Coordinate, end: Coordinate) {
        Line::new()
            .from(start)
            .to(end)
            .apply(self);
    }

    /// Draw a rectangle on the canvas.
    pub fn rectangle(&mut self, top_left: Coordinate, top_right: Coordinate) {
        Rectangle::new()
            .from(top_left)
            .to(top_right)
            .apply(self);
    }

    /// Draw text on the canvas.
    pub fn text(&mut self, start: Coordinate, content: &str) {
        Text::new()
            .at(start)
            .content(content)
            .apply(self);
    }
}

impl Default for Canvas {
    // Create a canvas with default values.
    fn default() -> Self {
        let size = Terminal::size();
        Self::new(size.0, size.1)
    }
}

impl Display for Canvas {
    /// Display the canvas on screen.
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut result = Err(fmt::Error);

        for cell in self.buffer.iter() {
            result = write!(f, "{}", cell.symbol());

            if result.is_err() {
                break;
            }
        }

        result
    }
}

impl Write for Canvas {
    /// Write a string into the canvas.
    fn write_str(&mut self, string: &str) -> fmt::Result {
        for symbol in string.chars() {
            let cell = Cell::new(symbol);
            self.buffer.push(cell);
        }
        Ok(())
    }
}
