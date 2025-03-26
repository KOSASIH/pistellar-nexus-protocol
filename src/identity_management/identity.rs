use serde::{Deserialize, Serialize};
use ed25519_dalek::{Keypair, Signature, Signer, Verifier};
use rand::rngs::OsRng;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum IdentityError {
    NotFound(String),
    SigningError(String),
    VerificationError(String),
    Other(String),
}

impl fmt::Display for IdentityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for IdentityError {}

#[derive(Serialize, Deserialize, Clone)]
pub struct Identity {
    pub id: String,
    pub public_key: Vec<u8>,
    pub metadata: HashMap<String, String>,
}

pub struct IdentityManager {
    identities: HashMap<String, (Identity, Keypair)>, // Store keypair alongside identity
}

impl IdentityManager {
    pub fn new() -> Self {
        Self {
            identities: HashMap::new(),
        }
    }

    pub fn create_identity(&mut self, id: String, metadata: HashMap<String, String>) -> Result<Identity, Box<dyn Error>> {
        let mut csprng = OsRng{};
        let keypair = Keypair::generate(&mut csprng);
        let identity = Identity {
            id: id.clone(),
            public_key: keypair.public.to_bytes().to_vec(),
            metadata,
        };
        self.identities.insert(id.clone(), (identity.clone(), keypair));
        Ok(identity)
    }

    pub fn sign_message(&self, id: &str, message: &[u8]) -> Result<Signature, Box<dyn Error>> {
        if let Some((_, keypair)) = self.identities.get(id) {
            Ok(keypair.sign(message))
        } else {
            Err(Box::new(IdentityError::NotFound(id.to_string())))
        }
    }

    pub fn verify_identity(&self, id: &str, signature: &Signature, message: &[u8]) -> Result<bool, Box<dyn Error>> {
        if let Some((identity, _)) = self.identities.get(id) {
            let public_key = ed25519_dalek::PublicKey::from_bytes(&identity.public_key)?;
            Ok(public_key.verify(message, signature).is_ok())
        } else {
            Err(Box::new(IdentityError::NotFound(id.to_string())))
        }
    }

    pub fn revoke_identity(&mut self, id: &str, reason: Option<String>) -> Result<(), Box<dyn Error>> {
        if self.identities.remove(id).is_some() {
            if let Some(reason) = reason {
                println!("Identity {} revoked: {}", id, reason);
            } else {
                println!("Identity {} revoked.", id);
            }
            Ok(())
        } else {
            Err(Box::new(IdentityError::NotFound(id.to_string())))
        }
    }

    pub fn get_identity(&self, id: &str) -> Option<&Identity> {
        self.identities.get(id).map(|(identity, _)| identity)
    }
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use ed25519_dalek::Signature;

    #[test]
    fn test_create_and_get_identity() {
        let mut manager = IdentityManager::new();
        let mut metadata = HashMap::new();
        metadata.insert("email".to_string(), "test@example.com".to_string());

        let identity = manager.create_identity("user1".to_string(), metadata.clone()).unwrap();
        assert_eq!(identity.id, "user1");
        assert_eq!(identity.metadata, metadata);
        
        let retrieved_identity = manager.get_identity("user1").unwrap();
        assert_eq!(retrieved_identity.id, "user1");
    }

    #[test]
    fn test_sign_and_verify_identity() {
        let mut manager = IdentityManager::new();
        let identity = manager.create_identity("user1".to_string(), HashMap::new()).unwrap();
        
        let message = b"Hello, world!";
        let signature = manager.sign_message(&identity.id, message).unwrap();
        
        assert!(manager .verify_identity(&identity.id, &signature, message).unwrap());
    }

    #[test]
    fn test_revoke_identity() {
        let mut manager = IdentityManager::new();
        let identity = manager.create_identity("user1".to_string(), HashMap::new()).unwrap();
        
        assert!(manager.revoke_identity(&identity.id, Some("No longer needed".to_string())).is_ok());
        assert!(manager.get_identity(&identity.id).is_none());
    }

    #[test]
    fn test_revoke_nonexistent_identity() {
        let mut manager = IdentityManager::new();
        assert!(manager.revoke_identity("nonexistent", None).is_err());
    }

    #[test]
    fn test_verify_nonexistent_identity() {
        let mut manager = IdentityManager::new();
        let message = b"Hello, world!";
        let signature = Signature::default(); // Invalid signature for testing
        
        assert!(manager.verify_identity("nonexistent", &signature, message).is_err());
    }
            }
