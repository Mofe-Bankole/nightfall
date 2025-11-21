use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use std::io;

use crate::{
    db::db::connect_db,
    routes::{
        auth::{register_user, sign_in_user},
        wallet::create_wallet,
    },
};

pub mod db;
pub mod models;
pub mod routes;
pub mod services;
pub mod utils;

async fn index() -> impl Responder {
    HttpResponse::Ok().json("NIGHTFALL THE API FOR PRIVATE PAYMENTS")
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv::dotenv().ok();

    let pool = connect_db()
        .await
        .expect("Failed to establish database connection");

    let pool_data = web::Data::new(pool);

    HttpServer::new(move || {
        App::new()
            .app_data(pool_data.clone())
            .route("/", web::get().to(index))
            .service(
                web::scope("/api/v1")
                    .service(
                        web::scope("/auth")
                            .route("/register", web::post().to(register_user))
                            .route("/login", web::post().to(sign_in_user)),
                    )
                    .service(
                        web::scope("/create")
                            .route("/new/wallet", web::post().to(create_new_wallet)),
                    )
                    .service(
                        web::scope("/pczt").route("/new/wallet", web::post().to(init_transaction)),
                    )
                    .route("/wallet", web::post().to(create_wallet)),
            )
    })
    .bind("0.0.0.0:7654")?
    .run()
    .await
}
