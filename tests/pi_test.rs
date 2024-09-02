use pi_tool::PiTool;

#[cfg(test)]
mod pi_tests {
    use super::*;

    #[test]
    fn test_get_asset_info() {
        let pi_tool = PiTool::new();
        let asset_info = pi_tool.get_asset_info("PI");
        assert!(asset_info.is_some());
        let asset_info = asset_info.unwrap();
        assert_eq!(asset_info.get("symbol").unwrap().as_str().unwrap(), "PI");
    }

    #[test]
    fn test_get_network_info() {
        let pi_tool = PiTool::new();
        let network_info = pi_tool.get_network_info("mainnet");
        assert!(network_info.is_some());
        let network_info = network_info.unwrap();
        assert_eq!(network_info.get("api_url").unwrap().as_str().unwrap(), "https://api.pi.network");
    }

    #[test]
    fn test_fetch_api_data() {
        let pi_tool = PiTool::new();
        let api_data = pi_tool.fetch_api_data("mainnet");
        assert!(api_data.is_ok());
        let api_data = api_data.unwrap();
        assert!(api_data.contains("pi"));
    }
}
