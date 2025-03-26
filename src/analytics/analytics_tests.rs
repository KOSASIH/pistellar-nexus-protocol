#[cfg(test)]
mod tests {
    use super::*;
    use tokio::sync::mpsc;

    #[tokio::test]
    async fn test_process_data() {
        let analytics = Arc::new(Analytics::new());
        let (tx, mut rx) = mpsc::channel(32);

        // Simulate data streaming
        tokio::spawn(data_stream_simulator(analytics.clone(), tx));

        // Process incoming data from the channel
        while let Some((key, value)) = rx.recv().await {
            analytics.process_data(key, value).await;
        }

        // Validate the results
        assert_eq!(analytics.calculate_average("temperature"), Some(22.75));
        assert_eq!(analytics.calculate_sum("humidity"), Some(125.0));
        assert_eq!(analytics.get_data().len(), 2); // Check that we have two keys
    }

    #[tokio::test]
    async fn test_calculate_average() {
        let analytics = Arc::new(Analytics::new());
        analytics.process_data("temperature".to_string(), 20.0).await;
        analytics.process_data("temperature".to_string(), 30.0).await;

        assert_eq!(analytics.calculate_average("temperature"), Some(25.0));
    }

    #[tokio::test]
    async fn test_calculate_sum() {
        let analytics = Arc::new(Analytics::new());
        analytics.process_data("humidity".to_string(), 50.0).await;
        analytics.process_data("humidity".to_string(), 70.0).await;

        assert_eq!(analytics.calculate_sum("humidity"), Some(120.0));
    }

    #[tokio::test]
    async fn test_no_data_average() {
        let analytics = Arc::new(Analytics::new());
        assert_eq!(analytics.calculate_average("nonexistent_key"), None);
    }

    #[tokio::test]
    async fn test_no_data_sum() {
        let analytics = Arc::new(Analytics::new());
        assert_eq!(analytics.calculate_sum("nonexistent_key"), None);
    }

    #[tokio::test]
    async fn test_get_data() {
        let analytics = Arc::new(Analytics::new());
        analytics.process_data("temperature".to_string(), 22.5).await;
        analytics.process_data("humidity".to_string(), 60.0).await;

        let data = analytics.get_data();
        assert_eq!(data.len(), 2); // Ensure we have two keys
        assert_eq!(data["temperature"].len(), 1); // Check temperature data
        assert_eq!(data["humidity"].len(), 1); // Check humidity data
    }
        }
