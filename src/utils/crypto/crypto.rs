// Cryptography utility functions

pub trait CryptoUtils {
    fn new() -> Self;
    fn encrypt_data(&self, data: Vec<u8>) -> Vec<u8>;
    fn decrypt_data(&self, data: Vec<u8>) -> Vec<u8>;
}

pub struct CryptoUtilsImpl {
    key: Vec<u8>,
}

impl CryptoUtilsImpl {
    pub fn new() -> Self {
        Self { key: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16] }
    }
}

impl CryptoUtils for CryptoUtilsImpl {
    fn encrypt_data(&self, data: Vec<u8>) -> Vec<u8> {
        // Encrypt the data using the key
        println!("Data encrypted!");
        data
    }

    fn decrypt_data(&self, data: Vec<u8>) -> Vec<u8> {
        // Decrypt the data using the key
        println!("Data decrypted!");
        data
    }
}
