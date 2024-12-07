// src/collateralization.rs
pub struct Collateralization {
    collateral: f64,
    stablecoin_value: f64,
}

impl Collateralization {
    pub fn new() -> Self {
        Collateralization {
            collateral: 0.0,
            stablecoin_value: 314.159,
        }
    }

    pub fn add_collateral(&mut self, amount: f64) {
        self.collateral += amount;
    }

    pub fn check_collateralization(&self) -> bool {
        self.collateral >= self.stablecoin_value
    }
}
