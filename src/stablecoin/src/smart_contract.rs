use std::collections::HashMap;
use crate::events::EventLogger;

pub struct SmartContract {
    balances: HashMap<String, u64>,
    total_supply: u64,
    pi_value: f64,
    event_logger: EventLogger,
}

impl SmartContract {
    const INITIAL_PI_VALUE: f64 = 314159.00; // Set the initial value of Pi Coin

    pub fn new() -> Self {
        SmartContract {
            balances: HashMap::new(),
            total_supply: 0,
            pi_value: Self::INITIAL_PI_VALUE,
            event_logger: EventLogger::new(),
        }
    }

    pub fn mint(&mut self, user: String, amount: u64) -> Result<(), String> {
        if amount == 0 {
            return Err("Cannot mint zero coins".to_string());
        }
        self.balances.insert(user.clone(), self.balances.get(&user).unwrap_or(&0) + amount);
        self.total_supply += amount;
        self.event_logger.log_event(format!("Minted {} Pi Coins for {}", amount, user));
        Ok(())
    }

    pub fn burn(&mut self, user: String, amount: u64) -> Result<(), String> {
        if let Some(balance) = self.balances.get_mut(&user) {
            if *balance >= amount {
                *balance -= amount;
                self.total_supply -= amount;
                self.event_logger.log_event(format!("Burned {} Pi Coins from {}", amount, user));
                Ok(())
            } else {
                self.event_logger.log_event(format!("Insufficient balance to burn for {}", user));
                Err("Insufficient balance".to_string())
            }
        } else {
            Err("User  not found".to_string())
        }
    }

    pub fn transfer(&mut self, from: String, to: String, amount: u64) -> Result<(), String> {
        if let Some(balance_from) = self.balances.get_mut(&from) {
            if *balance_from >= amount {
                *balance_from -= amount;
                let balance_to = self.balances.entry(to.clone()).or_insert(0);
                *balance_to += amount;
                self.event_logger.log_event(format!("Transferred {} Pi Coins from {} to {}", amount, from, to));
                Ok(())
            } else {
                self.event_logger.log_event(format!("Insufficient balance to transfer from {}", from));
                Err("Insufficient balance".to_string())
            }
        } else {
            Err("Sender not found".to_string())
        }
    }

    pub fn get_balance(&self, user: &str) -> u64 {
        *self.balances.get(user).unwrap_or(&0)
    }

    pub fn get_total_supply(&self) -> u64 {
        self.total_supply
    }

    pub fn get_pi_value(&self) -> f64 {
        self.pi_value
    }

    pub fn get_event_log(&self) -> Vec<String> {
        self.event_logger.get_events().iter().map(|event| event.message.clone()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mint() {
        let mut contract = SmartContract::new();
        assert!(contract.mint("user1".to_string(), 100).is_ok());
        assert_eq!(contract.get_balance("user1"), 100);
        assert_eq!(contract.get_total_supply(), 100);
    }

    #[test]
    fn test_burn() {
        let mut contract = SmartContract::new();
        contract.mint("user1".to_string(), 100).unwrap();
        assert!(contract.burn("user1".to_string(), 50).is_ok());
        assert_eq!(contract.get_balance("user1"), 50);
        assert_eq!(contract.get_total_supply(), 50);
    }

    #[test]
    fn test_transfer() {
        let mut contract = SmartContract::new();
        contract.mint("user1".to_string(), 100).unwrap();
        assert!(contract.transfer("user1".to_string(), "user2".to_string(), 50).is_ok());
        assert_eq!(contract.get_balance("user1"), 50);
        assert_eq!(contract.get_balance("user2"), 50);
    }

    #[test]
    fn test_insufficient_burn() {
        let mut contract = SmartContract::new();
        contract.mint("user1".to_string(), 100).unwrap();
        assert!(contract.burn("user1".to_string(), 150).is_err());
    }

    #[test]
    fn test_insufficient_transfer() {
        let mut contract = SmartContract::new();
        contract.mint("user1".to_string(), 100).unwrap();
        assert!(contract.transfer("user1".to_string(), "user2".to_string(), 150).is_err());
    }
    }
