// tests/unit_tests.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collateralization() {
        let mut collateralization = Collateralization::new();
        collateralization.add_collateral(500.0);
        assert!(collateralization.check_collateralization());
    }

    #[test]
    fn test_multi_sig_wallet() {
        let owners: HashSet<String> = ["owner1".to_string(), "owner2".to_string()].iter().cloned().collect();
        let wallet = MultiSigWallet::new(owners, 2);
        let mut signatures: HashSet<String> = HashSet::new();
        signatures.insert("owner1".to_string());
        signatures.insert("owner2".to_string());
        assert!(wallet.execute_transaction("Transfer 100 Pi Coins", &signatures));
    }
}
