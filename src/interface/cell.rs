//!
//! A canvas cell.
//!

/// A cell in the Canvas.
#[derive(Copy, Clone, Debug)]
pub struct Cell {
    symbol: char
}

impl Cell {
    /// Create a new Cell.
    pub fn new(symbol: char) -> Self {
        Cell { symbol }
    }

    /// Get the symbol.
    pub fn symbol(&self) -> char {
        self.symbol
    }
}
