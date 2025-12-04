use bip39::Mnemonic;
use zcash_keys::keys::{UnifiedFullViewingKey, UnifiedSpendingKey};

use crate::{address::ZcashAddress, keys::Key_Generation_Service};

pub struct WalletKeys {
    pub seed: [u8; 64],
    pub ufvk: UnifiedFullViewingKey,
    pub ufsk: UnifiedSpendingKey,
}
pub struct WalletInfo {
    pub transparent_address: ZcashAddress,
    pub unified_address: ZcashAddress,
}

pub struct Wallet {
    pub keys: WalletKeys,
    pub info: WalletInfo,
}

impl Wallet {
    pub fn new() -> Self {
        //generate seed phrase (pls do not REVEAL THIS AT ANY TIME)
        // even though this is on TESTNET

        let seed_phrase = Key_Generation_Service::generate_seed_phrase();
        let seed_bytes = Key_Generation_Service::phrase_to_seed(&seed_phrase).unwrap();
        let ufvk = Key_Generation_Service::seed_to_ufvk(&seed_bytes, 0);
        Self {
            keys: WalletKeys {
                seed: seed_bytes,
                ufvk: (),
                ufsk: (),
            },
            info: (),
        }
    }
}
