use serde::{Deserialize, Serialize};
use reqwest::Client;
use std::error::Error;
use log::{info, error};

#[derive(Serialize, Deserialize, Debug)]
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
    /// Creates a new instance of PiIntegration with the specified API URL.
    pub fn new(api_url: &str) -> Self {
        let client = Client::new();
        Self {
            client,
            pi_api_url: api_url.to_string(),
        }
    }

    /// Sends a transaction to the Pi network.
    pub async fn send_transaction(&self, transaction: PiTransaction) -> Result<String, Box<dyn Error>> {
        info!("Sending transaction: {:?}", transaction);
        
        let response = self.client.post(&self.pi_api_url)
            .json(&transaction)
            .send()
            .await?;

        if response.status().is_success() {
            let tx_id: String = response.json().await?;
            info!("Transaction sent successfully, ID: {}", tx_id);
            Ok(tx_id)
        } else {
            let error_message = format!("Failed to send transaction: {}", response.status());
            error!("{}", error_message);
            Err(error_message.into())
        }
    }

    /// Retrieves the status of a transaction by its ID.
    pub async fn get_transaction_status(&self, tx_id: &str) -> Result<String, Box<dyn Error>> {
        info!("Retrieving status for transaction ID: {}", tx_id);
        
        let response = self.client.get(format!("{}/{}", self.pi_api_url, tx_id))
            .send()
            .await?;

        if response.status().is_success() {
            let status: String = response.json().await?;
            info!("Transaction status retrieved successfully: {}", status);
            Ok(status)
        } else {
            let error_message = format!("Failed to retrieve transaction status: {}", response.status());
            error!("{}", error_message);
            Err(error_message.into())
        }
    }
}
