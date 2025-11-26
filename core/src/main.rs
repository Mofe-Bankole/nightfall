use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use std::io;

use crate::{address::AddressGenerationService, db::DatabaseManger, keys::Key_Generation_Service};

// use crate::{db::db::connect_db, routes::wallet::create_wallet};

pub mod address;
pub mod client;
pub mod constants;
pub mod db;
pub mod handlers;
pub mod keys;
pub mod lighwalletd_client;
pub mod models;
pub mod prover;
pub mod wallet;
// pub mod zcash_models;

// pub struct Account<C: rustsq
async fn index() -> impl Responder {
    HttpResponse::Ok().json("NIGHTFALL THE API FOR PRIVATE PAYMENTS")
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv::dotenv().ok();

    // let _db_manager = DatabaseManger::new();
    // let seed_phrase = Key_Generation_Service::generate_seed_phrase();
    // let _adr_gen = AddressGenerationService::seed_from_mnemonic(seed_phrase.as_str());

    // let zcash_address = AddressGenerationService::generate_shielded_address(seed, account)
    HttpServer::new(move || {
        App::new()
            // .app_data(pool_data.clone())
            .route("/", web::get().to(index))
            .service(web::scope("/api/v1"))
        // .route("/api/v1/create/seed-phrase", web::get().to(keygen_service))
    })
    .bind("0.0.0.0:7654")?
    .run()
    .await
}
