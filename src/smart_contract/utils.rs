pub fn calculate_fee(amount: Balance) -> Balance {
    // Example fee calculation: 1% of the transaction amount
    amount / 100
}

pub fn log_event(event: &str) {
    // Log an event to the blockchain
    ink_env::debug_println!("{}", event);
}
