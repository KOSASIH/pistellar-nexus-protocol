// education/webinars.rs

use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Webinar {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub presenter: String,
    pub date: String, // Consider using a more structured date type in a real application
}

pub struct WebinarManager {
    webinars: HashMap<Uuid, Webinar>,
}

impl WebinarManager {
    pub fn new() -> Self {
        WebinarManager {
            webinars: HashMap::new(),
        }
    }

    pub fn add_webinar(&mut self, title: String, description: String, presenter: String, date: String) -> Uuid {
        let webinar = Webinar {
            id: Uuid::new_v4(),
            title,
            description,
            presenter,
            date,
        };
        let webinar_id = webinar.id;
        self.webinars.insert(webinar_id, webinar);
        webinar_id
    }

    pub fn get_webinar(&self, webinar_id: Uuid) -> Option<&Webinar> {
        self.webinars.get(&webinar_id)
    }

    pub fn list_webinars(&self) -> Vec<&Webinar> {
        self.webinars.values().collect()
    }
}
