pub mod controller;
pub mod error;
pub mod model;

use anyhow::Result;
use sqlx::PgPool;
use std::env;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{fmt::Layer, layer::SubscriberExt, util::SubscriberInitExt, Layer as _};

#[derive(Debug)]
pub struct AppState {
    pg_pool: PgPool,
}

impl AppState {
    pub fn new(pg_pool: PgPool) -> Self {
        Self { pg_pool }
    }
}

pub async fn init_pg_pool() -> Result<PgPool> {
    let _ = dotenvy::dotenv();
    let database_url =
        env::var("DATABASE_URL").expect("environment variable DATABASE_URL is missing.");
    let pg_pool = PgPool::connect(&database_url)
        .await
        .expect("failed to connect to database.");
    Ok(pg_pool)
}

pub fn init_trace() {
    let layer = Layer::new().with_filter(LevelFilter::INFO);
    tracing_subscriber::registry().with(layer).init();
}
