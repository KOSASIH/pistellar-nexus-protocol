// src/utils.rs
pub fn validate_address(address: &str) -> bool {
    // Simple validation logic for addresses
    address.len() == 42 && address.starts_with("0x")
}
