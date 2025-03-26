use ink_lang as ink;

#[ink::contract]
mod contract {
    use ink_storage::collections::HashMap as StorageMap;

    #[ink(storage)]
    pub struct MyContract {
        owner: AccountId,
        balances: StorageMap<AccountId, Balance>,
    }

    impl MyContract {
        #[ink(constructor)]
        pub fn new() -> Self {
            let caller = Self::env().caller();
            Self {
                owner: caller,
                balances: StorageMap::new(),
            }
        }

        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, value: Balance) -> Result<(), &'static str> {
            let caller = self.env().caller();
            let caller_balance = self.balances.get(&caller).unwrap_or(0);
            if caller_balance < value {
                return Err("Insufficient balance");
            }
            self.balances.insert(caller, caller_balance - value);
            let recipient_balance = self.balances.get(&to).unwrap_or(0);
            self.balances.insert(to, recipient_balance + value);
            Ok(())
        }

        #[ink(message)]
        pub fn balance_of(&self, account: AccountId) -> Balance {
            self.balances.get(&account).unwrap_or(0)
        }

        #[ink(message)]
        pub fn owner(&self) -> AccountId {
            self.owner
        }
    }
            }
