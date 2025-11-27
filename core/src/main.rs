use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use rand::rngs::OsRng;
use rusqlite::{Connection, Params};
use std::io;
use zcash_client_sqlite::util::Clock;

use crate::db::DatabaseManger;

// use crate::{db::db::connect_db, routes::wallet::create_wallet};

pub mod address;
pub mod client;
pub mod constants;
pub mod db;
pub mod handlers;
pub mod keys;
pub mod lighwalletd_client;
pub mod models;
pub mod transactions;
pub mod wallet;
async fn index() -> impl Responder {
    HttpResponse::Ok().json("NIGHTFALL THE API FOR PRIVATE PAYMENTS")
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv::dotenv().ok();
    let dbm = DatabaseManger::init_user_db();
    let walletdb = DatabaseManger::init_wallet_db();
    HttpServer::new(move || {
        App::new()
            .route("/", web::get().to(index))
            .service(web::scope("/api/v1"))
        // .route("/api/v1/create/seed-phrase", web::get().to(keygen_service))
    })
    .bind("0.0.0.0:7654")?
    .run()
    .await
}
