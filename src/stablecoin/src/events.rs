use std::sync::{Arc, Mutex};
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    timestamp: DateTime<Utc>,
    message: String,
}

pub struct EventLogger {
    events: Arc<Mutex<Vec<Event>>>, // Use Arc and Mutex for thread-safe access
}

impl EventLogger {
    pub fn new() -> Self {
        EventLogger {
            events: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn log_event(&self, message: String) {
        let event = Event {
            timestamp: Utc::now(),
            message,
        };

        // Lock the mutex to safely access the events vector
        let mut events = self.events.lock().unwrap();
        events.push(event);
        println!("Event logged: {}", events.last().unwrap().message);
    }

    pub fn get_events(&self) -> Vec<Event> {
        // Lock the mutex to safely read the events vector
        let events = self.events.lock().unwrap();
        events.clone() // Return a clone of the events
    }

    pub fn clear_events(&self) {
        let mut events = self.events.lock().unwrap();
        events.clear();
        println!("All events cleared.");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_logging() {
        let logger = EventLogger::new();
        logger.log_event("Test event 1".to_string());
        logger.log_event("Test event 2".to_string());

        let events = logger.get_events();
        assert_eq!(events.len(), 2);
        assert_eq!(events[0].message, "Test event 1");
        assert_eq!(events[1].message, "Test event 2");
    }

    #[test]
    fn test_clear_events() {
        let logger = EventLogger::new();
        logger.log_event("Test event 1".to_string());
        logger.clear_events();

        let events = logger.get_events();
        assert!(events.is_empty());
    }
    }
