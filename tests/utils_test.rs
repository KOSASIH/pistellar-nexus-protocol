// Unit tests for the utility functions

use super::utils::crypto::CryptoUtilsImpl;
use super::utils::network::NetworkUtilsImpl;

#[test]
fn test_encrypt_data() {
    let crypto_utils = CryptoUtilsImpl::new();
    let data = vec![1, 2, 3, 4, 5];
    let encrypted_data = crypto_utils.encrypt_data(data);
    assert_ne!(encrypted_data, data);
}

#[test]
fn test_decrypt_data() {
    let crypto_utils = CryptoUtilsImpl::new();
    let data = vec![1, 2, 3, 4, 5];
    let encrypted_data = crypto_utils.encrypt_data(data);
    let decrypted_data = crypto_utils.decrypt_data(encrypted_data);
    assert_eq!(decrypted_data, data);
}

#[test]
fn test_send_data() {
    let network_utils = NetworkUtilsImpl::new();
    let data = vec![1, 2, 3, 4, 5];
    network_utils.send_data(data).unwrap();
}

#[test]
fn test_receive_data() {
    let network_utils = NetworkUtilsImpl::new();
    let data = network_utils.receive_data();
    assert_eq!(data, vec![]);
}
