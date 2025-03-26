use reqwest::blocking::get;
use reqwest::Error as ReqwestError;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use log::{info, error};

pub struct StellarTool {
    stellar_data: HashMap<String, Value>,
}

impl StellarTool {
    pub fn new() -> Result<Self, String> {
        let stellar_data = fs::read_to_string("data/stellar_data.json")
            .map_err(|e| format!("Failed to read stellar_data.json: {}", e))?;
        
        let stellar_data: HashMap<String, Value> = serde_json::from_str(&stellar_data)
            .map_err(|e| format!("Failed to parse JSON: {}", e))?;
        
        Ok(StellarTool { stellar_data })
    }

    pub fn get_asset_info(&self, code: &str) -> Option<Value> {
        self.stellar_data.get("stellar_assets").and_then(|assets| {
            assets.as_array().and_then(|assets| {
                assets.iter().find(|asset| {
                    asset.get("code").map_or(false, |c| c.as_str() == Some(code))
                }).cloned()
            })
        })
    }

    pub fn get_network_info(&self, network: &str) -> Option<Value> {
        self.stellar_data.get("stellar_network").and_then(|networks| {
            networks.get(network).cloned()
        })
    }

    pub fn fetch_horizon_data(&self, network: &str) -> Result<String, String> {
        let horizon_url = self.get_network_info(network)
            .and_then(|network| network.get("horizon_url").and_then(|url| url.as_str()))
            .ok_or_else(|| format!("Horizon URL not found for network: {}", network))?;
        
        info!("Fetching data from Horizon API: {}", horizon_url);
        
        get(horizon_url)
            .map_err(|e| format!("Failed to fetch data from Horizon API: {}", e))?
            .text()
            .map_err(|e| format!("Failed to read response text: {}", e))
    }
                }
