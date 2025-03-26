use std::collections::HashMap;
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use std::fs::{self, File};
use std::io::{self, Write, Read};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tutorial {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub author: String,
    pub version: u32,
}

pub struct TutorialManager {
    tutorials: HashMap<Uuid, Tutorial>,
}

impl TutorialManager {
    pub fn new() -> Self {
        TutorialManager {
            tutorials: HashMap::new(),
        }
    }

    pub fn add_tutorial(&mut self, title: String, content: String, author: String) -> Uuid {
        let tutorial = Tutorial {
            id: Uuid::new_v4(),
            title,
            content,
            author,
            version: 1,
        };
        let tutorial_id = tutorial.id;
        self.tutorials.insert(tutorial_id, tutorial);
        tutorial_id
    }

    pub fn update_tutorial(&mut self, tutorial_id: Uuid, title: Option<String>, content: Option<String>) -> Result<(), String> {
        if let Some(tutorial) = self.tutorials.get_mut(&tutorial_id) {
            if let Some(new_title) = title {
                tutorial.title = new_title;
            }
            if let Some(new_content) = content {
                tutorial.content = new_content;
            }
            tutorial.version += 1; // Increment version on update
            Ok(())
        } else {
            Err("Tutorial not found.".to_string())
        }
    }

    pub fn get_tutorial(&self, tutorial_id: Uuid) -> Option<&Tutorial> {
        self.tutorials.get(&tutorial_id)
    }

    pub fn list_tutorials(&self) -> Vec<&Tutorial> {
        self.tutorials.values().collect()
    }

    pub fn search_tutorials(&self, query: &str) -> Vec<&Tutorial> {
        self.tutorials.values()
            .filter(|tutorial| tutorial.title.contains(query) || tutorial.author.contains(query))
            .collect()
    }

    pub fn save_to_file(&self, file_path: &str) -> io::Result<()> {
        let serialized = serde_json::to_string(&self.tutorials)?;
        let mut file = File::create(file_path)?;
        file.write_all(serialized.as_bytes())?;
        Ok(())
    }

    pub fn load_from_file(&mut self, file_path: &str) -> io::Result<()> {
        let mut file = File::open(file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        self.tutorials = serde_json::from_str(&contents)?;
        Ok(())
    }
}

// Example usage
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_tutorial() {
        let mut manager = TutorialManager::new();
        let tutorial_id = manager.add_tutorial("Rust Basics".to_string(), "Learn Rust from scratch.".to_string(), "Alice".to_string());

        let tutorial = manager.get_tutorial(tutorial_id).unwrap();
        assert_eq!(tutorial.title, "Rust Basics");
        assert_eq!(tutorial.content, "Learn Rust from scratch.");
        assert_eq!(tutorial.author, "Alice");
        assert_eq!(tutorial.version, 1);
    }

    #[test]
    fn test_update_tutorial() {
        let mut manager = TutorialManager::new();
        let tutorial_id = manager.add_tutorial("Rust Basics".to_string(), "Learn Rust from scratch.".to_string(), "Alice".to_string());

        manager.update_tutorial(tutorial_id, Some("Advanced Rust".to_string()), None).unwrap();
        let tutorial = manager.get_tutorial(tutorial_id).unwrap();
        assert_eq!(tutorial.title, "Advanced Rust");
        assert_eq!(tutorial.version, 2);
    }

    #[test]
    fn test_search_tutorials() {
        let mut manager = TutorialManager::new();
        manager.add_tutorial("Rust Basics".to_string(), "Learn Rust from scratch.".to_string(), "Alice".to_string());
        manager.add_tutorial ("Advanced Rust".to_string(), "Deep dive into Rust.".to_string(), "Bob".to_string());

        let results = manager.search_tutorials("Rust");
        assert_eq!(results.len(), 2);

        let results_by_author = manager.search_tutorials("Alice");
        assert_eq!(results_by_author.len(), 1);
        assert_eq!(results_by_author[0].author, "Alice");
    }

    #[test]
    fn test_save_and_load_tutorials() {
        let mut manager = TutorialManager::new();
        manager.add_tutorial("Rust Basics".to_string(), "Learn Rust from scratch.".to_string(), "Alice".to_string());

        let file_path = "tutorials.json";
        manager.save_to_file(file_path).unwrap();

        let mut new_manager = TutorialManager::new();
        new_manager.load_from_file(file_path).unwrap();

        let loaded_tutorial = new_manager.get_tutorial(manager.tutorials.keys().next().unwrap().clone()).unwrap();
        assert_eq!(loaded_tutorial.title, "Rust Basics");
        assert_eq!(loaded_tutorial.content, "Learn Rust from scratch.");
        assert_eq!(loaded_tutorial.author, "Alice");
        assert_eq!(loaded_tutorial.version, 1);

        // Clean up
        let _ = fs::remove_file(file_path);
    }

    #[test]
    fn test_update_nonexistent_tutorial() {
        let mut manager = TutorialManager::new();
        let result = manager.update_tutorial(Uuid::new_v4(), Some("New Title".to_string()), None);
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), "Tutorial not found.");
    }
    }
