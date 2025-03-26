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
    /// Creates a new instance of StellarToolImpl
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
        protocol.connect().map_err(|e| format!("Failed to connect: {}", e))?;
        protocol.send_data(data.clone()).map_err(|e| format!("Failed to send data: {}", e))?;
        
        let response = protocol.receive_data().map_err(|e| format!("Failed to receive data: {}", e))?;  // Handle response if needed
        
        println!("Stellar operation performed! Response: {:?}", response);
        Ok(())
    }
}

// Example concrete implementation for testing
struct MockStellarProtocol;

impl StellarProtocol for MockStellarProtocol {
    fn connect(&self) -> Result<(), String> {
        println!("Mock connection established.");
        Ok(())
    }
    
    fn send_data(&self, _data: Vec<u8>) -> Result<(), String> {
        println!("Mock data sent.");
        Ok(())
    }
    
    fn receive_data(&self) -> Result<Vec<u8>, String> {
        println!("Mock data received.");
        Ok(vec![1, 2, 3])  // Simulated response
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
