#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_collateralization() {
        let mut collateralization = Collateralization::new();
        collateralization.add_collateral(500.0);
        
        // Check if collateralization is valid
        assert!(collateralization.check_collateralization());
        
        // Test with insufficient collateral
        collateralization.add_collateral(-600.0); // Simulate removing collateral
        assert!(!collateralization.check_collateralization());
    }

    #[test]
    fn test_multi_sig_wallet() {
        let owners: HashSet<String> = ["owner1".to_string(), "owner2".to_string()].iter().cloned().collect();
        let wallet = MultiSigWallet::new(owners, 2); // Require 2 signatures
        
        let mut signatures: HashSet<String> = HashSet::new();
        signatures.insert("owner1".to_string());
        
        // Attempt to execute transaction with insufficient signatures
        assert!(!wallet.execute_transaction("Transfer 100 Pi Coins", &signatures));
        
        // Add the second signature
        signatures.insert("owner2".to_string());
        assert!(wallet.execute_transaction("Transfer 100 Pi Coins", &signatures));
    }

    #[test]
    fn test_insufficient_funds() {
        let mut wallet = MultiSigWallet::new(HashSet::new(), 2);
        wallet.add_funds(100.0);
        
        // Attempt to withdraw more than available
        assert!(!wallet.withdraw(150.0));
        
        // Withdraw available funds
        assert!(wallet.withdraw(100.0));
    }

    #[test]
    fn test_collateralization_with_multiple_assets() {
        let mut collateralization = Collateralization::new();
        collateralization.add_collateral(500.0);
        collateralization.add_collateral(300.0);
        
        // Check if total collateralization is valid
        assert!(collateralization.check_collateralization());
        
        // Remove some collateral
        collateralization.add_collateral(-600.0); // Simulate removing collateral
        assert!(!collateralization.check_collateralization());
    }

    #[test]
    fn test_transaction_history() {
        let mut wallet = MultiSigWallet::new(HashSet::new(), 2);
        let owners: HashSet<String> = ["owner1".to_string(), "owner2".to_string()].iter().cloned().collect();
        wallet.set_owners(owners);
        
        let mut signatures: HashSet<String> = HashSet::new();
        signatures.insert("owner1".to_string());
        signatures.insert("owner2".to_string());
        
        // Execute a transaction
        wallet.execute_transaction("Transfer 100 Pi Coins", &signatures);
        
        // Check transaction history
        assert_eq!(wallet.transaction_history.len(), 1);
        assert_eq!(wallet.transaction_history[0], "Transfer 100 Pi Coins");
    }
    }
