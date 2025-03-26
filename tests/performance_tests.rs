use education::webinars::WebinarManager;
use std::time::Instant;
use log::{info, error};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_performance_add_webinars() {
        let mut manager = WebinarManager::new();
        let start_time = Instant::now();

        for i in 0..1000 {
            manager.add_webinar(
                format!("Webinar {}", i),
                "Performance testing webinar addition.".to_string(),
                "Performance Presenter".to_string(),
                "2023-10-24".to_string(),
            );
        }

        let duration = start_time.elapsed();
        info!("Time taken to add 1000 webinars: {:?}", duration);
        assert!(duration.as_secs() < 2, "Adding 1000 webinars took too long: {:?}", duration);
    }

    #[test]
    fn test_performance_list_webinars() {
        let mut manager = WebinarManager::new();
        for i in 0..1000 {
            manager.add_webinar(
                format!("Webinar {}", i),
                "Testing listing performance.".to_string(),
                "List Presenter".to_string(),
                "2023-10-25".to_string(),
            );
        }

        let start_time = Instant::now();
        let _webinars = manager.list_webinars();
        let duration = start_time.elapsed();
        info!("Time taken to list 1000 webinars: {:?}", duration);
        assert!(duration.as_secs() < 1, "Listing 1000 webinars took too long: {:?}", duration);
    }

    #[test]
    fn test_performance_add_large_number_of_webinars() {
        let mut manager = WebinarManager::new();
        let num_webinars = 5000; // Test with a larger number
        let start_time = Instant::now();

        for i in 0..num_webinars {
            manager.add_webinar(
                format!("Webinar {}", i),
                "Performance testing large number of webinar additions.".to_string(),
                "Large Number Presenter".to_string(),
                "2023-10-26".to_string(),
            );
        }

        let duration = start_time.elapsed();
        info!("Time taken to add {} webinars: {:?}", num_webinars, duration);
        assert!(duration.as_secs() < 5, "Adding {} webinars took too long: {:?}", num_webinars, duration);
    }
    }
