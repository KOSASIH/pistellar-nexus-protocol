#[cfg(test)]
mod tests {
    use super::*;
    use ink_env::test;

    #[ink::test]
    fn test_transfer() {
        let mut contract = MyContract::new();
        let accounts = test::default_accounts();
        let initial_balance = 100;

        // Set initial balances
        contract.balances.insert(accounts.alice, initial_balance);
        contract.balances.insert(accounts.bob, 0);

        // Transfer funds
        assert_eq!(contract.transfer(accounts.bob, 50), Ok(()));
        assert_eq!(contract.balance_of(accounts.alice), 50);
        assert_eq!(contract.balance_of(accounts.bob), 50);
    }

    #[ink::test]
    fn test_insufficient_balance() {
        let mut contract = MyContract::new();
        let accounts = test::default_accounts();
        contract.balances.insert(accounts.alice, 10);
        contract.balances.insert(accounts.bob, 0);

        // Attempt to transfer more than available balance
        assert_eq!(contract.transfer(accounts.bob, 20), Err("Insufficient balance"));
    }
}
