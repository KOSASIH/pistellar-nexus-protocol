use serde::{Deserialize, Serialize};
use ed25519_dalek::{Keypair, Signature, Signer, Verifier};
use rand::rngs::OsRng;
use std::collections::HashMap;
use std::error::Error;

#[derive(Serialize, Deserialize, Clone)]
pub struct Identity {
    pub id: String,
    pub public_key: Vec<u8>,
    pub metadata: HashMap<String, String>,
}

pub struct IdentityManager {
    identities: HashMap<String, Identity>,
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
        self.identities.insert(id, identity.clone());
        Ok(identity)
    }

    pub fn verify_identity(&self, id: &str, signature: &Signature, message: &[u8]) -> Result<bool, Box<dyn Error>> {
        if let Some(identity) = self.identities.get(id) {
            let public_key = ed25519_dalek::PublicKey::from_bytes(&identity.public_key)?;
            Ok(public_key.verify(message, signature).is_ok())
        } else {
            Err("Identity not found".into())
        }
    }

    pub fn revoke_identity(&mut self, id: &str) -> Result<(), Box<dyn Error>> {
        if self.identities.remove(id).is_some() {
            Ok(())
        } else {
            Err("Identity not found".into())
        }
    }

    pub fn get_identity(&self, id: &str) -> Option<&Identity> {
        self.identities.get(id)
    }
}
