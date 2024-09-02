use stellar_tool::StellarTool;

#[cfg(test)]
mod stellar_tests {
    use super::*;

    #[test]
    fn test_get_asset_info() {
        let stellar_tool = StellarTool::new();
        let asset_info = stellar_tool.get_asset_info("USD");
        assert!(asset_info.is_some());
        let asset_info = asset_info.unwrap();
        assert_eq!(asset_info.get("code").unwrap().as_str().unwrap(), "USD");
    }

    #[test]
    fn test_get_network_info() {
        let stellar_tool = StellarTool::new();
        let network_info = stellar_tool.get_network_info("mainnet");
        assert!(network_info.is_some());
        let network_info = network_info.unwrap();
        assert_eq!(network_info.get("horizon_url").unwrap().as_str().unwrap(), "https://horizon.stellar.org");
    }

    #[test]
    fn test_fetch_horizon_data() {
        let stellar_tool = StellarTool::new();
        let horizon_data = stellar_tool.fetch_horizon_data("mainnet");
        assert!(horizon_data.is_ok());
        let horizon_data = horizon_data.unwrap();
        assert!(horizon_data.contains("stellar"));
    }
}
