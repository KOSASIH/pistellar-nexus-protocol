use reqwest::blocking::get;
use reqwest::Error as ReqwestError;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use log::{info, error};

pub struct PiTool {
    pi_data: HashMap<String, Value>,
}

impl PiTool {
    pub fn new() -> Result<Self, String> {
        let pi_data = fs::read_to_string("data/pi_data.json")
            .map_err(|e| format!("Failed to read pi_data.json: {}", e))?;
        
        let pi_data: HashMap<String, Value> = serde_json::from_str(&pi_data)
            .map_err(|e| format!("Failed to parse JSON: {}", e))?;
        
        Ok(PiTool { pi_data })
    }

    pub fn get_asset_info(&self, symbol: &str) -> Option<Value> {
        self.pi_data.get("pi_assets").and_then(|assets| {
            assets.as_array().and_then(|assets| {
                assets.iter().find(|asset| {
                    asset.get("symbol").map_or(false, |s| s.as_str() == Some(symbol))
                }).cloned()
            })
        })
    }

    pub fn get_network_info(&self, network: &str) -> Option<Value> {
        self.pi_data.get("pi_network").and_then(|networks| {
            networks.get(network).cloned()
        })
    }

    pub fn fetch_api_data(&self, network: &str) -> Result<String, String> {
        let api_url = self.get_network_info(network)
            .and_then(|network| network.get("api_url").and_then(|url| url.as_str()))
            .ok_or_else(|| format!("API URL not found for network: {}", network))?;
        
        info!("Fetching data from API: {}", api_url);
        
        get(api_url)
            .map_err(|e| format!("Failed to fetch data from API: {}", e))?
            .text()
            .map_err(|e| format!("Failed to read response text: {}", e))
    }
                }
