use serde::{Deserialize, Serialize};
use reqwest::Client;
use std::error::Error;

#[derive(Serialize, Deserialize)]
pub struct PiTransaction {
    pub sender: String,
    pub receiver: String,
    pub amount: f64,
}

pub struct PiIntegration {
    client: Client,
    pi_api_url: String,
}

impl PiIntegration {
    pub fn new(api_url: &str) -> Self {
        let client = Client::new();
        Self {
            client,
            pi_api_url: api_url.to_string(),
        }
    }

    pub async fn send_transaction(&self, transaction: PiTransaction) -> Result<String, Box<dyn Error>> {
        let response = self.client.post(&self.pi_api_url)
            .json(&transaction)
            .send()
            .await?;

        if response.status().is_success() {
            let tx_id: String = response.json().await?;
            Ok(tx_id)
        } else {
            Err("Failed to send transaction".into())
        }
    }

    pub async fn get_transaction_status(&self, tx_id: &str) -> Result<String, Box<dyn Error>> {
        let response = self.client.get(format!("{}/{}", self.pi_api_url, tx_id))
            .send()
            .await?;

        if response.status().is_success() {
            let status: String = response.json().await?;
            Ok(status)
        } else {
            Err("Failed to retrieve transaction status".into())
        }
    }
}
