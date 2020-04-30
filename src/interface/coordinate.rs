//!
//! A canvas coordinate.
//!

/// A canvas coordinate.
pub struct Coordinate {
    pub x: u16,
    pub y: u16
}

impl Coordinate {
    /// Create a new coordinate.
    pub fn new(x: u16, y: u16) -> Self {
        Coordinate { x, y }
    }
}