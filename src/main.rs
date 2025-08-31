use std::sync::Arc;

use line_webhook::{
    config::config_loader,
    infrastructure::{axum::http_serve::start, postgres::postgres_connection},
};
use tracing::{error, info};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let dotenvy_env = match config_loader::load() {
        Ok(env) => env,
        Err(err) => {
            error!("Failed to load .env file: {}", err);
            std::process::exit(1);
        }
    };
    info!("Loaded .env file successfully");

    let postgres_pool = match postgres_connection::establish_connection(&dotenvy_env.database.url) {
        Ok(pool) => pool,
        Err(err) => {
            error!("Failed to connect to PostgreSQL: {}", err);
            std::process::exit(1);
        }
    };
    info!("Connected to PostgreSQL successfully");

    start(Arc::new(dotenvy_env), Arc::new(postgres_pool))
        .await
        .expect("Failed to start server");
}
