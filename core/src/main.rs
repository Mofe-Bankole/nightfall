use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use std::io;

pub mod address;
pub mod api_client;
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
    // dotenv::dotenv().ok();
    HttpServer::new(move || {
        App::new()
            .route("/", web::get().to(index))
            .service(web::scope("x`api/v1"))
    })
    .bind("0.0.0.0:7654")?
    .run()
    .await
}
