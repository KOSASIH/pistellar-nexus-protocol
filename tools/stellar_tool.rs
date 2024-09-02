use serde_json::json;
use reqwest::blocking::get;
use std::collections::HashMap;

pub struct StellarTool {
    stellar_data: HashMap<String, serde_json::Value>,
}

impl StellarTool {
    pub fn new() -> Self {
        let stellar_data = serde_json::from_str(&std::fs::read_to_string("data/stellar_data.json").unwrap()).unwrap();
        StellarTool { stellar_data }
    }

    pub fn get_asset_info(&self, code: &str) -> Option<serde_json::Value> {
        self.stellar_data.get("stellar_assets").and_then(|assets| {
            assets.as_array().and_then(|assets| {
                assets.into_iter().find(|asset| asset.get("code").map_or(false, |code| code.as_str() == Some(code))).cloned()
            })
        })
    }

    pub fn get_network_info(&self, network: &str) -> Option<serde_json::Value> {
        self.stellar_data.get("stellar_network").and_then(|networks| {
            networks.get(network).cloned()
        })
    }

    pub fn fetch_horizon_data(&self, network: &str) -> Result<String, reqwest::Error> {
        let horizon_url = self.get_network_info(network).and_then(|network| network.get("horizon_url").map_or(None, |url| url.as_str()))?;
        get(horizon_url)?.text()
    }
}
