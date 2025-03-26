#[cfg(test)]
mod tests {
    use super::super::tokenization::{TokenizationManager, Uuid};

    #[test]
    fn test_create_asset() {
        let mut manager = TokenizationManager::new();
        let asset_id = manager.create_asset("Gold".to_string(), 1500.0);
        let asset = manager.get_asset(asset_id).unwrap();
        assert_eq!(asset.name, "Gold");
        assert_eq!(asset.value, 1500.0);
    }

    #[test]
    fn test_create_token() {
        let mut manager = TokenizationManager::new();
        let asset_id = manager.create_asset("Gold".to_string(), 1500.0);
        let token_id = manager.create_token(asset_id, "Alice".to_string(), 1.0).unwrap();
        let token = manager.get_token(token_id).unwrap();
        assert_eq!(token.owner, "Alice");
        assert_eq!(token.amount, 1.0);
    }

    #[test]
    fn test_create_token_for_nonexistent_asset() {
        let mut manager = TokenizationManager::new();
        let result = manager.create_token(Uuid::new_v4(), "Alice".to_string(), 1.0);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Asset does not exist.");
    }

    #[test]
    fn test_transfer_token() {
        let mut manager = TokenizationManager::new();
        let asset_id = manager.create_asset("Gold".to_string(), 1500.0);
        let token_id = manager.create_token(asset_id, "Alice".to_string(), 1.0).unwrap();
        
        // Transfer token from Alice to Bob
        let transfer_result = manager.transfer_token(token_id, "Bob".to_string());
        assert!(transfer_result.is_ok());

        let token = manager.get_token(token_id).unwrap();
        assert_eq!(token.owner, "Bob");
    }

    #[test]
    fn test_transfer_nonexistent_token() {
        let mut manager = TokenizationManager::new();
        let result = manager.transfer_token(Uuid::new_v4(), "Bob".to_string());
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Token does not exist.");
    }

    #[test]
    fn test_get_nonexistent_asset() {
        let manager = TokenizationManager::new();
        let asset = manager.get_asset(Uuid::new_v4());
        assert!(asset.is_none());
    }

    #[test]
    fn test_get_nonexistent_token() {
        let manager = TokenizationManager::new();
        let token = manager.get_token(Uuid::new_v4());
        assert!(token.is_none());
    }

    #[test]
    fn test_create_token_with_zero_amount() {
        let mut manager = TokenizationManager::new();
        let asset_id = manager.create_asset("Gold".to_string(), 1500.0);
        let result = manager.create_token(asset_id, "Alice".to_string(), 0.0);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Token amount must be greater than zero.");
    }

    #[test]
    fn test_transfer_token_to_same_owner() {
        let mut manager = TokenizationManager::new();
        let asset_id = manager.create_asset("Gold".to_string(), 1500.0);
        let token_id = manager.create_token(asset_id, "Alice".to_string(), 1.0).unwrap();
        
        // Attempt to transfer token to the same owner
        let transfer_result = manager.transfer_token(token_id, "Alice".to_string());
        assert!(transfer_result.is_err());
        assert_eq!(transfer_result.unwrap_err(), "New owner must be different from the current owner.");
    }
            }
