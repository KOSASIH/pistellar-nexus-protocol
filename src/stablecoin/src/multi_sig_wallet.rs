use std::collections::{HashSet, HashMap};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MultiSigWallet {
    owners: HashSet<String>,
    required_signatures: usize,
    transactions: HashMap<String, Transaction>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    description: String,
    signatures: HashSet<String>,
}

impl MultiSigWallet {
    pub fn new(owners: HashSet<String>, required_signatures: usize) -> Self {
        MultiSigWallet {
            owners,
            required_signatures,
            transactions: HashMap::new(),
        }
    }

    pub fn propose_transaction(&mut self, transaction_id: String, description: String) {
        let transaction = Transaction {
            description,
            signatures: HashSet::new(),
        };
        self.transactions.insert(transaction_id, transaction);
        println!("Transaction proposed: {}", description);
    }

    pub fn sign_transaction(&mut self, transaction_id: &str, signer: String) -> Result<(), String> {
        if !self.owners.contains(&signer) {
            return Err("Signer is not an owner".to_string());
        }

        if let Some(transaction) = self.transactions.get_mut(transaction_id) {
            if transaction.signatures.insert(signer) {
                println!("{} signed transaction: {}", signer, transaction_id);
                Ok(())
            } else {
                Err("Transaction already signed by this owner".to_string())
            }
        } else {
            Err("Transaction not found".to_string())
        }
    }

    pub fn execute_transaction(&self, transaction_id: &str) -> Result<bool, String> {
        if let Some(transaction) = self.transactions.get(transaction_id) {
            if transaction.signatures.len() >= self.required_signatures {
                // Execute the transaction
                println!("Transaction executed: {}", transaction.description);
                return Ok(true);
            } else {
                return Err("Not enough signatures to execute transaction.".to_string());
            }
        }
        Err("Transaction not found".to_string())
    }

    pub fn get_transaction_status(&self, transaction_id: &str) -> Option<&Transaction> {
        self.transactions.get(transaction_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_propose_transaction() {
        let owners: HashSet<String> = ["owner1".to_string(), "owner2".to_string()].iter().cloned().collect();
        let mut wallet = MultiSigWallet::new(owners, 2);
        
        wallet.propose_transaction("tx1".to_string(), "Transfer 100 Pi Coins".to_string());
        assert!(wallet.get_transaction_status("tx1").is_some());
    }

    #[test]
    fn test_sign_transaction() {
        let owners: HashSet<String> = ["owner1".to_string(), "owner2".to_string()].iter().cloned().collect();
        let mut wallet = MultiSigWallet::new(owners, 2);
        
        wallet.propose_transaction("tx1".to_string(), "Transfer 100 Pi Coins".to_string());
        assert!(wallet.sign_transaction("tx1", "owner1".to_string()).is_ok());
        assert!(wallet.sign_transaction("tx1", "owner1".to_string()).is_err()); // Already signed
    }

    #[test]
    fn test_execute_transaction() {
        let owners: HashSet<String> = ["owner1".to_string(), "owner2".to_string()].iter().cloned().collect();
        let mut wallet = MultiSigWallet::new(owners, 2);
        
        wallet.propose_transaction("tx1".to_string(), "Transfer 100 Pi Coins".to_string());
        wallet.sign_transaction("tx1", "owner1".to_string()).unwrap();
        assert!(wallet.execute_transaction("tx1").is_err()); // Not enough signatures
        
        wallet.sign_transaction("tx1", "owner2".to_string()).unwrap();
        assert!(wallet.execute_transaction("tx1").is_ok()); // Now enough signatures
    }
            }
