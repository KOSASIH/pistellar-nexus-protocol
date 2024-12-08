use serde::{Deserialize, Serialize};
use reqwest::Client;
use std::error::Error;

#[derive(Serialize, Deserialize)]
pub struct StellarTransaction {
    pub source_account: String,
    pub destination_account: String,
    pub amount: f64,
}

pub struct StellarIntegration {
    client: Client,
    stellar_api_url: String,
}

impl StellarIntegration {
    pub fn new(api_url: &str) -> Self {
        let client = Client::new();
        Self {
            client,
            stellar_api_url: api_url.to_string(),
        }
    }

    pub async fn send_transaction(&self, transaction: StellarTransaction) -> Result<String, Box<dyn Error>> {
        let response = self.client.post(&self.stellar_api_url)
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
        let response = self.client.get(format!("{}/{}", self.stellar_api_url, tx_id))
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
