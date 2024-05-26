use crate::{error, model, AppState};
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
) -> Result<impl IntoResponse, error::Error> {
    let id = model::url::insert_url(&app_state.pg_pool, data.url)
        .await
        .map_err(|e| error::Error::InternalServerError(e.to_string()))?;

    let _ = dotenvy::dotenv();
    let domain =
        env::var("DOMAIN").map_err(|e| error::Error::InternalServerError(e.to_string()))?;
    let body = Json(ShortenResp {
        url: format!("{}/{}", domain, id),
    });

    Ok((StatusCode::CREATED, body))
}

pub async fn redirect(
    State(app_state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, error::Error> {
    let url = model::url::get_url(&app_state.pg_pool, &id)
        .await
        .map_err(|e: anyhow::Error| error::Error::InternalServerError(e.to_string()))?
        .ok_or(error::Error::ShortenUrlNotFound(id))?;

    Ok(Redirect::permanent(&url))
}
