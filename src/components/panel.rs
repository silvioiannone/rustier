use crate::components::Component;
use crate::interface::canvas::Canvas;

/// A panel component.
pub struct Panel {
    pub title: String
}

impl Default for Panel {
    /// Create a new panel.
    fn default() -> Panel {
        Panel {
            title: "Default title".to_string()
        }
    }
}

impl Component for Panel {
    /// Draw the component.
    fn draw<'a>(&self, canvas: &'a mut Canvas) -> &'a Canvas {
        println!("Drawing panel...");
        canvas
    }
}