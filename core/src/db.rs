use std::sync::{Arc, Mutex};

use anyhow::anyhow;
use rusqlite::Connection;
// use std::error::Error;

pub struct DatabaseManger {
    db: Arc<Mutex<rusqlite::Connection>>,
}

impl DatabaseManger {
    pub async fn new() -> Arc<Mutex<Result<Connection, anyhow::Error>>> {
        let db = Arc::new(Mutex::new(
            rusqlite::Connection::open("./storage/main_wallet_db.db")
                .map_err(|e| anyhow!("Unable To Fetch Database : {}", e)),
        ));

        db
    }
}
