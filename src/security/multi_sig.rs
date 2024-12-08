use ed25519_dalek::{Keypair, Signature, Signer, Verifier};
use std::collections::HashMap;

pub struct MultiSigWallet {
    pub threshold: u32,
    pub public_keys: Vec<ed25519_dalek::PublicKey>,
    pub signatures: HashMap<ed25519_dalek::PublicKey, Signature>,
}

impl MultiSigWallet {
    pub fn new(threshold: u32, public_keys: Vec<ed25519_dalek::PublicKey>) -> Self {
        Self {
            threshold,
            public_keys,
            signatures: HashMap::new(),
        }
    }

    pub fn add_signature(&mut self, public_key: ed25519_dalek::PublicKey, signature: Signature) {
        self.signatures.insert(public_key, signature);
    }

    pub fn verify(&self, message: &[u8]) -> bool {
        let mut valid_signatures = 0;
        for (public_key, signature) in &self.signatures {
            if public_key.verify(message, signature).is_ok() {
                valid_signatures += 1;
            }
        }
        valid_signatures >= self.threshold
    }
}
