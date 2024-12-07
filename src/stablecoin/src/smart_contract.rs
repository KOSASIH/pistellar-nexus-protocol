// src/smart_contract.rs
use std::collections::HashMap;
use crate::events::EventLogger;

pub struct SmartContract {
    balances: HashMap<String, u64>,
    total_supply: u64,
    pi_value: f64,
    event_logger: EventLogger,
}

impl SmartContract {
    pub fn new() -> Self {
        SmartContract {
            balances: HashMap::new(),
            total_supply: 0,
            pi_value: 314.159,
            event_logger: EventLogger::new(),
        }
    }

    pub fn mint(&mut self, user: String, amount: u64) {
        self.balances.insert(user.clone(), self.balances.get(&user).unwrap_or(&0) + amount);
        self.total_supply += amount;
        self.event_logger.log_event(format!("Minted {} Pi Coins for {}", amount, user));
    }

    pub fn burn(&mut self, user: String, amount: u64) {
        if let Some(balance) = self.balances.get_mut(&user) {
            if *balance >= amount {
                *balance -= amount;
                self.total_supply -= amount;
                self.event_logger.log_event(format!("Burned {} Pi Coins from {}", amount, user));
            } else {
                self.event_logger.log_event(format!("Insufficient balance to burn for {}", user));
            }
        }
    }

    pub fn transfer(&mut self, from: String, to: String, amount: u64) {
        if let Some(balance_from) = self.balances.get_mut(&from) {
            if *balance_from >= amount {
                *balance_from -= amount;
                let balance_to = self.balances.entry(to.clone()).or_insert(0);
                *balance_to += amount;
                self.event_logger.log_event(format!("Transferred {} Pi Coins from {} to {}", amount, from, to));
            } else {
                self.event_logger.log_event(format!("Insufficient balance to transfer from {}", from));
            }
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
}
