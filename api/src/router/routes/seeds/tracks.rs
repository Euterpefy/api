use axum::{ extract::Json, http::StatusCode, response::IntoResponse, Extension };
use rustyspoty::models::recommendations::RecommendationsRequest;
use serde::{ Deserialize, Serialize };
use serde_json::json;

use crate::app::{ AppResult, AppState };

#[derive(Deserialize, Serialize)]
pub struct SeedTracksRequest {
    pub genres: Option<Vec<String>>,
    pub artists: Option<Vec<String>>,
}

pub async fn get_track_seeds(
    Extension(state): Extension<AppState>,
    Json(request_body): Json<SeedTracksRequest>
) -> AppResult<impl IntoResponse> {
    // Lock the client for safe access
    let mut client = state.spotify_client.lock().await;

    let mut request_details = RecommendationsRequest::new();
    request_details.seed_genres = request_body.genres;
    request_details.seed_artists = request_body.artists;
    request_details.limit = Some(50);

    match client.get_recommendations(&request_details).await {
        Ok(recommendations) => {
            Ok((StatusCode::OK, Json(json!({"tracks": recommendations.tracks}))).into_response())
        }
        Err(e) => {
            // If there's an error, return an appropriate response
            let error_message = format!("Error getting track seeds: {}", e);
            Ok(
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "error": error_message })).into_response(),
                ).into_response()
            )
        }
    }
}
