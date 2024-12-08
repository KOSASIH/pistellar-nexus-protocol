use aes_gcm::{Aes128Gcm, Key, Nonce};
use rand::Rng;

pub fn encrypt(data: &[u8], key: &[u8]) -> Vec<u8> {
    let cipher = Aes128Gcm::new(Key::from_slice(key));
    let nonce = rand::thread_rng().gen::<Nonce>();
    let encrypted_data = cipher.encrypt(nonce, data).unwrap();
    let mut result = Vec::new();
    result.extend_from_slice(&nonce);
    result.extend_from_slice(&encrypted_data);
    result
}

pub fn decrypt(data: &[u8], key: &[u8]) -> Vec<u8> {
    let cipher = Aes128Gcm::new(Key::from_slice(key));
    let nonce = Nonce::from_slice(&data[..12]);
    let encrypted_data = &data[12..];
    cipher.decrypt(nonce, encrypted_data).unwrap()
}
