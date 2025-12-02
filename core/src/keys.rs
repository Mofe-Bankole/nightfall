use anyhow::anyhow;
use bip39::Mnemonic;

use rand::{TryRngCore, rngs::OsRng};
use zcash_client_backend::encoding::{
    encode_extended_full_viewing_key, encode_extended_spending_key,
};
use zcash_keys::keys::sapling::ExtendedSpendingKey;
use zcash_protocol::consensus::{NetworkConstants, TEST_NETWORK};
use zip32::ChildIndex;

#[derive(Debug, Clone)]
pub enum KeyError {
    InvalidEntropy,
    InvalidMnemonic,
    InvalidHasher,
    DerivationFailed,
    UnknownError,
}

#[allow(non_camel_case_types)]
pub struct Key_Generation_Service;

impl Key_Generation_Service {
    pub fn init() -> Self {
        Self
    }
    /// Generates a seed phrase (pls do not share this with anyone)
    pub fn generate_seed_phrase() -> String {
        let mut entropy = [0u8; 32];
        #[allow(unused_must_use)]
        OsRng
            .try_fill_bytes(&mut entropy)
            .map_err(|e| anyhow!("Unknown Error : {}", e));
        let mnemonic = Mnemonic::from_entropy(&entropy).unwrap();
        mnemonic.to_string()
    }

    // Validates the seed phrase
    pub fn validate_seed_phrase(phrase: &str) -> Result<(), KeyError> {
        Mnemonic::parse(phrase)
            .map(|_| ())
            .map_err(|_| KeyError::InvalidMnemonic)
    }

    /// Generates a key pair from a seed phrase
    ///
    /// Returns [Public key , Private key]
    pub fn generate_key_pair(seed_phrase: &str) -> Result<(String, String), KeyError> {
        let mnemonic = Mnemonic::parse(seed_phrase).map_err(|_| KeyError::InvalidMnemonic)?;

        let seed_bytes = mnemonic.to_seed(seed_phrase);

        #[allow(deprecated)]
        let extsk = ExtendedSpendingKey::master(&seed_bytes);
        #[allow(deprecated)]
        let extfvk = extsk.to_extended_full_viewing_key();

        let spending_key_hex =
            encode_extended_spending_key(TEST_NETWORK.hrp_sapling_extended_spending_key(), &extsk);

        let viewing_key_hex = encode_extended_full_viewing_key(
            TEST_NETWORK.hrp_sapling_extended_full_viewing_key(),
            &extfvk,
        );
        println!("{} : {}", spending_key_hex, viewing_key_hex);
        Ok((spending_key_hex, viewing_key_hex))
    }

    pub fn derive_zcash_spending_key(seed: &[u8; 64], account: u32) -> ExtendedSpendingKey {
        let master = ExtendedSpendingKey::master(seed);
        let purpose = master.derive_child(ChildIndex::hardened(32));
        let coin = purpose.derive_child(ChildIndex::hardened(TEST_NETWORK.coin_type()));
        coin.derive_child(ChildIndex::hardened(account))
    }
}
