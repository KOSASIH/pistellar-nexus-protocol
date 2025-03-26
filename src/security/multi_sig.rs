use ed25519_dalek::{Keypair, Signature, Signer, Verifier, PublicKey};
use std::collections::HashMap;
use std::error::Error;

/// A multi-signature wallet that requires a threshold number of signatures to authorize a transaction.
pub struct MultiSigWallet {
    pub threshold: u32,
    pub public_keys: Vec<PublicKey>,
    pub signatures: HashMap<PublicKey, Signature>,
}

impl MultiSigWallet {
    /// Creates a new MultiSigWallet with the specified threshold and public keys.
    ///
    /// # Arguments
    ///
    /// * `threshold` - The minimum number of signatures required to authorize a transaction.
    /// * `public_keys` - A vector of public keys that can sign transactions.
    pub fn new(threshold: u32, public_keys: Vec<PublicKey>) -> Self {
        if threshold == 0 || threshold > public_keys.len() as u32 {
            panic!("Threshold must be greater than 0 and less than or equal to the number of public keys.");
        }
        Self {
            threshold,
            public_keys,
            signatures: HashMap::new(),
        }
    }

    /// Adds a signature from a public key to the wallet.
    ///
    /// # Arguments
    ///
    /// * `public_key` - The public key of the signer.
    /// * `signature` - The signature to add.
    pub fn add_signature(&mut self, public_key: PublicKey, signature: Signature) -> Result<(), String> {
        if self.signatures.contains_key(&public_key) {
            return Err("Signature from this public key has already been added.".to_string());
        }
        self.signatures.insert(public_key, signature);
        Ok(())
    }

    /// Verifies the signatures against the provided message.
    ///
    /// # Arguments
    ///
    /// * `message` - The message that was signed.
    ///
    /// # Returns
    ///
    /// A boolean indicating whether the required number of valid signatures has been reached.
    pub fn verify(&self, message: &[u8]) -> bool {
        let mut valid_signatures = 0;
        for (public_key, signature) in &self.signatures {
            if public_key.verify(message, signature).is_ok() {
                valid_signatures += 1;
            }
        }
        valid_signatures >= self.threshold
    }

    /// Returns the number of valid signatures collected so far.
    pub fn valid_signature_count(&self) -> usize {
        self.signatures.len()
    }

    /// Returns the public keys associated with this wallet.
    pub fn get_public_keys(&self) -> &Vec<PublicKey> {
        &self.public_keys
    }
        }
