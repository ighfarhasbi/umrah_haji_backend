mod helper;
mod jsonwebtoken;
mod models;
mod routes;
mod middleware;

use std::{env, time::Duration};

use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;

pub async fn run() -> Result<(), String> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .min_connections(5)
        .acquire_timeout(Duration::from_secs(600))
        .idle_timeout(Duration::from_secs(600))
        .connect(&database_url)
        .await
        .map_err(|e| e.to_string())?;

    let app = routes::create_route(pool).await;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap(); // port server aplikasi
    axum::serve(listener, app)
        .await
        .map_err(|_| "Failed to start server".to_string())?; // run server aplikasi dan db
    Ok(())
}
