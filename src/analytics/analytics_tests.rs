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

        while let Some((key, value)) = rx.recv().await {
            analytics.process_data(key, value).await;
        }

        assert_eq!(analytics.calculate_average("temperature"), Some(22.75));
        assert_eq!(analytics.calculate_sum("humidity"), Some(125.0));
        assert_eq!(analytics.get_data().len(), 2);
    }

    #[tokio::test]
    async fn test_calculate_average() {
        let analytics = Analytics::new();
        analytics.process_data("temperature".to_string(), 20.0).await;
        analytics.process_data("temperature".to_string(), 30.0).await;

        assert_eq!(analytics.calculate_average("temperature"), Some(25.0));
    }

    #[tokio::test]
    async fn test_calculate_sum() {
        let analytics = Analytics::new();
        analytics.process_data("humidity".to_string(), 50.0).await;
        analytics.process_data("humidity".to_string(), 70.0).await;

        assert_eq!(analytics.calculate_sum("humidity"), Some(120.0));
    }
}
