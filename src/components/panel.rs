use crate::components::Component;
use crate::interface::canvas::Canvas;
use crate::interface::coordinate::Coordinate;

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
        let top_left = Coordinate::new(0, 0);
        let bottom_right = Coordinate::new(canvas.width() - 1, canvas.height() - 1);

        canvas.rectangle(top_left, bottom_right);

        canvas
    }
}