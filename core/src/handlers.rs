use crate::address::*;
use crate::client::*;
use crate::db::DatabaseManger;
use crate::keys::Key_Generation_Service;

// This file is responsible for exposing all API Routes in this project
//
// In the directory above there is a comprehensive README.md file that explains how to use this project.

static database_manager : DatabaseManger = DatabaseManger::new();
let seed_phrase = Key_Generation_Service::generate_seed_phrase();
