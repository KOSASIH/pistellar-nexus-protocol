#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mint() {
        // Test minting functionality
        let mut contract = SmartContract::new();
        contract.mint("user1".to_string(), 100).expect("Minting failed");
        
        assert_eq!(contract.get_balance("user1"), 100);
        assert_eq!(contract.get_total_supply(), 100);
    }

    #[test]
    fn test_burn() {
        let mut contract = SmartContract::new();
        contract.mint("user1".to_string(), 100).expect("Minting failed");
        contract.burn("user1".to_string(), 50).expect("Burning failed");
        
        assert_eq!(contract.get_balance("user1"), 50);
        assert_eq!(contract.get_total_supply(), 50);
    }

    #[test]
    fn test_transfer() {
        let mut contract = SmartContract::new();
        contract.mint("user1".to_string(), 100).expect("Minting failed");
        contract.transfer("user1".to_string(), "user2".to_string(), 50).expect("Transfer failed");
        
        assert_eq!(contract.get_balance("user1"), 50);
        assert_eq!(contract.get_balance("user2"), 50);
    }

    #[test]
    fn test_governance() {
        let mut governance = Governance::new();
        governance.create_proposal("1".to_string(), "Increase supply".to_string()).expect("Proposal creation failed");
        
        governance.vote("1".to_string(), "voter1".to_string(), true).expect("Voting failed");
        governance.vote("1".to_string(), "voter2".to_string(), false).expect("Voting failed");
        
        let results = governance.get_results("1").expect("Failed to get results");
        assert_eq!(results.votes_for, 1);
        assert_eq!(results.votes_against, 1);
    }

    #[test]
    fn test_transfer_insufficient_balance() {
        let mut contract = SmartContract::new();
        contract.mint("user1".to_string(), 50).expect("Minting failed");
        
        // Attempt to transfer more than the balance
        let result = contract.transfer("user1".to_string(), "user2".to_string(), 100);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Insufficient balance for transfer");
    }

    #[test]
    fn test_governance_multiple_votes() {
        let mut governance = Governance::new();
        governance.create_proposal("2".to_string(), "Decrease supply".to_string()).expect("Proposal creation failed");
        
        governance.vote("2".to_string(), "voter1".to_string(), true).expect("Voting failed");
        governance.vote("2".to_string(), "voter1".to_string(), false).expect("Voting failed"); // Same voter voting again
        
        let results = governance.get_results("2").expect("Failed to get results");
        assert_eq!(results.votes_for, 1);
        assert_eq!(results.votes_against, 1); // Should still count as one vote against
    }
}
