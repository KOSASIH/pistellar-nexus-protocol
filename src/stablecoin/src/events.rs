// src/events.rs
pub struct EventLogger {
    events: Vec<String>,
}

impl EventLogger {
    pub fn new() -> Self {
        EventLogger {
            events: Vec::new(),
        }
    }

    pub fn log_event(&mut self, event: String) {
        self.events.push(event);
        println!("Event logged: {}", event);
    }
}
