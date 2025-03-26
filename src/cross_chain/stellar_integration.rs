use serde::{Deserialize, Serialize};
use reqwest::Client;
use std::error::Error;
use log::{info, error};

#[derive(Serialize, Deserialize, Debug)]
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
    /// Creates a new instance of StellarIntegration with the specified API URL.
    pub fn new(api_url: &str) -> Self {
        let client = Client::new();
        Self {
            client,
            stellar_api_url: api_url.to_string(),
        }
    }

    /// Sends a transaction to the Stellar network.
    pub async fn send_transaction(&self, transaction: StellarTransaction) -> Result<String, Box<dyn Error>> {
        info!("Sending Stellar transaction: {:?}", transaction);
        
        let response = self.client.post(&self.stellar_api_url)
            .json(&transaction)
            .send()
            .await?;

        if response.status().is_success() {
            let tx_id: String = response.json().await?;
            info!("Stellar transaction sent successfully, ID: {}", tx_id);
            Ok(tx_id)
        } else {
            let error_message = format!("Failed to send transaction: {}", response.status());
            error!("{}", error_message);
            Err(error_message.into())
        }
    }

    /// Retrieves the status of a transaction by its ID.
    pub async fn get_transaction_status(&self, tx_id: &str) -> Result<String, Box<dyn Error>> {
        info!("Retrieving status for Stellar transaction ID: {}", tx_id);
        
        let response = self.client.get(format!("{}/{}", self.stellar_api_url, tx_id))
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
