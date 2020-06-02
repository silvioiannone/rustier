use crate::components::Component;
use crate::interface::canvas::Canvas;
use crate::interface::coordinate::Coordinate;

/// A panel component.
pub struct AppFrame {
    children: Vec<Box<dyn Component>>,
    title: String,
}

impl AppFrame {
    /// Set the app frame title
    pub fn title(mut self, title: String) -> Self {
        self.title = title;
        self
    }

    /// Insert a component inside the app frame.
    pub fn insert(mut self, component: Box<dyn Component>) -> Self {
        self.children.push(component);
        self
    }
}

impl Default for AppFrame {
    /// Create a new panel.
    fn default() -> AppFrame {
        AppFrame {
            children: vec![],
            title: "App name".to_string(),
        }
    }
}

impl Component for AppFrame {
    /// Draw the component.
    fn draw<'a>(&self, canvas: &'a mut Canvas) -> &'a Canvas {
        // Rectangle frame.
        let top_left = Coordinate::new(0, 0);
        let bottom_right = Coordinate::new(canvas.width() - 1, canvas.height() - 1);

        canvas.rectangle(top_left, bottom_right);

        // Title.
        let prefix = "╡ ".to_string();
        let suffix = " ╞".to_string();
        let mut title = String::from(prefix);
        title.push_str(&self.title);
        title.push_str(&suffix);

        canvas.text(Coordinate::new(4, 0), &title);

        canvas
    }
}