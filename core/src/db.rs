use anyhow::{Ok, anyhow};
use rand::{RngCore, rngs::OsRng};
use rusqlite::Connection;
use std::{
    borrow::BorrowMut,
    sync::{Arc, Mutex},
};
use zcash_client_sqlite::{
    WalletDb,
    util::{Clock, SystemClock},
};
use zcash_protocol::consensus::{Parameters, TEST_NETWORK, TestNetwork};
// use std::error::Error;

// #[derive(Debug)]
pub struct DatabaseManger<P, PR, CL, RNG> {
    user_db: rusqlite::Connection,
    wallet_db: WalletDb<P, PR, CL, RNG>,
}

impl<P, PR, CL, RNG> DatabaseManger<P, PR, CL, RNG> {
    pub fn init_user_db() -> Result<Connection, anyhow::Error> {
        let user_db = rusqlite::Connection::open("./storage/user_db.db")
            .map_err(|e| anyhow!("Unable To Fetch Database : {}", e))?;

        Ok(user_db)
    }
    pub fn init_wallet_db()
    -> Result<WalletDb<Connection, TestNetwork, SystemClock, OsRng>, anyhow::Error> {
        let params = TEST_NETWORK;
        let wallet_db = WalletDb::for_path("/storage/wallet_db.db", params, SystemClock, OsRng)
            .map_err(|e| anyhow!("Err : {}", e))?;

        Ok(wallet_db)
    }
}
