use crate::{address::ZcashAddress, keys::Key_Generation_Service};
use std::collections::HashMap;
use zcash_primitives::transaction::Transaction;

pub struct WalletState {
    pub mnemonic: String,
    pub seed: [u8; 64],
    pub shielded_addresses: Vec<ZcashAddress>,
    pub transparent_addresses: Vec<ZcashAddress>,
    pub transactions: Vec<Transaction>,
    pub balances_record: HashMap<String, u64>,
}

pub struct Wallet {
    state: Option<WalletState>,
}

impl Wallet {
    pub fn new(state: Option<WalletState>) -> Self {
        Self { state }
    }

    pub fn initialize_new_wallet(mnemonic: String, seed: [u8; 64]) -> Self {
        //generate seed phrase (pls do not REVEAL THIS AT ANY TIME)
        // even though this is on TESTNET
        let seed_phrase = Key_Generation_Service::generate_seed_phrase();
        
        let state = WalletState {
            mnemonic,
            seed,
            shielded_addresses: Vec::new(),
            transparent_addresses: Vec::new(),
            transactions: Vec::new(),
            balances_record: HashMap::new(),
        };
        Self::new(Some(state))
    }
}
