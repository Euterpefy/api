use axum::{ routing::post, Router };

use self::{ recommendations::get_tracks_recommendations, seeds::seeds_routes };

mod seeds;
mod recommendations;
mod tests;

pub fn api_routes() -> Router {
    Router::new()
        .nest("/seeds", seeds_routes())
        .route("/recommendation-tracks", post(get_tracks_recommendations))
}
