import os
import re
import json
import logging
from dotenv import load_dotenv

# Configure logging
logging.basicConfig(level=logging.DEBUG)
logger = logging.getLogger(__name__)

class Config:
    def __init__(self, config_file='config.json'):
        # Load environment variables from .env file
        load_dotenv()
        self.config = self.load_config(config_file)
        self.validate_config()

    def load_config(self, config_file):
        """Load configuration from a JSON file and substitute environment variables."""
        with open(config_file, 'r') as f:
            config = json.load(f)
        return self.substitute_env_variables(config)

    def substitute_env_variables(self, config):
        """Recursively substitute environment variables in the config."""
        if isinstance(config, dict):
            return {key: self.substitute_env_variables(value) for key, value in config.items()}
        elif isinstance(config, list):
            return [self.substitute_env_variables(item) for item in config]
        elif isinstance(config, str):
            return self.replace_env_variable(config)
        return config

    def replace_env_variable(self, value):
        """Replace environment variable placeholders with actual values."""
        pattern = r'\${(.*?)}'
        matches = re.findall(pattern, value)
        for match in matches:
            env_value = os.getenv(match)
            if env_value is not None:
                value = value.replace(f'${{{match}}}', env_value)
            else:
                raise ValueError(f"Environment variable '{match}' not set.")
        return value

    def validate_config(self):
        """Validate required configuration settings."""
        required_keys = [
            "APP_NAME", "APP_VERSION", "ENVIRONMENT",
            "STELLAR_PROTOCOL_HOST", "STELLAR_PROTOCOL_PORT", "STELLAR_API_KEY",
            "PI_NETWORK_HOST", "PI_NETWORK_PORT", "PI_API_KEY",
            "CRYPTO_ALGORITHM", "CRYPTO_KEY", "NETWORK_SOCKET",
            "LOGGING_LEVEL", "LOGGING_OUTPUT"
        ]
        for key in required_keys:
            if os.getenv(key) is None:
                logger.error(f"Missing required environment variable: {key}")
                raise ValueError(f"Missing required environment variable: {key}")

    def get(self, key, default=None):
        """Get a configuration value by key."""
        keys = key.split('.')
        config_value = self.config
        for k in keys:
            config_value = config_value.get(k, default)
            if config_value is default:
                break
        return config_value

if __name__ == "__main__":
    # Load configuration
    config = Config()

    # Example usage
    logger.info("App Name: %s", config.get("app.name"))
    logger.info("Stellar Host: %s", config.get("stellar.protocol.host"))
    logger.info("Pi Network API Key: %s", config.get("pi.network.api_key"))
    logger.info("Logging Level: %s", config.get("logging.level"))
