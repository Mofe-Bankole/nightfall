use anyhow::{Ok, anyhow};
use rand::rngs::OsRng;
use rusqlite::Connection;
use zcash_client_sqlite::{WalletDb, util::SystemClock};
use zcash_protocol::consensus::{TEST_NETWORK, TestNetwork};

// #[derive(Debug)]
pub struct DatabaseManager;

impl DatabaseManager {
    pub fn init() -> Self {
        Self
    }

    pub fn init_user_db() -> Result<Connection, anyhow::Error> {
        let user_db = rusqlite::Connection::open("./storage/user_db.db")
            .map_err(|e| anyhow!("Unable To Fetch Database : {}", e))?;

        Ok(user_db)
    }

    pub fn init_wallet_db()
    -> Result<WalletDb<Connection, TestNetwork, SystemClock, OsRng>, anyhow::Error> {
        let params = TEST_NETWORK;
        let wallet_db = WalletDb::for_path("./storage/wallet_db.db", params, SystemClock, OsRng)
            .map_err(|e| anyhow!("Err : {}", e))?;

        Ok(wallet_db)
    }
}
