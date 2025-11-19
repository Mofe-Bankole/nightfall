use axum::{
    Extension, Json, Router,
    routing::{get, post},
};

#[allow(unused)]
use std::{env, error::Error};
use tokio::{net::TcpListener, sync::Mutex};

use crate::{
    db::db::connect_db, routes::auth::register_user, utils::client::get_latest_block_height_raw,
};

pub mod db;
pub mod models;
pub mod routes;
pub mod services;
pub mod utils;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();

    let pool = connect_db().await?;

    let api = Router::new()
        .route(
            "/",
            get(|| async { Json("NIGHTFALL THE API FOR PRIVATE PAYMENTS") }),
        )
        .route("/api/v1/auth/register", post(register_user))
        // .route("/api/v1/wallet", post(create_wallet))
        // .route("/api/v1/wallet/:id/balance", get(retrieve_wallet_balances))
        // .route("/api/v1/tx/:id", post(get_transaction_by_uuid))
        // .route("/api/v1/block/latest", post(get_latest_block_height_raw()))
        // .route("/api/v1/block/:number", post(fetch_block))
        // .route("/api/v1/health", get(fetch_api_health))
        // .route("api/v1/tx/pczt/create", post(initialize_transaction))
        // .route("api/v1/tx/pczt/prove", post(validate_pczt))
        .layer(Extension(pool));

    let listener = TcpListener::bind("0.0.0.0:5843").await.unwrap();
    axum::serve(listener, api).await?;
    println!("=================   NIGHTFALL THE PRIVATE PAYMENTS API  =================");

    Ok(())
}
