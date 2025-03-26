use aes_gcm::{Aes128Gcm, Key, Nonce, aead::{Aead, NewAead}};
use rand::Rng;
use std::error::Error;

/// Encrypts the given data using AES-GCM with the provided key.
///
/// # Arguments
///
/// * `data` - The data to encrypt.
/// * `key` - The encryption key (must be 16 bytes for AES-128).
///
/// # Returns
///
/// A vector containing the nonce followed by the encrypted data.
pub fn encrypt(data: &[u8], key: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    if key.len() != 16 {
        return Err("Key must be 16 bytes long for AES-128.".into());
    }

    let cipher = Aes128Gcm::new(Key::from_slice(key));
    let mut nonce = [0u8; 12]; // AES-GCM nonce size is 12 bytes
    rand::thread_rng().fill(&mut nonce); // Generate a random nonce
    let nonce = Nonce::from_slice(&nonce); // Create a Nonce from the random bytes

    let encrypted_data = cipher.encrypt(nonce, data)
        .map_err(|_| "Encryption failed.".to_string())?;

    let mut result = Vec::new();
    result.extend_from_slice(&nonce);
    result.extend_from_slice(&encrypted_data);
    Ok(result)
}

/// Decrypts the given data using AES-GCM with the provided key.
///
/// # Arguments
///
/// * `data` - The data to decrypt (must include the nonce).
/// * `key` - The decryption key (must be 16 bytes for AES-128).
///
/// # Returns
///
/// The decrypted data.
pub fn decrypt(data: &[u8], key: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    if key.len() != 16 {
        return Err("Key must be 16 bytes long for AES-128.".into());
    }

    let cipher = Aes128Gcm::new(Key::from_slice(key));
    let nonce = Nonce::from_slice(&data[..12]); // Extract the nonce from the data
    let encrypted_data = &data[12..]; // The rest is the encrypted data

    let decrypted_data = cipher.decrypt(nonce, encrypted_data)
        .map_err(|_| "Decryption failed.".to_string())?;

    Ok(decrypted_data)
        }
