//!
//! Event handler.
//!
use crossterm::event::{
    self, Event, KeyCode, KeyEvent, KeyModifiers
};
use std::collections::HashMap;
use std::time::Duration;

/// A handler function.
type Handler = Box<dyn FnMut(Event) + Send>;

/// Event handlers container.
///
/// It's an hash-map which has an `crossterm::event::Event` as a key and a `Box` with handler
/// closures as a value.
type HandlersContainer = HashMap<Event, Vec<Handler>>;

/// A terminal event handler.
pub struct EventHandler {
    handlers: HandlersContainer
}

/// Event handler implementation.
impl EventHandler {

    /// Create a new event handler.
    pub fn new() -> EventHandler {
        let (sender, receiver) = crossbeam_channel::unbounded();

        EventHandler {
            handlers: HashMap::new()
        }
    }

    /// Register an event handler.
    pub fn register(&mut self, event: Event, handler: Handler) {
        if ! self.handlers.contains_key(&event) {
            self.handlers.insert(event.clone(), Vec::new());
        }

        self.handlers.get_mut(&event).unwrap().push(handler);
    }

    /// Register an event handler that responds to a key press.
    pub fn register_key(&mut self, key_code: KeyCode, handler: Handler) {
        self.register(
            Event::Key(KeyEvent::new(key_code, KeyModifiers::NONE)),
            handler
        )
    }

    /// Poll an event and handle it.
    pub fn handle(&mut self) {
        match event::poll(Duration::from_millis(1)) {
            Ok(true) => match event::read() {
                Ok(event) => self.handle_event(event),
                Err(e) => panic!("{:?}", e),
            }
            _ => ()
        }
    }

    /// Handle a registered event.
    fn handle_event(&mut self, event: Event) {
        if ! self.handlers.contains_key(&event) {
            return;
        }

        // TODO: use async
        for handler in self.handlers.get_mut(&event).unwrap().iter_mut() {
            handler(event);
        }
    }
}
