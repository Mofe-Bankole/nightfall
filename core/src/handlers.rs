use crate::api_client::*;
use crate::db::DatabaseManager;
use crate::keys::Key_Generation_Service;

// This file is responsible for exposing all API Routes in this project
//
// In the directory above there is a comprehensive README.md file that explains how to use this project.

pub struct Handler {
    // wallet: Wallet,
    database: DatabaseManager,
    key_generation_service: Key_Generation_Service,
    api_client: APIClient,
}

impl Handler {
    pub fn create_new_handler() -> Self {
        let database = DatabaseManager::init();
        let key_gen = Key_Generation_Service::init();
        let api_client = APIClient::init();

        Self {
            database: database,
            key_generation_service: key_gen,
            api_client,
        }
    }
}
