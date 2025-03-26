use std::collections::HashMap;
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use std::fs::{self, File};
use std::io::{self, Write, Read};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Webinar {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub presenter: String,
    pub date: String, // Consider using a more structured date type in a real application
    pub is_completed: bool,
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
            is_completed: false,
        };
        let webinar_id = webinar.id;
        self.webinars.insert(webinar_id, webinar);
        webinar_id
    }

    pub fn complete_webinar(&mut self, webinar_id: Uuid) -> Result<(), String> {
        if let Some(webinar) = self.webinars.get_mut(&webinar_id) {
            webinar.is_completed = true;
            Ok(())
        } else {
            Err("Webinar not found.".to_string())
        }
    }

    pub fn get_webinar(&self, webinar_id: Uuid) -> Option<&Webinar> {
        self.webinars.get(&webinar_id)
    }

    pub fn list_webinars(&self) -> Vec<&Webinar> {
        self.webinars.values().collect()
    }

    pub fn search_webinars(&self, query: &str) -> Vec<&Webinar> {
        self.webinars.values()
            .filter(|webinar| webinar.title.contains(query) || webinar.presenter.contains(query))
            .collect()
    }

    pub fn save_to_file(&self, file_path: &str) -> io::Result<()> {
        let serialized = serde_json::to_string(&self.webinars)?;
        let mut file = File::create(file_path)?;
        file.write_all(serialized.as_bytes())?;
        Ok(())
    }

    pub fn load_from_file(&mut self, file_path: &str) -> io::Result<()> {
        let mut file = File::open(file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        self.webinars = serde_json::from_str(&contents)?;
        Ok(())
    }
}

// Example usage
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_webinar() {
        let mut manager = WebinarManager::new();
        let webinar_id = manager.add_webinar("Rust Basics".to_string(), "Introduction to Rust programming.".to_string(), "Alice".to_string(), "2023-10-01".to_string());

        let webinar = manager.get_webinar(webinar_id).unwrap();
        assert_eq!(webinar.title, "Rust Basics");
        assert_eq!(webinar.description, "Introduction to Rust programming.");
        assert_eq!(webinar.presenter, "Alice");
        assert_eq!(webinar.is_completed, false);
    }

    #[test]
    fn test_complete_webinar() {
        let mut manager = WebinarManager::new();
        let webinar_id = manager.add_webinar("Rust Basics".to_string(), "Introduction to Rust programming.".to_string(), "Alice".to_string(), "2023-10-01".to_string());

        manager.complete_webinar(webinar_id).unwrap();
        let webinar = manager.get_webinar(webinar_id).unwrap();
        assert!(webinar.is_completed);
    }

    #[test]
    fn test_search_webinars() {
        let mut manager = WebinarManager::new();
        manager.add_webinar("Rust Basics".to_string(), "Introduction to Rust programming.".to_string(), "Alice".to_string(), "2023-10-01".to_string());
        manager.add_webinar("Advanced Rust". .to_string(), "Deep dive into Rust.".to_string(), "Bob".to_string(), "2023-10-15".to_string());

        let results = manager.search_webinars("Rust");
        assert_eq!(results.len(), 2);

        let results_by_presenter = manager.search_webinars("Alice");
        assert_eq!(results_by_presenter.len(), 1);
        assert_eq!(results_by_presenter[0].presenter, "Alice");
    }

    #[test]
    fn test_save_and_load_webinars() {
        let mut manager = WebinarManager::new();
        manager.add_webinar("Rust Basics".to_string(), "Introduction to Rust programming.".to_string(), "Alice".to_string(), "2023-10-01".to_string());

        let file_path = "webinars.json";
        manager.save_to_file(file_path).unwrap();

        let mut new_manager = WebinarManager::new();
        new_manager.load_from_file(file_path).unwrap();

        let loaded_webinar = new_manager.get_webinar(manager.webinars.keys().next().unwrap().clone()).unwrap();
        assert_eq!(loaded_webinar.title, "Rust Basics");
        assert_eq!(loaded_webinar.description, "Introduction to Rust programming.");
        assert_eq!(loaded_webinar.presenter, "Alice");
        assert_eq!(loaded_webinar.is_completed, false);

        // Clean up
        let _ = fs::remove_file(file_path);
    }

    #[test]
    fn test_complete_nonexistent_webinar() {
        let mut manager = WebinarManager::new();
        let result = manager.complete_webinar(Uuid::new_v4());
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), "Webinar not found.");
    }
    }
