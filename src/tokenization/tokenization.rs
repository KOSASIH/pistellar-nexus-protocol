// tokenization/tokenization.rs

use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Asset {
    pub id: Uuid,
    pub name: String,
    pub value: f64,
}

#[derive(Debug, Clone)]
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
    pub fn new() -> Self {
        TokenizationManager {
            assets: HashMap::new(),
            tokens: HashMap::new(),
        }
    }

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

    pub fn create_token(&mut self, asset_id: Uuid, owner: String, amount: f64) -> Result<Uuid, String> {
        if !self.assets.contains_key(&asset_id) {
            return Err("Asset does not exist.".to_string());
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

    pub fn get_asset(&self, asset_id: Uuid) -> Option<&Asset> {
        self.assets.get(&asset_id)
    }

    pub fn get_token(&self, token_id: Uuid) -> Option<&Token> {
        self.tokens.get(&token_id)
    }

    pub fn transfer_token(&mut self, token_id: Uuid, new_owner: String) -> Result<(), String> {
        if let Some(token) = self.tokens.get_mut(&token_id) {
            token.owner = new_owner;
            Ok(())
        } else {
            Err("Token does not exist.".to_string())
        }
    }
}
