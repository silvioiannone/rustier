//!
//! A collection of tools that can draw on the canvas.
//!

mod line;
mod rectangle;
mod text;

pub use self::line::Line;
pub use self::rectangle::Rectangle;
pub use self::text::Text;

use crate::interface::Canvas;

/// Base tool.
///
/// Each new tool should implement this trait.
pub trait Tool {
    /// Apply the tool on the canvas.
    fn apply<'a>(&self, canvas: &'a mut Canvas) -> &'a Canvas;
}
