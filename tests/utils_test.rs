use scripts::utils;
use log::{info, error};

#[cfg(test)]
mod utils_tests {
    use super::*;

    #[test]
    fn test_get_stellar_asset() {
        let asset_code = "USD";
        let asset_info = utils::get_stellar_asset(asset_code);
        
        assert!(asset_info.is_some(), "Asset info for 'USD' should be present.");
        let asset_info = asset_info.unwrap();
        
        assert!(asset_info.contains("USD"), "Asset info should contain 'USD'.");
        info!("Successfully retrieved stellar asset info: {:?}", asset_info);
    }

    #[test]
    fn test_get_pi_asset() {
        let asset_symbol = "PI";
        let asset_info = utils::get_pi_asset(asset_symbol);
        
        assert!(asset_info.is_some(), "Asset info for 'PI' should be present.");
        let asset_info = asset_info.unwrap();
        
        assert!(asset_info.contains("PI"), "Asset info should contain 'PI'.");
        info!("Successfully retrieved Pi asset info: {:?}", asset_info);
    }

    #[test]
    fn test_get_stellar_network() {
        let network = "mainnet";
        let network_info = utils::get_stellar_network(network);
        
        assert!(network_info.is_some(), "Network info for 'mainnet' should be present.");
        let network_info = network_info.unwrap();
        
        assert!(network_info.contains("horizon.stellar.org"), "Network info should contain 'horizon.stellar.org'.");
        info!("Successfully retrieved stellar network info: {:?}", network_info);
    }

    #[test]
    fn test_get_pi_network() {
        let network = "mainnet";
        let network_info = utils::get_pi_network(network);
        
        assert!(network_info.is_some(), "Network info for 'mainnet' should be present.");
        let network_info = network_info.unwrap();
        
        assert!(network_info.contains("api.pi.network"), "Network info should contain 'api.pi.network'.");
        info!("Successfully retrieved Pi network info: {:?}", network_info);
    }

    #[test]
    fn test_get_nonexistent_stellar_asset() {
        let asset_code = "NON_EXISTENT";
        let asset_info = utils::get_stellar_asset(asset_code);
        
        assert!(asset_info.is_none(), "Asset info for 'NON_EXISTENT' should not be present.");
        info!("Correctly handled retrieval of nonexistent stellar asset info.");
    }

    #[test]
    fn test_get_invalid_pi_network() {
        let network = "invalid_network";
        let network_info = utils::get_pi_network(network);
        
        assert!(network_info.is_none(), "Network info for 'invalid_network' should not be present.");
        info!("Correctly handled retrieval of nonexistent Pi network info.");
    }
            }
