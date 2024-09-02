use serde_json::json;
use reqwest::blocking::get;
use std::collections::HashMap;

pub struct PiTool {
    pi_data: HashMap<String, serde_json::Value>,
}

impl PiTool {
    pub fn new() -> Self {
        let pi_data = serde_json::from_str(&std::fs::read_to_string("data/pi_data.json").unwrap()).unwrap();
        PiTool { pi_data }
    }

    pub fn get_asset_info(&self, symbol: &str) -> Option<serde_json::Value> {
        self.pi_data.get("pi_assets").and_then(|assets| {
            assets.as_array().and_then(|assets| {
                assets.into_iter().find(|asset| asset.get("symbol").map_or(false, |symbol| symbol.as_str() == Some(symbol))).cloned()
            })
        })
    }

    pub fn get_network_info(&self, network: &str) -> Option<serde_json::Value> {
        self.pi_data.get("pi_network").and_then(|networks| {
            networks.get(network).cloned()
        })
    }

    pub fn fetch_api_data(&self, network: &str) -> Result<String, reqwest::Error> {
        let api_url = self.get_network_info(network).and_then(|network| network.get("api_url").map_or(None, |url| url.as_str()))?;
        get(api_url)?.text()
    }
}
