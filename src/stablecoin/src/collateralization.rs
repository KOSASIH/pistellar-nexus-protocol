pub struct Collateralization {
    collateral: f64,
    stablecoin_value: f64,
    symbol: String,
    total_supply: u64,
}

impl Collateralization {
    const STABLECOIN_VALUE: f64 = 314159.00; // Target value of the stablecoin
    const SYMBOL: &str = "Pi"; // Symbol for the Pi Coin
    const TOTAL_SUPPLY: u64 = 100_000_000_000; // Total supply of the Pi Coin

    pub fn new() -> Self {
        Collateralization {
            collateral: 0.0,
            stablecoin_value: Self::STABLECOIN_VALUE,
            symbol: Self::SYMBOL.to_string(),
            total_supply: Self::TOTAL_SUPPLY,
        }
    }

    pub fn add_collateral(&mut self, amount: f64) -> Result<(), String> {
        if amount < 0.0 {
            return Err("Cannot add negative collateral".to_string());
        }
        self.collateral += amount;
        Ok(())
    }

    pub fn remove_collateral(&mut self, amount: f64) -> Result<(), String> {
        if amount < 0.0 {
            return Err("Cannot remove negative collateral".to_string());
        }
        if amount > self.collateral {
            return Err("Insufficient collateral to remove".to_string());
        }
        self.collateral -= amount;
        Ok(())
    }

    pub fn check_collateralization(&self) -> bool {
        self.collateral >= self.stablecoin_value
    }

    pub fn get_collateral(&self) -> f64 {
        self.collateral
    }

    pub fn get_stablecoin_value(&self) -> f64 {
        self.stablecoin_value
    }

    pub fn get_symbol(&self) -> String {
        self.symbol.clone()
    }

    pub fn get_total_supply(&self) -> u64 {
        self.total_supply
    }

    pub fn collateral_ratio(&self) -> f64 {
        if self.stablecoin_value == 0.0 {
            return 0.0; // Avoid division by zero
        }
        self.collateral / self.stablecoin_value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_collateral() {
        let mut collateralization = Collateralization::new();
        assert!(collateralization.add_collateral(500.0).is_ok());
        assert_eq!(collateralization.get_collateral(), 500.0);
    }

    #[test]
    fn test_remove_collateral() {
        let mut collateralization = Collateralization::new();
        collateralization.add_collateral(500.0).unwrap();
        assert!(collateralization.remove_collateral(200.0).is_ok());
        assert_eq!(collateralization.get_collateral(), 300.0);
    }

    #[test]
    fn test_remove_insufficient_collateral() {
        let mut collateralization = Collateralization::new();
        collateralization.add_collateral(100.0).unwrap();
        assert!(collateralization.remove_collateral(150.0).is_err());
    }

    #[test]
    fn test_check_collateralization() {
        let mut collateralization = Collateralization::new();
        collateralization.add_collateral(500.0).unwrap();
        assert!(collateralization.check_collateralization());
        
        collateralization.remove_collateral(200.0).unwrap();
        assert!(collateralization.check_collateralization());
        
        collateralization.remove_collateral(300.0).unwrap();
        assert!(!collateralization.check_collateralization());
    }

    #[test]
    fn test_collateral_ratio() {
        let mut collateralization = Collateralization::new();
        collateralization.add_collateral(500.0).unwrap();
        assert_eq!(collateralization.collateral_ratio(), 1.5874010519681994); // 500 / 314.159
    }

    #[test]
    fn test_symbol() {
        let collateralization = Collateralization::new();
        assert_eq!(collateralization.get_symbol(), "Pi");
    }

    #[test]
    fn test_total_supply() {
        let collateralization = Collateralization::new();
        assert_eq!(collateralization.get_total_supply(), 100_000_000_000);
    }
        }
