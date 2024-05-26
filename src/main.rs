use anyhow::Result;
use axum::{
    routing::{get, post},
    Router,
};
use shortener_rs::{controller, init_pg_pool, init_trace, AppState};
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    init_trace();

    let app_state = AppState::new(init_pg_pool().await?);

    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(addr).await?;
    info!("listening on {}", addr);

    let app = Router::new()
        .route("/", post(controller::url::shorten))
        .route("/:id", get(controller::url::redirect))
        .with_state(Arc::new(app_state))
        .layer(TraceLayer::new_for_http());

    axum::serve(listener, app).await?;

    Ok(())
}
