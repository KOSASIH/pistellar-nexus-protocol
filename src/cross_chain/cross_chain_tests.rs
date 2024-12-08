#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_stellar_integration() {
        let stellar = StellarIntegration::new("https://api.stellar.org/transactions");
        let transaction = StellarTransaction {
            source_account: "GABC1234567890".to_string(),
            destination_account: "GXYZ1234567890".to_string(),
            amount: 10.0,
        };

        let result = stellar.send_transaction(transaction).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_pi_integration() {
        let pi = PiIntegration::new("https://api.pi.network/transactions");
        let transaction = PiTransaction {
            sender: "PiUser 1".to_string(),
            receiver: "PiUser 2".to_string(),
            amount: 5.0,
        };

        let result = pi.send_transaction(transaction).await;
        assert!(result.is_ok());
    }
}
