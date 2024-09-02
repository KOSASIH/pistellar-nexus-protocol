use scripts::utils;

#[cfg(test)]
mod utils_tests {
    use super::*;

    #[test]
    fn test_get_stellar_asset() {
        let asset_code = "USD";
        let asset_info = utils::get_stellar_asset(asset_code);
        assert!(asset_info.contains("USD"));
    }

    #[test]
    fn test_get_pi_asset() {
        let asset_symbol = "PI";
        let asset_info = utils::get_pi_asset(asset_symbol);
        assert!(asset_info.contains("PI"));
    }

    #[test]
    fn test_get_stellar_network() {
        let network = "mainnet";
        let network_info = utils::get_stellar_network(network);
        assert!(network_info.contains("horizon.stellar.org"));
    }

    #[test]
    fn test_get_pi_network() {
        let network = "mainnet";
        let network_info = utils::get_pi_network(network);
        assert!(network_info.contains("api.pi.network"));
    }
}
