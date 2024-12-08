#[cfg(test)]
mod tests {
    use super::*;
    use ed25519_dalek::{Keypair, Signature, Signer};
    use std::collections::HashMap;

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
        assert!(manager.revoke_identity("user3").is_ok());
        assert!(manager.get_identity("user3").is_none());
    }

    #[test]
    fn test_random_id_generation() {
        let random_id = generate_random_id(10);
        assert_eq!(random_id.len(), 10);
    }
}
