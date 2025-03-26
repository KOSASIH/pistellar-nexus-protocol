import json
import logging

# Configure logging
logging.basicConfig(level=logging.DEBUG)
logger = logging.getLogger(__name__)

class Config:
    def __init__(self, config_file='config.json'):
        self.config = self.load_config(config_file)
        self.validate_config()

    def load_config(self, config_file):
        """Load configuration from a JSON file."""
        with open(config_file, 'r') as f:
            config = json.load(f)
        return config

    def validate_config(self):
        """Validate required configuration settings."""
        required_keys = [
            "pi_network.mainnet.api_url", "pi_network.mainnet.chain_id",
            "pi_network.testnet.api_url", "pi_network.testnet.chain_id",
            "pi_assets"
        ]
        for key in required_keys:
            if self.get(key) is None:
                logger.error(f"Missing required configuration key: {key}")
                raise ValueError(f"Missing required configuration key: {key}")

    def get(self, key, default=None):
        """Get a configuration value by key."""
        keys = key.split('.')
        config_value = self.config
        for k in keys:
            config_value = config_value.get(k, default)
            if config_value is default:
                break
        return config_value

    def list_assets(self):
        """List all assets in the configuration."""
        return self.get("pi_assets", [])

if __name__ == "__main__":
    # Load configuration
    config = Config()

    # Example usage
    logger.info("Mainnet API URL: %s", config.get("pi_network.mainnet.api_url"))
    logger.info("Testnet API URL: %s", config.get("pi_network.testnet.api_url"))
    logger.info("Assets: %s", config.list_assets())
