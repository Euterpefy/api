use axum::{ http::StatusCode, response::IntoResponse, Extension, Json };
use serde_json::json;

use crate::app::{ AppResult, AppState };

pub async fn get_seed_genres(Extension(state): Extension<AppState>) -> AppResult<
    impl IntoResponse
> {
    let mut client = state.spotify_client.lock().await; // Lock the client for safe access

    match client.get_genre_seeds().await {
        Ok(genre_seeds_response) => {
            // If the call is successful, return the genre seeds as JSON
            Ok((StatusCode::OK, Json(genre_seeds_response)).into_response())
        }
        Err(error) => {
            // Handle errors appropriately
            // This example returns a simple error message and a corresponding HTTP status code
            // You might want to map your custom error type to HTTP status codes
            let error_message = format!("Error fetching genre seeds: {}", error);
            let response_body = json!({ "error": error_message });
            Ok((StatusCode::INTERNAL_SERVER_ERROR, Json(response_body)).into_response())
        }
    }
}
