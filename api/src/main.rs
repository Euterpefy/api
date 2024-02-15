use api::{ app::AppState, create_spotify_client, run_api };

#[tokio::main]
async fn main() {
    let app_state = AppState::new(create_spotify_client());
    run_api(app_state).await;
}
