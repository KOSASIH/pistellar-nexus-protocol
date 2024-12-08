// tests/integration_tests.rs

use uuid::Uuid;
use education::webinars::{WebinarManager, Webinar};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_webinar() {
        let mut manager = WebinarManager::new();
        let webinar_id = manager.add_webinar(
            "Test Webinar".to_string(),
            "Testing the add_webinar function.".to_string(),
            "Test Presenter".to_string(),
            "2023-10-20".to_string(),
        );

        assert!(manager.get_webinar(webinar_id).is_some());
    }

    #[test]
    fn test_get_webinar() {
        let mut manager = WebinarManager::new();
        let webinar_id = manager.add_webinar(
            "Another Test Webinar".to_string(),
            "Testing retrieval of a webinar.".to_string(),
            "Another Presenter".to_string(),
            "2023-10-21".to_string(),
        );

        let webinar = manager.get_webinar(webinar_id).unwrap();
        assert_eq!(webinar.title, "Another Test Webinar");
        assert_eq!(webinar.presenter, "Another Presenter");
    }

    #[test]
    fn test_list_webinars() {
        let mut manager = WebinarManager::new();
        manager.add_webinar(
            "Webinar 1".to_string(),
            "Description 1".to_string(),
            "Presenter 1".to_string(),
            "2023-10-22".to_string(),
        );
        manager.add_webinar(
            "Webinar 2".to_string(),
            "Description 2".to_string(),
            "Presenter 2".to_string(),
            "2023-10-23".to_string(),
        );

        let webinars = manager.list_webinars();
        assert_eq!(webinars.len(), 2);
    }
}
