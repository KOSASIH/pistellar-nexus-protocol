use std::collections::HashMap;
use rust_decimal::Decimal;
use chrono::Utc;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PiCoin {
    // Core Stablecoin Parameters
    target_value: Decimal,     // Target value of $314,159
    current_value: Decimal,
    total_supply: u64,
    
    // Stabilization Mechanisms
    reserve_backing: Decimal,
    algorithmic_adjustment_factor: f64,
    
    // Governance & Compliance
    governance_parameters: HashMap<String, String>,
    compliance_checks: Vec<ComplianceRule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum ComplianceRule {
    MinimumReserveRatio(f64),
    MaximumSupplyLimit(u64),
    StabilityWindowCheck(u64),
}

impl PiCoin {
    const TARGET_PRICE: Decimal = Decimal::from_f64(314159.00).unwrap(); // Set target price to $314,159
    const MAX_SUPPLY: u64 = 100_000_000_000; // Total supply of Pi Coin

    pub fn new(initial_supply: u64) -> Self {
        Self {
            target_value: Self::TARGET_PRICE,
            current_value: Self::TARGET_PRICE,
            total_supply: initial_supply,
            reserve_backing: Decimal::from_f64(initial_supply as f64 * 314159.00).unwrap(),
            algorithmic_adjustment_factor: 1.0,
            governance_parameters: HashMap::new(),
            compliance_checks: vec![
                ComplianceRule::MinimumReserveRatio(0.5),
                ComplianceRule::MaximumSupplyLimit(Self::MAX_SUPPLY),
            ],
        }
    }

    // Advanced Stabilization Algorithm
    pub fn stabilize_value(&mut self) {
        let market_deviation = self.calculate_market_deviation();
        
        if market_deviation > Decimal::from_f64(0.01).unwrap() {
            // Expansionary mechanism
            self.algorithmic_adjustment_factor *= 0.99;
            self.current_value *= Decimal::from_f64(0.99).unwrap();
        } else if market_deviation < Decimal::from_f64(-0.01).unwrap() {
            // Contractionary mechanism
            self.algorithmic_adjustment_factor *= 1.01;
            self.current_value *= Decimal::from_f64(1.01).unwrap();
        }

        self.validate_compliance_rules().expect("Compliance check failed");
    }

    fn calculate_market_deviation(&self) -> Decimal {
        (self.current_value - self.target_value) / self.target_value
    }

    fn validate_compliance_rules(&self) -> Result<(), String> {
        for rule in &self.compliance_checks {
            match rule {
                ComplianceRule::MinimumReserveRatio(min_ratio) => {
                    let current_ratio = self.reserve_backing.to_f64().unwrap() / 
                                        (self.total_supply as f64 * 314159.00);
                    if current_ratio < *min_ratio {
                        return Err("Reserve ratio violation".to_string());
                    }
                }
                ComplianceRule::MaximumSupplyLimit(max_supply) => {
                    if self.total_supply > *max_supply {
                        return Err("Maximum supply limit exceeded".to_string());
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }

    // Oracle-like price feed mechanism
    pub fn get_current_price(&self) -> Decimal {
        self.current_value
    }

    // Economic expansion/contraction
    pub fn mint(&mut self, amount: u64) -> Result<(), String> {
        if self.total_supply + amount > Self::MAX_SUPPLY {
            return Err("Mint would exceed maximum supply".to_string());
        }
        
        self.total_supply += amount;
        self.reserve_backing += Decimal::from_f64(amount as f64 * 314159.00).unwrap();
        
        Ok(())
    }

    pub fn burn(&mut self, amount: u64) -> Result<(), String> {
        if amount > self.total_supply {
            return Err("Burn amount exceeds total supply".to_string());
        }
        
        self.total_supply -= amount;
        self.reserve_backing -= Decimal::from_f64(amount as f64 * 314159.00).unwrap();
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pi_coin_initialization() {
        let pi_coin = PiCoin::new(100_000);
        
        assert_eq!(pi_coin.current_value, PiCoin::TARGET_PRICE);
        assert_eq!(pi_coin.total_supply, 100_000);
    }

    #[test]
    fn test_stabilization_mechanism() {
        let mut pi_coin = PiCoin::new(100_000);
        pi_coin.current_value *= Decimal::from_f64(1.02).unwrap(); // Simulate market deviation
        pi_coin.stabilize_value();
        
        // Verify stabilization logic
        assert!(pi_coin.current_value.abs_diff(PiCoin::TARGET_PRICE) < 
                Decimal::from_f64(1.0).unwrap());
    }

    #[test]
    fn test_minting() {
        let mut pi_coin = PiCoin::new(100_000);
        let result = pi_coin.mint(50_000);
        
        assert!(result.is_ok());
        assert_eq!(pi_coin.total_supply, 150_000);
    }

    #[test]
    fn test_burning() {
        let mut pi_coin = PiCoin::new(100_000);
        let result = pi_coin.burn(50_000);
        
        assert!(result.is_ok());
        assert_eq!(pi_coin.total_supply, 50_000);
    }

    #[test]
    fn test_compliance_check() {
        let mut pi_coin = PiCoin::new(100_000);
        pi_coin.reserve_backing = Decimal::from_f64(10_000_000_000.0).unwrap(); // Set a high reserve
        
        let result = pi_coin.validate_compliance_rules();
        assert!(result.is_ok());
    }

    #[test]
    fn test_exceeding_supply_limit() {
        let mut pi_coin = PiCoin::new(PiCoin::MAX_SUPPLY);
        let result = pi_coin.mint(1);
        
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Mint would exceed maximum supply");
    }
        }
