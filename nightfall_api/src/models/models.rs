use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use zcash_client_backend::address::Address;

pub struct LoginUser {
    pub email: String,
    pub password: String,
}

pub struct CreateUser {
    pub id: String,
    pub email: String,
    pub username: String,
    pub password: String,
}

pub struct AuthResponse {
    pub user: User,
    pub token: String,
}

pub struct UserResponse {
    pub id: String,
    pub email: String,
    pub created_at: chrono::DateTime<Utc>,
}

pub struct ErrorResponse {
    pub error: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct User {
    pub id: String,
    pub email: String,
    pub user_name: String,
    pub password_hash: String,
    #[serde(default)]
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
    pub email: String,
}

#[derive(Debug)]
pub struct Wallet {
    pub address: Address,
    pub seed_phrase: Vec<String>,
}
