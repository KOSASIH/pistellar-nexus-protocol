use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;

#[derive(Debug)]
pub struct Analytics {
    data_stream: Arc<Mutex<HashMap<String, Vec<f64>>>>,
}

impl Analytics {
    pub fn new() -> Self {
        Analytics {
            data_stream: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn process_data(&self, key: String, value: f64) {
        let mut data = self.data_stream.lock().unwrap();
        data.entry(key).or_insert_with(Vec::new).push(value);
    }

    pub fn calculate_average(&self, key: &str) -> Option<f64> {
        let data = self.data_stream.lock().unwrap();
        if let Some(values) = data.get(key) {
            let sum: f64 = values.iter().sum();
            Some(sum / values.len() as f64)
        } else {
            None
        }
    }

    pub fn calculate_sum(&self, key: &str) -> Option<f64> {
        let data = self.data_stream.lock().unwrap();
        data.get(key).map(|values| values.iter().sum())
    }

    pub fn get_data(&self) -> HashMap<String, Vec<f64>> {
        self.data_stream.lock().unwrap().clone()
    }
}

// Function to simulate real-time data streaming
pub async fn data_stream_simulator(analytics: Arc<Analytics>, mut tx: mpsc::Sender<(String, f64)>) {
    let data_points = vec![
        ("temperature".to_string(), 22.5),
        ("humidity".to_string(), 60.0),
        ("temperature".to_string(), 23.0),
        ("humidity".to_string(), 65.0),
    ];

    for (key, value) in data_points {
        analytics.process_data(key.clone(), value).await;
        tx.send((key, value)).await.unwrap();
    }
}
