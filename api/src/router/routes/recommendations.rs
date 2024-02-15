use axum::{ http::StatusCode, response::IntoResponse, Extension, Json };
use rustyspoty::models::recommendations::RecommendationsRequest;
use serde_json::json;

use crate::app::{ AppResult, AppState };

pub async fn get_tracks_recommendations(
    Extension(state): Extension<AppState>,
    Json(mut request_details): Json<RecommendationsRequest>
) -> AppResult<impl IntoResponse> {
    let mut client = state.spotify_client.lock().await;

    if request_details.limit.is_none() {
        request_details.limit = Some(50);
    }

    match client.get_recommendations(&request_details).await {
        Ok(recommendations) => { Ok((StatusCode::OK, Json(recommendations)).into_response()) }
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

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{ body::Body, http::{ Request, StatusCode }, routing::post, Extension, Router };

    use rustyspoty::models::recommendations::RecommendationsResponse;
    use serde_json::json;
    use tower::ServiceExt; // for `oneshot` method

    use crate::{
        app::{ AppResult, AppState },
        create_spotify_client,
        router::routes::tests::extract_response_body,
    };

    // Initialize test environment
    fn initialize_test_router() -> Router {
        Router::new()
            .route("/recommendation-tracks", post(get_tracks_recommendations))
            .layer(Extension(AppState::new(create_spotify_client())))
    }

    #[tokio::test]
    async fn test_rec_tracks() -> AppResult<()> {
        let router = initialize_test_router();

        {
            let mut rec1_request = RecommendationsRequest::new();
            rec1_request.limit = Some(10);

            rec1_request.seed_genres = Some(vec!["hip-hop".to_string(), "chill".to_string()]);
            rec1_request.seed_artists = Some(vec!["4NHQUGzhtTLFvgF5SZesLK".to_string()]);
            rec1_request.seed_tracks = Some(vec!["0c6xIDDpzE81m2q797ordA".to_string()]);

            let rec1_response = router
                .clone()
                .oneshot(
                    Request::builder()
                        .method("POST")
                        .uri(format!("/recommendation-tracks"))
                        .header("Content-Type", "application/json")
                        .body(Body::from(json!(rec1_request).to_string()))
                        .unwrap()
                ).await
                .unwrap();

            assert_eq!(rec1_response.status(), StatusCode::OK);
            let rec_response: RecommendationsResponse = extract_response_body(
                rec1_response
            ).await.expect("error extracting recommendation response from body");

            assert_eq!(rec_response.tracks.len(), 10);
        }

        Ok(())
    }
}
