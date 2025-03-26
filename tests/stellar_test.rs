use stellar_tool::StellarTool;
use log::{info, error};

#[cfg(test)]
mod stellar_tests {
    use super::*;

    #[test]
    fn test_get_asset_info() {
        let stellar_tool = StellarTool::new().expect("Failed to initialize StellarTool");
        let asset_info = stellar_tool.get_asset_info("USD");
        
        assert!(asset_info.is_some(), "Asset info for 'USD' should be present.");
        let asset_info = asset_info.unwrap();
        
        assert_eq!(asset_info.get("code").unwrap().as_str().unwrap(), "USD");
        info!("Successfully retrieved asset info: {:?}", asset_info);
    }

    #[test]
    fn test_get_network_info() {
        let stellar_tool = StellarTool::new().expect("Failed to initialize StellarTool");
        let network_info = stellar_tool.get_network_info("mainnet");
        
        assert!(network_info.is_some(), "Network info for 'mainnet' should be present.");
        let network_info = network_info.unwrap();
        
        assert_eq!(network_info.get("horizon_url").unwrap().as_str().unwrap(), "https://horizon.stellar.org");
        info!("Successfully retrieved network info: {:?}", network_info);
    }

    #[test]
    fn test_fetch_horizon_data() {
        let stellar_tool = StellarTool::new().expect("Failed to initialize StellarTool");
        let horizon_data = stellar_tool.fetch_horizon_data("mainnet");
        
        assert!(horizon_data.is_ok(), "Fetching Horizon data should succeed.");
        let horizon_data = horizon_data.unwrap();
        
        assert!(horizon_data.contains("stellar"), "Horizon data should contain the string 'stellar'.");
        info!("Successfully fetched Horizon data: {}", horizon_data);
    }

    #[test]
    fn test_get_nonexistent_asset_info() {
        let stellar_tool = StellarTool::new().expect("Failed to initialize StellarTool");
        let asset_info = stellar_tool.get_asset_info("NON_EXISTENT");
        
        assert!(asset_info.is_none(), "Asset info for 'NON_EXISTENT' should not be present.");
        info!("Correctly handled retrieval of nonexistent asset info.");
    }

    #[test]
    fn test_get_invalid_network_info() {
        let stellar_tool = StellarTool::new().expect("Failed to initialize StellarTool");
        let network_info = stellar_tool.get_network_info("invalid_network");
        
        assert!(network_info.is_none(), "Network info for 'invalid_network' should not be present.");
        info!("Correctly handled retrieval of nonexistent network info.");
    }
        }
