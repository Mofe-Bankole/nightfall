use crate::keys::Key_Generation_Service;
use bip39::Mnemonic;
// use orchard::keys::Diversifier;
use serde::{Deserialize, Serialize};
use thiserror::Error;
// use zcash_client_backend::keys::UnifiedSpendingKey;
use zcash_keys::{
    // address::{Receiver, UnifiedAddress},
    encoding::{encode_payment_address_p, encode_transparent_address_p},
};
use zcash_protocol::consensus::TEST_NETWORK;
use zcash_transparent::keys::{AccountPrivKey, IncomingViewingKey};
use zip32::{AccountId, Scope};

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
    // Address type is either shielded or transparent
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
    /// Generates a seed from a mnemonic phrase.
    ///
    /// Examples of seed phrases : abandon abandon abandon.........about
    // todo pass this as json
    pub fn seed_from_mnemonic(mnemonic: &str) -> Result<[u8; 64], AddressError> {
        let seed = Mnemonic::parse(mnemonic)
            .map_err(|_| AddressError::InvalidSeedPhrase)?
            .to_seed("");

        let mut seed_bytes = [0u8; 64];
        seed_bytes.copy_from_slice(&seed);
        Ok(seed_bytes)
    }

    /// Generates a shielded address from a seed and account index.
    ///
    /// Type ----- "u-address"
    pub fn generate_shielded_address(
        seed: &[u8; 64],
        account: u32,
    ) -> Result<ZcashAddress, AddressError> {
        // extract the spending key
        let spending_key = Key_Generation_Service::derive_zcash_spending_key(seed, account);
        let (_, payment_address) = spending_key.default_address();

        // todo
        // Store public address as plain text in sqliteDB
        let encoded = encode_payment_address_p(&TEST_NETWORK, &payment_address);
        Ok(ZcashAddress::new(encoded, AddressType::Shielded, account))
    }

    /// Derives the external transparent receiver / address for the requested account.
    ///
    /// Type ----- "t-address"
    pub fn generate_transparent_address(
        seed: &[u8; 64],
        account: u32,
    ) -> Result<ZcashAddress, AddressError> {
        let account_id = AccountId::try_from(account).map_err(|_| AddressError::InvalidAccount)?;

        // Creates a new accounts PRIVATE KEY (NOT TO BE SHARED)
        let account_pk = AccountPrivKey::from_seed(&TEST_NETWORK, seed, account_id)
            .map_err(|_| AddressError::FailedKeyGen)?;

        let account_pub = account_pk.to_account_pubkey();

        let external_ivk = account_pub
            .derive_external_ivk()
            .map_err(|_| AddressError::FailedKeyGen)?;

        let (transparent_address, _) = external_ivk.default_address();

        let encoded = encode_transparent_address_p(&TEST_NETWORK, &transparent_address);
        Ok(ZcashAddress::new(
            encoded,
            AddressType::Transparent,
            account,
        ))
    }

    pub fn validate_address(address: &str) -> Result<(), AddressError> {
        // In production, this would use zcash_primitives to validate the address
        // For now, we'll create a placeholder format
        if address.starts_with("z1")
            || address.starts_with("t1")
            || address.starts_with("t3")
            || address.starts_with("u1")
        {
            Ok(())
        } else {
            Err(AddressError::InvalidAddress)
        }
    }

    /// Generates a shielded address from a mnemonic phrase
    pub fn generate_shielded_address_from_mnemonic(
        mnemonic: &str,
        account: u32,
    ) -> Result<ZcashAddress, AddressError> {
        let seed = AddressGenerationService::seed_from_mnemonic(mnemonic)?;
        AddressGenerationService::generate_shielded_address(&seed, account)
    }

    /// Generates a transparent address from a mnemonic phrase
    pub fn generate_transparent_address_from_mnemonic(
        mnemonic: &str,
        account: u32,
    ) -> Result<ZcashAddress, AddressError> {
        let seed = AddressGenerationService::seed_from_mnemonic(mnemonic)?;
        AddressGenerationService::generate_transparent_address(&seed, account)
    }

    // Derives Transparent Address from seed
    pub fn derive_transparent_address_from_seed(
        seed: &[u8; 64],
        account: u32,
    ) -> Result<ZcashAddress, anyhow::Error> {
        // gets the account id
        let account_id = AccountId::try_from(account).unwrap();
        // retreives account spending key
        let account_sk = AccountPrivKey::from_seed(&TEST_NETWORK, seed, account_id);
        // account private key
        let account_pk = account_sk.unwrap().to_account_pubkey();
        let ext_ivk = account_pk.derive_external_ivk().unwrap();
        let (taddr, _) = ext_ivk.default_address();
        Ok(ZcashAddress {
            address: encode_transparent_address_p(&TEST_NETWORK, &taddr),
            address_type: AddressType::Transparent,
            account: account,
        })
    }

    // to do refractor
    // pub fn generate_unified_address(
    //     seed: &[u8; 64],
    //     account: u32,
    // ) -> Result<ZcashAddress, AddressError> {
    //     // Derive the AccountId from the provided account parameter.
    //     let account_id = AccountId::try_from(account)
    //         .map_err(|_| AddressError::InvalidAccount)?;

    //     // Generate the UnifiedSpendingKey using the seed and account ID.
    //     let usk = UnifiedSpendingKey::from_seed(&TEST_NETWORK, seed, account_id)
    //         .map_err(|_| AddressError::FailedKeyGen)?;

    //     // Get the UnifiedFullViewingKey.
    //     let ufvk = usk.to_unified_full_viewing_key();

    //     // Helper closure to get transparent receiver, if present.
    //     let t_receiver = ufvk.transparent()
    //         .and_then(|t| {
    //             let (taddr, _) = t.default_address();
    //             Some(Receiver::Transparent(taddr))
    //         });

    //     // Helper closure to get orchard receiver, if present
    //     let o_receiver = ufvk.orchard()
    //         .and_then(|o| {
    //             // In a real implementation the correct diversifier should be determined per key
    //             let diversifier = Diversifier::from_bytes([0u8; 11]);
    //             // Satisfy diverification and get address
    //             match o.address(diversifier) {
    //                 Some(addr) => Some(Receiver::Orchard(addr)),
    //                 None => None
    //             }
    //         });

    //     // Compose a unified address from available receivers
    //     let ua = UnifiedAddress::from_receivers(
    //         o_receiver,
    //         None, // No Sapling receiver in this example
    //         t_receiver
    //     ).map_err(|_| AddressError::GenerationFailed)?;

    //     Ok(ZcashAddress {
    //         address: ua.to_string(),
    //         address_type: AddressType::Shielded,
    //         account,
    //     })
    // }
}
