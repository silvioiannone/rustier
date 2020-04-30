//!
//! A canvas tool that draws a rectangle on it.
//!

use crate::interface::{Coordinate, Canvas, Cell};
use crate::interface::tools::{Tool, Line};
use crate::interface::tools::line::Settings as LineSettings;

/// Rectangle settings.
pub struct Settings {
    vertical_stretch_character: Cell,
    horizontal_stretch_character: Cell,
    top_left_corner_character: Cell,
    top_right_corner_character: Cell,
    bottom_left_corner_character: Cell,
    bottom_right_corner_character: Cell
}

/// A rectangle drawing tool.
pub struct Rectangle {
    top_left: Coordinate,
    bottom_right: Coordinate,
    settings: Settings
}

impl Rectangle {
    /// Create a new rectangle.
    pub fn new() -> Self {
        Rectangle {
            top_left: Coordinate::new(0, 0),
            bottom_right: Coordinate::new(0, 0),
            settings: Settings::default()
        }
    }

    /// Define the starting point.
    pub fn from(mut self, start: Coordinate) -> Self {
        self.top_left = start;
        self
    }

    /// Define the ending point.
    pub fn to(mut self, end: Coordinate) -> Self {
        self.bottom_right = end;
        self
    }

    /// Top right corner coordinates.
    fn top_right_corner(&self) -> Coordinate {
        Coordinate::new(self.bottom_right.x, self.top_left.y)
    }

    /// Top left corner coordinates.
    fn top_left_corner(&self) -> Coordinate {
        Coordinate::new(self.top_left.x, self.top_left.y)
    }

    /// Bottom left corner coordinates.
    fn bottom_left_corner(&self) -> Coordinate {
        Coordinate::new(self.top_left.x, self.bottom_right.y)
    }

    /// Bottom right corner coordinates.
    fn bottom_right_corner(&self) -> Coordinate {
        Coordinate::new(self.bottom_right.x, self.bottom_right.y)
    }
}

impl Tool for Rectangle {
    /// Apply the tool on the canvas.
    fn apply<'a>(&self, canvas: &'a mut Canvas) -> &'a Canvas {
        // First draw the 4 sides.

        // Top side
        let top_side_settings = LineSettings::default()
            .set_cell(self.settings.horizontal_stretch_character);

        Line::new()
            .configure(top_side_settings)
            .from(self.top_left_corner())
            .to(self.top_right_corner())
            .apply(canvas);

        // Right side
        let right_side_settings  = LineSettings::default()
            .set_cell(self.settings.vertical_stretch_character);

        Line::new()
            .configure(right_side_settings)
            .from(self.top_right_corner())
            .to(self.bottom_right_corner())
            .apply(canvas);

        // Bottom side
        let bottom_side_settings = LineSettings::default()
            .set_cell(self.settings.horizontal_stretch_character);

        Line::new()
            .configure(bottom_side_settings)
            .from(self.bottom_left_corner())
            .to(self.bottom_right_corner())
            .apply(canvas);

        // Left side
        let left_side_settings = LineSettings::default()
            .set_cell(self.settings.vertical_stretch_character);

        Line::new()
            .configure(left_side_settings)
            .from(self.top_left_corner())
            .to(self.bottom_left_corner())
            .apply(canvas);

        // Draw the corners.
        canvas.put(self.settings.top_left_corner_character, self.top_left_corner());
        canvas.put(self.settings.top_right_corner_character, self.top_right_corner());
        canvas.put(self.settings.bottom_right_corner_character, self.bottom_right_corner());
        canvas.put(self.settings.bottom_left_corner_character, self.bottom_left_corner());

        canvas
    }
}

impl Default for Settings {
    /// Default settings.
    fn default() -> Self {
        Settings {
            vertical_stretch_character: Cell::new('║'),
            horizontal_stretch_character: Cell::new('═'),
            top_left_corner_character: Cell::new('╔'),
            top_right_corner_character: Cell::new('╗'),
            bottom_left_corner_character: Cell::new('╚'),
            bottom_right_corner_character: Cell::new('╝'),
        }
    }
}