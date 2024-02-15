use std::sync::Arc;

use axum::{ extract::FromRef, http::StatusCode, response::{ IntoResponse, Response } };
use rustyspoty::{ RustyError, SpotifyClientCredentials };
use serde_json::json;
use tokio::sync::Mutex;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub spotify_client: Arc<Mutex<SpotifyClientCredentials>>,
}

impl AppState {
    pub fn new(spotify_client: SpotifyClientCredentials) -> Self {
        AppState {
            spotify_client: Arc::new(Mutex::new(spotify_client)),
        }
    }
}

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug)]
pub enum AppError {
    ApiError(String),
    RustySpoty(RustyError),
}

impl From<RustyError> for AppError {
    fn from(err: RustyError) -> Self {
        AppError::RustySpoty(err)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::ApiError(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::RustySpoty(err) => (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", err)),
        };
        let body = json!({ "error": error_message }).to_string();
        Response::builder()
            .status(status)
            .header("Content-Type", "application/json")
            .body(body.into())
            .unwrap()
    }
}
