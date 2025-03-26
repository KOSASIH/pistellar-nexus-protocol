use uuid::Uuid;
use education::webinars::{WebinarManager, Webinar};
use log::{info, error};

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
        info!("Webinar added successfully with ID: {}", webinar_id);
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
        info!("Retrieved webinar: {:?}", webinar);
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
        info!("Total webinars listed: {}", webinars.len());
    }

    #[test]
    fn test_get_nonexistent_webinar() {
        let manager = WebinarManager::new();
        let nonexistent_id = Uuid::new_v4(); // Generate a random UUID
        let webinar = manager.get_webinar(nonexistent_id);
        assert!(webinar.is_none());
        info!("Correctly handled retrieval of nonexistent webinar with ID: {}", nonexistent_id);
    }

    #[test]
    fn test_add_duplicate_webinar() {
        let mut manager = WebinarManager::new();
        let webinar_id = manager.add_webinar(
            "Duplicate Webinar".to_string(),
            "Testing duplicate webinar addition.".to_string(),
            "Duplicate Presenter".to_string(),
            "2023-10-24".to_string(),
        );

        // Attempt to add the same webinar again
        let duplicate_id = manager.add_webinar(
            "Duplicate Webinar".to_string(),
            "Testing duplicate webinar addition.".to_string(),
            "Duplicate Presenter".to_string(),
            "2023-10-24".to_string(),
        );

        assert_ne!(webinar_id, duplicate_id);
        info!("Successfully added duplicate webinar with new ID: {}", duplicate_id);
    }

    #[test]
    fn test_close_webinar() {
        let mut manager = WebinarManager::new();
        let webinar_id = manager.add_webinar(
            "Closing Webinar".to_string(),
            "Testing closing a webinar.".to_string(),
            "Closer Presenter".to_string(),
            "2023-10-25".to_string(),
        );

        // Assuming there's a method to close a webinar
        manager.close_webinar(webinar_id).expect("Failed to close webinar");
        let webinar = manager.get_webinar(webinar_id).unwrap();
        assert!(!webinar.is_active); // Assuming `is_active` is a field in the Webinar struct
        info!("Successfully closed webinar with ID: {}", webinar_id);
    }
            }
