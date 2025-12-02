use crate::keys::Key_Generation_Service;
use bip39::Mnemonic;
use orchard::keys::Diversifier;
use zcash_client_backend::keys::UnifiedSpendingKey;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use zcash_keys::{
    address::{Receiver, UnifiedAddress},
    encoding::{encode_payment_address_p, encode_transparent_address_p},
};
use zcash_protocol::consensus::TEST_NETWORK;
use zcash_transparent::{
    keys::{AccountPrivKey, IncomingViewingKey},
};
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
    // todo pass this as json
    pub fn seed_from_mnemonic(mnemonic: &str) -> Result<[u8; 64], AddressError> {
        let seed = Mnemonic::parse(mnemonic)
            .map_err(|_| AddressError::InvalidSeedPhrase)?
            .to_seed("");

        let mut seed_bytes = [0u8; 64];
        seed_bytes.copy_from_slice(&seed);
        // println!("{}" , seed_bytes);
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
    pub fn derive_transparent_address(
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
    pub fn generate_unified_address(
        seed: &[u8; 64],
        account: u32,
    ) -> Result<ZcashAddress, AddressError> {
        let usk =
            UnifiedSpendingKey::from_seed(&TEST_NETWORK, seed, AccountId::try_from(account).unwrap())
                .map_err(|_| AddressError::FailedKeyGen)
                .unwrap();

        // full viewing
        let ufvk = usk.to_unified_full_viewing_key();
        let diversifier = ufvk.orchard().then()
        let orchard_address = ufvk.orchard().expect("No orchard key generated").address(Diversifier::fro, Scope::External);

        let orchard_receiver = Receiver::Orchard(orchard_address);

        let transparent_addr = ufvk
            .transparent()
            .and_then(|t| {
                let (_, taddr) = t.default_address();
                Some(taddr)
            });

        let t_addr = ufvk.transparent().map(|pk| {
            let ivk = ufvk.to_unified_incoming_viewing_key();
        });
        let sapling_receiver = None;
        let ua = UnifiedAddress::from_receivers(orchard_receiver, None, transparent);
        return Ok(ZcashAddress{
            account,
            address,
            address_type : AddressType::Shielded
        });
    }
}
