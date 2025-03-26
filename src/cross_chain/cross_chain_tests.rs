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

        // Attempt to send the transaction and assert the result
        let result = stellar.send_transaction(transaction).await;
        assert!(result.is_ok(), "Failed to send Stellar transaction: {:?}", result.err());
    }

    #[tokio::test]
    async fn test_pi_integration() {
        let pi = PiIntegration::new("https://api.pi.network/transactions");
        let transaction = PiTransaction {
            sender: "PiUser  1".to_string(),
            receiver: "PiUser  2".to_string(),
            amount: 5.0,
        };

        // Attempt to send the transaction and assert the result
        let result = pi.send_transaction(transaction).await;
        assert!(result.is_ok(), "Failed to send Pi transaction: {:?}", result.err());
    }

    #[tokio::test]
    async fn test_stellar_integration_invalid_account() {
        let stellar = StellarIntegration::new("https://api.stellar.org/transactions");
        let transaction = StellarTransaction {
            source_account: "INVALID_ACCOUNT".to_string(),
            destination_account: "GXYZ1234567890".to_string(),
            amount: 10.0,
        };

        // Attempt to send the transaction with an invalid account
        let result = stellar.send_transaction(transaction).await;
        assert!(result.is_err(), "Expected error for invalid source account");
    }

    #[tokio::test]
    async fn test_pi_integration_insufficient_balance() {
        let pi = PiIntegration::new("https://api.pi.network/transactions");
        let transaction = PiTransaction {
            sender: "PiUser  1".to_string(),
            receiver: "PiUser  2".to_string(),
            amount: 1000.0, // Assuming this amount exceeds the balance
        };

        // Attempt to send the transaction with insufficient balance
        let result = pi.send_transaction(transaction).await;
        assert!(result.is_err(), "Expected error for insufficient balance");
    }
        }
