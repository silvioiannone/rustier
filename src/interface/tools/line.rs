//!
//! A canvas tool that draws a line on it.
//!

extern crate bresenham;

use crate::interface::{Canvas, Cell, Coordinate, Tool};
use bresenham::Bresenham;
use std::convert::TryInto;

/// A line displayed on the canvas.
pub struct Line {
    start: Coordinate,
    end: Coordinate,
    pub settings: Settings
}

/// Line tool settings.
pub struct Settings {
    cell: Cell
}

impl Line {
    /// Create a new line.
    pub fn new() -> Self {
        Line {
            start: Coordinate::new(0, 0),
            end: Coordinate::new(0, 0),
            settings: Settings::default()
        }
    }

    /// Define the starting point.
    pub fn from(mut self, start: Coordinate) -> Self {
        self.start = start;
        self
    }

    /// Define the ending point.
    pub fn to(mut self, end: Coordinate) -> Self {
        self.end = end;
        self
    }

    /// Set the line settings.
    pub fn configure(mut self, settings: Settings) -> Self {
        self.settings = settings;
        self
    }
}

impl Tool for Line {
    /// Apply the tool on the canvas.
    fn apply<'a>(&self, canvas: &'a mut Canvas) -> &'a Canvas {
        let start = ((self.start.x as isize), (self.start.y as isize));
        // We increase the `x` and `y` coordinates by 1 because the last point should be included.
        let end = ((self.end.x as isize) + 1, (self.end.y as isize) + 1);

        for (x, y) in Bresenham::new(start, end) {
            let coordinate = Coordinate::new(
                x.try_into().unwrap(),
                y.try_into().unwrap()
            );

            canvas.put(self.settings.cell, coordinate);
        }

        canvas
    }
}

impl Settings {
    /// Set the cell that will be used by default to draw the line.
    pub fn set_cell(mut self, cell: Cell) -> Self {
        self.cell = cell;
        self
    }
}

impl Default for Settings {
    /// Default line settings.
    fn default() -> Self {
        Settings {
            cell: Cell::new('+')
        }
    }
}
