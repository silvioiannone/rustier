//!
//! Event handler.
//!

use crossterm::event::{poll, read, Event, KeyCode, KeyEvent, KeyModifiers};
use std::time::Duration;
use std::collections::HashMap;

/// A handler function.
type Handler = Box<dyn Fn(Event) -> ()>;

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
        EventHandler {
            handlers: HashMap::new()
        }
    }

    /// Register an event handler.
    pub fn register(&mut self, event: Event, handler: Handler) {
        if ! self.handlers.contains_key(&event) {
            self.handlers.insert(event.clone(), Vec::new());
        }

        self.handlers.get_mut(&event).unwrap().push(Box::new(handler));
    }

    /// Register an event handler that responds to a key press.
    pub fn register_key(&mut self, key_code: KeyCode, handler: Handler) {
        self.register(
            Event::Key(KeyEvent::new(key_code, KeyModifiers::NONE)),
            handler
        )
    }

    /// Poll an event and handle it.
    pub fn handle(&self) {
        if poll(Duration::from_millis(0)).unwrap() {
            let event = read().unwrap();
            self.handle_event(event);
        }
    }

    /// Handle a registered event.
    fn handle_event(&self, event: Event) {
        if ! self.handlers.contains_key(&event) {
            return;
        }

        /// TODO: use async
        for handler in self.handlers.get(&event).unwrap().iter() {
            handler(event);
        }
    }
}