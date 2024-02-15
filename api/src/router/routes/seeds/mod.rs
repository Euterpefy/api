use axum::{ routing::{ get, post }, Router };

use self::{ artists::get_artists_seeds, genres::get_seed_genres, tracks::get_track_seeds };

mod genres;
mod tracks;
mod artists;

pub fn seeds_routes() -> Router {
    Router::new()
        .route("/genres", get(get_seed_genres))
        .route("/artists", post(get_artists_seeds))
        .route("/tracks", post(get_track_seeds))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{ body::Body, http::{ Request, StatusCode }, Extension, Router };
    use rustyspoty::models::{
        artist::Artists,
        recommendations::GenreSeedsResponse,
        track::TracksResponse,
    };

    use serde_json::json;
    use tower::ServiceExt; // for `oneshot` method

    use crate::{
        app::{ AppResult, AppState },
        create_spotify_client,
        router::routes::tests::extract_response_body,
    };

    // Initialize test environment
    fn initialize_test_router() -> Router {
        seeds_routes().layer(Extension(AppState::new(create_spotify_client())))
    }

    #[tokio::test]
    async fn test_seeds() -> AppResult<()> {
        let router = initialize_test_router();

        let genres_response = router
            .clone()
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri(format!("/genres"))
                    .body(Body::empty())
                    .unwrap()
            ).await
            .unwrap();

        assert_eq!(genres_response.status(), StatusCode::OK);

        let _: GenreSeedsResponse = extract_response_body(genres_response).await.expect(
            "error extracting genres response from body"
        );

        let artists_request = artists::SeedArtistsRequest {
            genres: vec!["chill".to_string(), "hip-hop".to_string()],
        };

        let artists_response = router
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri(format!("/artists"))
                    .header("Content-Type", "application/json")
                    .body(Body::from(json!(artists_request).to_string()))
                    .unwrap()
            ).await
            .unwrap();
        assert_eq!(artists_response.status(), StatusCode::OK);

        let seed_artists: Artists = extract_response_body(artists_response).await.expect(
            "error extracting tracks from body"
        );

        let seed_tracks_request = tracks::SeedTracksRequest {
            genres: Some(vec!["chill".to_string(), "hip-hop".to_string()]),
            artists: Some(
                vec![seed_artists.artists[0].id.clone(), seed_artists.artists[1].id.clone()]
            ),
        };

        let tracks_response = router
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri(format!("/tracks"))
                    .header("Content-Type", "application/json")
                    .body(Body::from(json!(seed_tracks_request).to_string()))
                    .unwrap()
            ).await
            .unwrap();

        assert_eq!(tracks_response.status(), StatusCode::OK);

        let _tracks: TracksResponse = extract_response_body(tracks_response).await.expect(
            "error extracting tracks from body"
        );

        Ok(())
    }
}
