use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use std::io;

pub mod address;
pub mod api_client;
pub mod app_state;
pub mod constants;
pub mod db;
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
            .route("/", web::get().to(index))
            // .route("/api/v1/create/wallet", web::post().to(create_new_wallet))
            .service(web::scope("api/v1"))
            .route("/wallet/create", web::post().to(create_new_wallet))
    })
    .bind("0.0.0.0:7654")?
    .run()
    .await
}
