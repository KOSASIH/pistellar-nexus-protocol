#[cfg(test)]
mod tests {
    use super::*;
    use ed25519_dalek::{Keypair, Signature, Signer};
    use rand::Rng; // Import Rng for random number generation
    use std::collections::HashMap;

    #[test]
    fn test_multi_sig_wallet() {
        // Generate keypairs for the wallet
        let keypair1 = Keypair::generate(&mut rand::thread_rng());
        let keypair2 = Keypair::generate(&mut rand::thread_rng());

        // Create a MultiSigWallet with a threshold of 2
        let mut wallet = MultiSigWallet::new(2, vec![
            keypair1.public,
            keypair2.public,
        ]);

        let message = b"Hello, world!";

        // Sign the message with the first keypair
        let signature1: Signature = keypair1.sign(message);
        assert!(wallet.add_signature(keypair1.public, signature1).is_ok());
        assert!(!wallet.verify(message), "Wallet should not verify with only one signature.");

        // Sign the message with the second keypair
        let signature2: Signature = keypair2.sign(message);
        assert!(wallet.add_signature(keypair2.public, signature2).is_ok());
        assert!(wallet.verify(message), "Wallet should verify with two valid signatures.");
    }

    #[test]
    fn test_add_duplicate_signature() {
        let keypair = Keypair::generate(&mut rand::thread_rng());
        let mut wallet = MultiSigWallet::new(2, vec![keypair.public]);

        let message = b"Hello, world!";
        let signature: Signature = keypair.sign(message);

        // Add the signature once
        assert!(wallet.add_signature(keypair.public, signature.clone()).is_ok());
        // Attempt to add the same signature again
        assert!(wallet.add_signature(keypair.public, signature).is_err(), "Should not allow duplicate signatures.");
    }

    #[test]
    fn test_encryption() {
        // Generate a random key for encryption
        let key: [u8; 32] = rand::thread_rng().gen();
        let data = b"Hello, world!";
        
        // Encrypt the data
        let encrypted_data = encrypt(data, &key).expect("Encryption failed");
        
        // Decrypt the data
        let decrypted_data = decrypt(&encrypted_data, &key).expect("Decryption failed");
        
        // Assert that the decrypted data matches the original data
        assert_eq!(data, &decrypted_data[..]);
    }

    #[test]
    fn test_encryption_with_invalid_key() {
        let key1: [u8; 32] = rand::thread_rng().gen();
        let key2: [u8; 32] = rand::thread_rng().gen(); // Different key for testing
        let data = b"Hello, world!";
        
        // Encrypt the data with the first key
        let encrypted_data = encrypt(data, &key1).expect("Encryption failed");
        
        // Attempt to decrypt with a different key
        let result = decrypt(&encrypted_data, &key2);
        assert!(result.is_err(), "Decryption should fail with an invalid key.");
    }
    }
