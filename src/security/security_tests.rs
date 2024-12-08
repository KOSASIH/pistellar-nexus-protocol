#[cfg(test)]
mod tests {
    use super::*;
    use ed25519_dalek::{Keypair, Signature, Signer};
    use std::collections::HashMap;

    #[test]
    fn test_multi_sig_wallet() {
        let mut wallet = MultiSigWallet::new(2, vec![
            Keypair::generate(&mut rand::thread_rng()).public,
            Keypair::generate(&mut rand::thread_rng()).public,
        ]);

        let message = b"Hello, world!";
        let keypair = Keypair::generate(&mut rand::thread_rng());
        let signature: Signature = keypair.sign(message);

        wallet.add_signature(keypair.public, signature);
        assert!(!wallet.verify(message));

        let keypair2 = Keypair::generate(&mut rand::thread_rng());
        let signature2: Signature = keypair2.sign(message);

        wallet.add_signature(keypair2.public, signature2);
        assert!(wallet.verify(message));
    }

    #[test]
    fn test_encryption() {
        let key = rand::thread_rng().gen::<[u8; 32]>();
        let data = b"Hello, world!";
        let encrypted_data = encrypt(data, &key);
        let decrypted_data = decrypt(&encrypted_data, &key);
        assert_eq!(data, &decrypted_data[..]);
    }
}
