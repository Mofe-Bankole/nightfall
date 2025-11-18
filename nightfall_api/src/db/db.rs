use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use std::env;

use crate::utils::constants::MAX_CONNECTIONS;

pub type DbPool = Pool<Postgres>;

pub async fn connect_db() -> Result<DbPool, sqlx::Error> {
    dotenv::dotenv().ok();

    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL must be set in environment variables");

    let pool = PgPoolOptions::new()
        .max_connections(MAX_CONNECTIONS)
        .min_connections(1)
        .connect(&database_url)
        .await?;

    Ok(pool)
}

pub async fn test_connection(pool: &DbPool) -> Result<(), sqlx::Error> {
    sqlx::query("SELECT 1").execute(pool).await?;

    println!("Database connection successful!");
    Ok(())
}
