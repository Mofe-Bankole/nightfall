use axum::{
    Extension, Json, Router,
    routing::{get, post},
};
use nightfall::routes::auth::register_user;
#[allow(unused)]
use std::{env, error::Error};
use tokio::net::TcpListener;

use crate::db::db::connect_db;

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
        .layer(Extension(pool));

    let listener = TcpListener::bind("0.0.0.0:5843").await.unwrap();
    axum::serve(listener, api).await?;
    println!("=================   NIGHTFALL THE PRIVATE PAYMENTS API  =================");

    Ok(())
}
