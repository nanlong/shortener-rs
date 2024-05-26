use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Internal Server Error: {0}")]
    InternalServerError(String),
    #[error("Not Found for id: {0}")]
    ShortenUrlNotFound(String),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Error::InternalServerError(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
            }
            Error::ShortenUrlNotFound(_) => {
                (StatusCode::NOT_FOUND, self.to_string()).into_response()
            }
        }
    }
}
