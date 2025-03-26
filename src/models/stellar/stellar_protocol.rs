/// Trait defining the Stellar Protocol interface
pub trait StellarProtocol {
    fn new() -> Self;
    fn get_stellar_data(&self) -> Vec<u8>;
    fn process_stellar_transaction(&mut self, transaction: Vec<u8>) -> Result<(), String>;
}

/// Implementation of the Stellar Protocol
pub struct StellarProtocolImpl {
    stellar_data: Vec<u8>,
}

impl StellarProtocolImpl {
    /// Creates a new instance of StellarProtocolImpl
    pub fn new() -> Self {
        Self { stellar_data: vec![] }
    }

    /// Adds data to the Stellar Protocol
    pub fn add_stellar_data(&mut self, data: Vec<u8>) {
        self.stellar_data.extend(data);
    }
}

impl StellarProtocol for StellarProtocolImpl {
    /// Retrieves the current Stellar data
    fn get_stellar_data(&self) -> Vec<u8> {
        self.stellar_data.clone()
    }

    /// Processes a Stellar transaction
    fn process_stellar_transaction(&mut self, transaction: Vec<u8>) -> Result<(), String> {
        // Validate the transaction data
        if transaction.is_empty() {
            return Err("Transaction data cannot be empty.".to_string());
        }

        // Process the Stellar transaction (placeholder for actual logic)
        println!("Processing Stellar transaction: {:?}", transaction);
        
        // Here you could add logic to update the state of the Stellar Protocol
        self.add_stellar_data(transaction); // Example of adding transaction data to the protocol

        println!("Stellar transaction processed successfully!");
        Ok(())
    }
}
