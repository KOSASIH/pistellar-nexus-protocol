use std::sync::{Arc, Mutex};

// Trait defining the Stellar Tool interface
pub trait StellarTool {
    fn new(stellar_protocol: Arc<Mutex<dyn StellarProtocol>>) -> Self;
    fn perform_stellar_operation(&self, data: Vec<u8>) -> Result<(), String>;
}

// Trait defining the Stellar Protocol interface
pub trait StellarProtocol: Send + 'static {  // Added Send + 'static for thread safety
    fn connect(&self) -> Result<(), String>;
    fn send_data(&self, data: Vec<u8>) -> Result<(), String>;
    fn receive_data(&self) -> Result<Vec<u8>, String>;
}

// Implementation of the Stellar Tool
pub struct StellarToolImpl {
    stellar_protocol: Arc<Mutex<dyn StellarProtocol>>,
}

impl StellarToolImpl {
    pub fn new(stellar_protocol: Arc<Mutex<dyn StellarProtocol>>) -> Self {
        Self { stellar_protocol }
    }
}

impl StellarTool for StellarToolImpl {
    fn new(stellar_protocol: Arc<Mutex<dyn StellarProtocol>>) -> Self {
        StellarToolImpl::new(stellar_protocol)
    }

    fn perform_stellar_operation(&self, data: Vec<u8>) -> Result<(), String> {
        // Lock the mutex and use the protocol
        let protocol = self.stellar_protocol.lock()
            .map_err(|e| format!("Failed to lock protocol: {}", e))?;
        
        // Perform actual operations
        protocol.connect()?;
        protocol.send_data(data)?;
        let _response = protocol.receive_data()?;  // Do something with response if needed
        
        println!("Stellar operation performed!");
        Ok(())
    }
}

// Example concrete implementation for testing
struct MockStellarProtocol;

impl StellarProtocol for MockStellarProtocol {
    fn connect(&self) -> Result<(), String> {
        Ok(())
    }
    
    fn send_data(&self, _data: Vec<u8>) -> Result<(), String> {
        Ok(())
    }
    
    fn receive_data(&self) -> Result<Vec<u8>, String> {
        Ok(vec![])
    }
}

// Example usage
fn main() {
    let protocol = Arc::new(Mutex::new(MockStellarProtocol));
    let tool = StellarToolImpl::new(protocol);
    let result = tool.perform_stellar_operation(vec![1, 2, 3]);
    match result {
        Ok(()) => println!("Operation successful"),
        Err(e) => println!("Operation failed: {}", e),
    }
        }
