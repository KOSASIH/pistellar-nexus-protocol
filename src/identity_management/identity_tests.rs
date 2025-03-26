#[cfg(test)]
mod tests {
    use super::*;
    use ed25519_dalek::{Keypair, Signature, Signer};
    use rand::{Rng, rngs::OsRng};
    use std::collections::HashMap;

    // Function to generate a random ID of a specified length
    fn generate_random_id(length: usize) -> String {
        let mut rng = OsRng {};
        let chars: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789".chars().collect();
        (0..length)
            .map(|_| chars[rng.gen_range(0..chars.len())])
            .collect()
    }

    #[test]
    fn test_create_identity() {
        let mut manager = IdentityManager::new();
        let metadata = HashMap::new();
        let identity = manager.create_identity("user1".to_string(), metadata.clone()).unwrap();
        assert_eq!(identity.id, "user1");
        assert!(!identity.public_key.is_empty());
    }

    #[test]
    fn test_verify_identity() {
        let mut manager = IdentityManager::new();
        let metadata = HashMap::new();
        let identity = manager.create_identity("user2".to_string(), metadata.clone()).unwrap();

        let keypair = Keypair::from_bytes(&identity.public_key).unwrap();
        let message = b"Hello, world!";
        let signature: Signature = keypair.sign(message);

        let is_valid = manager.verify_identity("user2", &signature, message).unwrap();
        assert!(is_valid);
    }

    #[test]
    fn test_revoke_identity() {
        let mut manager = IdentityManager::new();
        let metadata = HashMap::new();
        manager.create_identity("user3".to_string(), metadata.clone()).unwrap();
        assert!(manager.revoke_identity("user3", None).is_ok());
        assert!(manager.get_identity("user3").is_none());
    }

    #[test]
    fn test_random_id_generation() {
        let random_id = generate_random_id(10);
        assert_eq!(random_id.len(), 10);
        assert!(random_id.chars().all(|c| c.is_alphanumeric())); // Ensure all characters are alphanumeric
    }

    #[test]
    fn test_revoke_nonexistent_identity() {
        let mut manager = IdentityManager::new();
        let result = manager.revoke_identity("nonexistent", None);
        assert!(result.is_err());
    }

    #[test]
    fn test_verify_identity_with_invalid_signature() {
        let mut manager = IdentityManager::new();
        let metadata = HashMap::new();
        let identity = manager.create_identity("user4".to_string(), metadata.clone()).unwrap();

        let message = b"Hello, world!";
        let invalid_signature = Signature::default(); // Invalid signature for testing

        let is_valid = manager.verify_identity("user4", &invalid_signature, message).unwrap();
        assert!(!is_valid);
    }

    #[test]
    fn test_create_identity_with_random_id() {
        let mut manager = IdentityManager::new();
        let random_id = generate_random_id(12);
        let metadata = HashMap::new();
        let identity = manager.create_identity(random_id.clone(), metadata.clone()).unwrap();
        assert_eq!(identity.id, random_id);
        assert!(!identity.public_key.is_empty());
    }
        }
