// tests/integration_tests.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mint() {
        // Test minting functionality ```rust
        let mut contract = SmartContract::new();
        contract.mint("user1".to_string(), 100);
        assert_eq!(contract.get_balance("user1"), 100);
        assert_eq!(contract.get_total_supply(), 100);
    }

    #[test]
    fn test_burn() {
        let mut contract = SmartContract::new();
        contract.mint("user1".to_string(), 100);
        contract.burn("user1".to_string(), 50);
        assert_eq!(contract.get_balance("user1"), 50);
        assert_eq!(contract.get_total_supply(), 50);
    }

    #[test]
    fn test_transfer() {
        let mut contract = SmartContract::new();
        contract.mint("user1".to_string(), 100);
        contract.transfer("user1".to_string(), "user2".to_string(), 50);
        assert_eq!(contract.get_balance("user1"), 50);
        assert_eq!(contract.get_balance("user2"), 50);
    }

    #[test]
    fn test_governance() {
        let mut governance = Governance::new();
        governance.create_proposal("1".to_string(), "Increase supply".to_string());
        governance.vote("1".to_string(), "voter1".to_string(), true);
        governance.vote("1".to_string(), "voter2".to_string(), false);
        let results = governance.get_results("1").unwrap();
        assert_eq!(results.votes_for, 1);
        assert_eq!(results.votes_against, 1);
    }
}
