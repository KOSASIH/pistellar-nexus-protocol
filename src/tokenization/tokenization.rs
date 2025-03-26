use std::collections::HashMap;
use uuid::Uuid;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Asset {
    pub id: Uuid,
    pub name: String,
    pub value: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Token {
    pub id: Uuid,
    pub asset_id: Uuid,
    pub owner: String,
    pub amount: f64,
}

pub struct TokenizationManager {
    assets: HashMap<Uuid, Asset>,
    tokens: HashMap<Uuid, Token>,
}

impl TokenizationManager {
    /// Creates a new instance of TokenizationManager.
    pub fn new() -> Self {
        TokenizationManager {
            assets: HashMap::new(),
            tokens: HashMap::new(),
        }
    }

    /// Creates a new asset and returns its ID.
    pub fn create_asset(&mut self, name: String, value: f64) -> Uuid {
        let asset = Asset {
            id: Uuid::new_v4(),
            name,
            value,
        };
        let asset_id = asset.id;
        self.assets.insert(asset_id, asset);
        asset_id
    }

    /// Creates a new token for a given asset and returns its ID.
    pub fn create_token(&mut self, asset_id: Uuid, owner: String, amount: f64) -> Result<Uuid, String> {
        if !self.assets.contains_key(&asset_id) {
            return Err("Asset does not exist.".to_string());
        }

        if amount <= 0.0 {
            return Err("Token amount must be greater than zero.".to_string());
        }

        let token = Token {
            id: Uuid::new_v4(),
            asset_id,
            owner,
            amount,
        };
        let token_id = token.id;
        self.tokens.insert(token_id, token);
        Ok(token_id)
    }

    /// Retrieves an asset by its ID.
    pub fn get_asset(&self, asset_id: Uuid) -> Option<&Asset> {
        self.assets.get(&asset_id)
    }

    /// Retrieves a token by its ID.
    pub fn get_token(&self, token_id: Uuid) -> Option<&Token> {
        self.tokens.get(&token_id)
    }

    /// Transfers ownership of a token to a new owner.
    pub fn transfer_token(&mut self, token_id: Uuid, new_owner: String) -> Result<(), String> {
        if let Some(token) = self.tokens.get_mut(&token_id) {
            if token.owner == new_owner {
                return Err("New owner must be different from the current owner.".to_string());
            }
            token.owner = new_owner;
            Ok(())
        } else {
            Err("Token does not exist.".to_string())
        }
    }

    /// Lists all assets in the system.
    pub fn list_assets(&self) -> Vec<&Asset> {
        self.assets.values().collect()
    }

    /// Lists all tokens in the system.
    pub fn list_tokens(&self) -> Vec<&Token> {
        self.tokens.values().collect()
    }
                        }
