import json
import os
import re

class Config:
    def __init__(self, config_file='config.json'):
        self.config = self.load_config(config_file)

    def load_config(self, config_file):
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
    print("App Name:", config.get("app.name"))
    print("Stellar Host:", config.get("stellar.protocol.host"))
    print("Pi Network API Key:", config.get("pi.network.api_key"))
    print("Logging Level:", config.get("logging.level"))
