// src/multi_sig_wallet.rs
use std::collections::HashSet;

pub struct MultiSigWallet {
    owners: HashSet<String>,
    required_signatures: usize,
}

impl MultiSigWallet {
    pub fn new(owners: HashSet<String>, required_signatures: usize) -> Self {
        MultiSigWallet {
            owners,
            required_signatures,
        }
    }

    pub fn execute_transaction(&self, transaction: &str, signatures: &HashSet<String>) -> bool {
        if signatures.len() >= self.required_signatures {
            // Execute the transaction
            println!("Transaction executed: {}", transaction);
            return true;
        }
        println!("Not enough signatures to execute transaction.");
        false
    }
}
