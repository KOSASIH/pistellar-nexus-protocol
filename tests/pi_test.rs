use pi_tool::PiTool;
use log::{info, error};

#[cfg(test)]
mod pi_tests {
    use super::*;

    #[test]
    fn test_get_asset_info() {
        let pi_tool = PiTool::new().expect("Failed to initialize PiTool");
        let asset_info = pi_tool.get_asset_info("PI");
        
        assert!(asset_info.is_some(), "Asset info for 'PI' should be present.");
        let asset_info = asset_info.unwrap();
        
        assert_eq!(asset_info.get("symbol").unwrap().as_str().unwrap(), "PI");
        info!("Successfully retrieved asset info: {:?}", asset_info);
    }

    #[test]
    fn test_get_network_info() {
        let pi_tool = PiTool::new().expect("Failed to initialize PiTool");
        let network_info = pi_tool.get_network_info("mainnet");
        
        assert!(network_info.is_some(), "Network info for 'mainnet' should be present.");
        let network_info = network_info.unwrap();
        
        assert_eq!(network_info.get("api_url").unwrap().as_str().unwrap(), "https://api.pi.network");
        info!("Successfully retrieved network info: {:?}", network_info);
    }

    #[test]
    fn test_fetch_api_data() {
        let pi_tool = PiTool::new().expect("Failed to initialize PiTool");
        let api_data = pi_tool.fetch_api_data("mainnet");
        
        assert!(api_data.is_ok(), "Fetching API data should succeed.");
        let api_data = api_data.unwrap();
        
        assert!(api_data.contains("pi"), "API data should contain the string 'pi'.");
        info!("Successfully fetched API data: {}", api_data);
    }

    #[test]
    fn test_get_nonexistent_asset_info() {
        let pi_tool = PiTool::new().expect("Failed to initialize PiTool");
        let asset_info = pi_tool.get_asset_info("NON_EXISTENT");
        
        assert!(asset_info.is_none(), "Asset info for 'NON_EXISTENT' should not be present.");
        info!("Correctly handled retrieval of nonexistent asset info.");
    }

    #[test]
    fn test_get_invalid_network_info() {
        let pi_tool = PiTool::new().expect("Failed to initialize PiTool");
        let network_info = pi_tool.get_network_info("invalid_network");
        
        assert!(network_info.is_none(), "Network info for 'invalid_network' should not be present.");
        info!("Correctly handled retrieval of nonexistent network info.");
    }
    }
