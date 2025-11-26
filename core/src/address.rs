use crate::keys::Key_Generation_Service;
use bip39::Mnemonic;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use zcash_keys::encoding::{encode_payment_address_p, encode_transparent_address_p};
use zcash_protocol::consensus::TEST_NETWORK;
use zcash_transparent::keys::{AccountPrivKey, IncomingViewingKey};
use zip32::AccountId;
/// Address Errors , Feel Free to add any other if you so derire
#[derive(Debug, Error)]
pub enum AddressError {
    #[error("Invalid Address")]
    InvalidAddress,
    #[error("Invalid Format")]
    InvalidFormat,
    #[error("Generation Failed")]
    GenerationFailed,
    #[error("Unknown Error")]
    UnknowError,
    #[error("Failed Key Generation")]
    FailedKeyGen,
    #[error("Invalid account index")]
    InvalidAccount,
    #[error("Invalid Seed Phrase")]
    InvalidSeedPhrase,
}

#[derive(Debug, Clone, Serialize, Deserialize, Copy, PartialEq, Eq)]
pub enum AddressType {
    /// Transparent addresses have balances that can be viewed from an explorer (eg https://testnet.cipherscan.app/)
    Transparent,
    /// Shielded Addresses cannot be viewed from an explorer, hence insuring privacy of the user
    ///
    /// It is recommended you ALWAYS use your shielded address
    Shielded,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZcashAddress {
    pub address: String,
    pub address_type: AddressType,
    pub account: u32,
}

impl ZcashAddress {
    pub fn new(address: String, address_type: AddressType, account: u32) -> Self {
        Self {
            address,
            address_type,
            account,
        }
    }
}

pub struct AddressGenerationService;

impl AddressGenerationService {
    pub fn seed_from_mnemonic(mnemonic: &str) -> Result<[u8; 64], AddressError> {
        let seed = Mnemonic::parse(mnemonic)
            .map_err(|_| AddressError::InvalidSeedPhrase)?
            .to_seed("");

        let mut seed_bytes = [0u8; 64];
        seed_bytes.copy_from_slice(&seed);

        Ok(seed_bytes)
    }

    pub fn generate_shielded_address(
        seed: &[u8; 64],
        account: u32,
    ) -> Result<ZcashAddress, AddressError> {
        let spending_key = Key_Generation_Service::derive_zcash_spending_key(seed, account);
        let (_, payment_address) = spending_key.default_address();

        let encoded = encode_payment_address_p(&TEST_NETWORK, &payment_address);

        Ok(ZcashAddress::new(encoded, AddressType::Shielded, account))
    }

    /// Derives the external transparent receiver for the requested account.
    pub fn generate_transparent_address(
        seed: &[u8; 64],
        account: u32,
    ) -> Result<ZcashAddress, AddressError> {
        let account_id = AccountId::try_from(account).map_err(|_| AddressError::InvalidAccount)?;
        
        // creates a new accounts PRIVATE KEY
        // 
        // Data is not to be shared
        let account_sk = AccountPrivKey::from_seed(&TEST_NETWORK, seed, account_id)
            .map_err(|_| AddressError::FailedKeyGen)?;

        let account_pub = account_sk.to_account_pubkey();
        let external_ivk = account_pub
            .derive_external_ivk()
            .map_err(|_| AddressError::FailedKeyGen)?;
        let (transparent_address, _) = external_ivk.default_address();
        
        let encoded = encode_transparent_address_p(&TEST_NETWORK, &transparent_address);
        println!("{}", encoded);
        Ok(ZcashAddress::new(
            encoded,
            AddressType::Transparent,
            account,
        ))
    }

    pub fn validate_address(address: &str) -> Result<(), AddressError> {
        // In production, this would use zcash_primitives to validate the address
        // For now, we'll create a placeholder format
        if address.starts_with("z1") || address.starts_with("t1") {
            Ok(())
        } else {
            Err(AddressError::InvalidAddress)
        }
    }
}
