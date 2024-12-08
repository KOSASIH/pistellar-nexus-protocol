// tests/performance_tests.rs

use education::webinars::WebinarManager;
use std::time::Instant;

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
        println!("Time taken to add 1000 webinars: {:?}", duration);
        assert!(duration.as_secs() < 2); // Ensure it takes less than 2 seconds
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
        println!("Time taken to list 1000 webinars: {:?}", duration);
        assert!(duration.as_secs() < 1); // Ensure it takes less than 1 second
    }
}
