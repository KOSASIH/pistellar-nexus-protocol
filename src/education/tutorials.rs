// education/tutorials.rs

use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Tutorial {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub author: String,
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
        };
        let tutorial_id = tutorial.id;
        self.tutorials.insert(tutorial_id, tutorial);
        tutorial_id
    }

    pub fn get_tutorial(&self, tutorial_id: Uuid) -> Option<&Tutorial> {
        self.tutorials.get(&tutorial_id)
    }

    pub fn list_tutorials(&self) -> Vec<&Tutorial> {
        self.tutorials.values().collect()
    }
}
