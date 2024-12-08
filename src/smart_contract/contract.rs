// smart_contract/contract.rs

use serde::{Deserialize, Serialize};
use ink_lang as ink;

#[ink::contract]
mod contract {
    use super::*;

    #[derive(Default, Serialize, Deserialize)]
    pub struct Asset {
        pub id: u32,
        pub owner: AccountId,
        pub value: u128,
        pub metadata: String,
    }

    #[ink(storage)]
    pub struct SmartContract {
        assets: ink_storage::collections::HashMap<u32, Asset>,
        total_assets: u32,
    }

    impl SmartContract {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                assets: ink_storage::collections::HashMap::new(),
                total_assets: 0,
            }
        }

        #[ink(message)]
        pub fn create_asset(&mut self, value: u128, metadata: String) -> u32 {
            self.total_assets += 1;
            let asset = Asset {
                id: self.total_assets,
                owner: Self::env().caller(),
                value,
                metadata,
            };
            self.assets.insert(self.total_assets, asset);
            self.total_assets
        }

        #[ink(message)]
        pub fn transfer_asset(&mut self, asset_id: u32, new_owner: AccountId) -> Result<(), &'static str> {
            let asset = self.assets.get_mut(&asset_id).ok_or("Asset not found")?;
            if asset.owner != Self::env().caller() {
                return Err("Only the owner can transfer the asset");
            }
            asset.owner = new_owner;
            Ok(())
        }

        #[ink(message)]
        pub fn get_asset(&self, asset_id: u32) -> Option<Asset> {
            self.assets.get(&asset_id).cloned()
        }

        #[ink(message)]
        pub fn get_total_assets(&self) -> u32 {
            self.total_assets
        }
    }
}
