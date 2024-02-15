use std::{ env, net::SocketAddr };

use app::AppState;
use axum::Router;
use rustyspoty::SpotifyClientCredentials;

use crate::router::create_router;

mod utils;
mod router;
pub mod app;

/// Starts the Axum web server and sets up routing.
///
/// This function initializes the Axum router with the provided application state,
/// then binds and serves the application on a specified address.
pub async fn run_api(app_state: AppState) {
    let app: Router = Router::new().nest("/api", create_router(app_state).await);

    println!("Starting server on 127.0.0.1:8080");
    let address: SocketAddr = SocketAddr::from(([127, 0, 0, 1], 8080));

    // axum::serve::Serve::bind(&address).serve(app.into_make_service()).await.unwrap();
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app.into_make_service()).await.unwrap();
}

pub fn create_spotify_client() -> SpotifyClientCredentials {
    dotenv::dotenv().ok();
    // Setup
    let client_id: String = env::var("SPOTIFY_CLIENT_ID").expect("Expected a client id");
    let client_secret: String = env
        ::var("SPOTIFY_CLIENT_SECRET")
        .expect("Expected a client secret");
    SpotifyClientCredentials::new(client_id, client_secret)
}
