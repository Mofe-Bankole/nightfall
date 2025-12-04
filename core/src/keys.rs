use anyhow::{Ok, anyhow};
use bip39::Mnemonic;
use rand::{TryRngCore, rngs::OsRng};
use zcash_client_backend::encoding::{
    encode_extended_full_viewing_key, encode_extended_spending_key,
};
use zcash_keys::keys::{UnifiedFullViewingKey, UnifiedSpendingKey, sapling::ExtendedSpendingKey};
use zcash_protocol::consensus::{NetworkConstants, TEST_NETWORK};
use zip32::{AccountId, ChildIndex};

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
            .map_err(|e| anyhow!("Unknown Error : {}", e))?;
        let mnemonic = Mnemonic::from_entropy(&entropy).unwrap();
        mnemonic.to_string()
    }

    // pub fn into_seed(seed_phrase: &str) -> Result<() , anyhow::Error>{
    //     let mnemonic = Mnemonic::<English>::from_phrase
    // }
    // Validates the seed phrase
    pub fn validate_seed_phrase(phrase: &str) -> Result<(), KeyError> {
        Mnemonic::parse(phrase)
            .map(|_| ())
            .map_err(|_| KeyError::InvalidMnemonic)
    }

    /// Generates a key pair from a seed phrase
    ///
    /// Returns [Spending Key]  [Viewing Key]
    pub fn generate_key_pair(seed_phrase: &str) -> Result<(String, String), KeyError> {
        let mnemonic = Mnemonic::parse(seed_phrase).map_err(|_| KeyError::InvalidMnemonic)?;

        let seed_bytes = mnemonic.to_seed("");

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

    // Derives wallet spending key
    pub fn derive_zcash_spending_key(seed: &[u8; 64], account: u32) -> ExtendedSpendingKey {
        let master = ExtendedSpendingKey::master(seed);
        let purpose = master.derive_child(ChildIndex::hardened(32));
        let coin = purpose.derive_child(ChildIndex::hardened(TEST_NETWORK.coin_type()));
        coin.derive_child(ChildIndex::hardened(account))
    }

    ///Derives unified full viewing key
    ///
    /// Very important
    ///
    /// DO NOT SHARE UPON REFRACTORING
    pub fn seed_to_ufvk(
        seed: &[u8; 64],
        account: u32,
    ) -> Result<UnifiedFullViewingKey, anyhow::Error> {
        let account_id = AccountId::try_from(account)?;
        let usk = UnifiedSpendingKey::from_seed(&TEST_NETWORK, seed, account_id)?;
        Ok(usk.to_unified_full_viewing_key())
    }

    pub fn phrase_to_seed(phrase: &str) -> Result<[u8; 64], anyhow::Error> {
        let seed_bytes_derivative = Mnemonic::parse(phrase).unwrap();
        let seed_bytes = seed_bytes_derivative.to_seed("");

        Ok(seed_bytes)
    }
}
