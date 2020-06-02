//!
//! Available components.
//!

pub mod panel;
pub mod app_frame;

use crate::interface::canvas::Canvas;

pub use self::app_frame::AppFrame;
pub use self::panel::Panel;

/// Base component.
///
/// Each component should implement this trait.
pub trait Component {
    /// Draw the component.
    fn draw<'a>(&self, canvas: &'a mut Canvas) -> &'a Canvas;
}

