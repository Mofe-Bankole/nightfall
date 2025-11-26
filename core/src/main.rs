use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use std::io;


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
