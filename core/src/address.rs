use crate::keys::Key_Generation_Service;
use bip39::Mnemonic;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use zcash_keys::{
    address::{Receiver, UnifiedAddress},
    encoding::{AddressCodec, encode_payment_address_p, encode_transparent_address_p},
};
use zcash_protocol::consensus::TEST_NETWORK;
use zcash_transparent::{
    address::TransparentAddress,
    keys::{AccountPrivKey, IncomingViewingKey},
};
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

        Ok(seed_bytes)
    }

    /// Generates a shielded address from a seed and account index.
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

    pub fn derive_sapling_address(
        seed: &[u8; 64],
        account: u32,
    ) -> Result<ZcashAddress, anyhow::Error> {
        let spending_key = Key_Generation_Service::derive_zcash_spending_key(seed, account);

        let (_, payment_address) = spending_key.default_address();
        let encoded = encode_payment_address_p(&TEST_NETWORK, &payment_address);
        let address = ZcashAddress::new(encoded, AddressType::Shielded, account);
        Ok(address)
    }

    pub fn derive_unified_address(
        &mut self,
        seed: &[u8; 64],
        account: u32,
    ) -> Result<ZcashAddress, anyhow::Error> {
        // SAPLING
        let sk = Key_Generation_Service::derive_zcash_spending_key(seed, account);

        #[allow(deprecated)]
        let fvk = sk.to_extended_full_viewing_key();
        let (_d, sapling) = fvk.default_address();
        let sapling_receiver = Receiver::Sapling(sapling);
        // let orchard_receiver = Receiver::Orchard(sapling);

        let t_addr = Self::derive_transparent_address(seed, account).unwrap();
        let transparent_raw = TransparentAddress::decode(&TEST_NETWORK, &t_addr.address);
        let transparent_receiver = Receiver::Transparent(transparent_raw.unwrap());

        let ua = UnifiedAddress::from_receivers(transparent_receiver, sapling_receiver)?;
    }
    use orchard::keys::{SpendingKey as OrchardSpendingKey, FullViewingKey as OrchardFullViewingKey};
    use zcash_protocol::consensus::Network;

    pub fn derive_orchard_keys(seed: &[u8; 32]) -> (OrchardSpendingKey, OrchardFullViewingKey) {
        // Orchard uses a 32-byte seed for the spending key
        let sk = OrchardSpendingKey::from_zip32_seed(seed, 0)
            .expect("32-byte seed always works");
        let fvk = OrchardFullViewingKey::from(&sk);

        (sk, fvk)
    }

}
