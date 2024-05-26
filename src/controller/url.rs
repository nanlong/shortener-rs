use crate::{model, AppState};
use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
};
use serde::{Deserialize, Serialize};
use std::{env, sync::Arc};

#[derive(Debug, Deserialize)]
pub struct ShortenReq {
    url: String,
}

#[derive(Debug, Serialize)]
pub struct ShortenResp {
    url: String,
}

pub async fn shorten(
    State(app_state): State<Arc<AppState>>,
    Json(data): Json<ShortenReq>,
) -> Result<impl IntoResponse, StatusCode> {
    let id = model::url::insert_url(&app_state.pg_pool, data.url)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let _ = dotenvy::dotenv();
    let domain = env::var("DOMAIN").map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let body = Json(ShortenResp {
        url: format!("{}/{}", domain, id),
    });

    Ok((StatusCode::CREATED, body))
}

pub async fn redirect(
    State(app_state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    let url = model::url::get_url(&app_state.pg_pool, id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Redirect::permanent(&url))
}
