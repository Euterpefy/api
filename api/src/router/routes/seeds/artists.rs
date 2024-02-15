use serde::{ Deserialize, Serialize };
use axum::{ extract::Json, http::StatusCode, response::IntoResponse, Extension };
use crate::app::{ AppResult, AppState };
use rustyspoty::models::recommendations::RecommendationsRequest;
use serde_json::json;
use std::collections::HashSet;

#[derive(Deserialize, Serialize)]
pub struct SeedArtistsRequest {
    pub genres: Vec<String>,
}

pub async fn get_artists_seeds(
    Extension(state): Extension<AppState>,
    Json(request_body): Json<SeedArtistsRequest> // Updated to use the new struct
) -> AppResult<impl IntoResponse> {
    let mut client = state.spotify_client.lock().await;

    let mut request_details = RecommendationsRequest::new();
    // Use the genres from the request body directly
    request_details.seed_genres = Some(request_body.genres);
    request_details.limit = Some(100);

    match client.get_recommendations(&request_details).await {
        Ok(recommendations) => {
            let artist_ids: HashSet<String> = recommendations.tracks
                .iter()
                .flat_map(|track| track.artists.iter().map(|artist| artist.id.clone()))
                .collect();
            let limited_artist_ids: Vec<String> = artist_ids.into_iter().take(50).collect();

            match client.get_several_artists(&limited_artist_ids).await {
                Ok(artists_info) => { Ok((StatusCode::OK, Json(artists_info)).into_response()) }
                Err(e) => {
                    let error_message = format!("Error fetching artist details: {}", e);
                    Ok(
                        (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            Json(json!({ "error": error_message })).into_response(),
                        ).into_response()
                    )
                }
            }
        }
        Err(e) => {
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
