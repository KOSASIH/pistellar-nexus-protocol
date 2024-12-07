// src/utils.rs
pub fn validate_address(address: &str) -> bool {
    // Validate Stellar address
    address.len() == 56 && address.starts_with("G")
}
