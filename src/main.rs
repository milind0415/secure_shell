use axum::Server;
use sqlx::postgres::PgPoolOptions;
use std::{net::SocketAddr, sync::Arc};
mod config;
mod db;
mod dtos;
mod errors;
mod handler;
mod middleware;
mod models;
mod router;
mod utils;

use config::Config;
use db::DBClient;
use router::create_router;

pub struct AppState {
    db: DBClient,
    config: Config,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::init();
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await?;
    let db_client = DBClient::new(pool);

    let app_state = Arc::new(AppState {
        db: db_client,
        config: config.clone(),
    });

    let app = create_router(app_state);

    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    println!("listening on {}", addr);
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
