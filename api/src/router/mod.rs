use axum::{ middleware, routing::get, Extension, Router };

use crate::app::AppState;

use self::{
    middlewares::{ json_response_wrapper::ApplyJsonResponseWrapperExt, log_route::log_route },
    routes::api_routes,
};

mod routes;

mod middlewares;

pub async fn create_router(app_state: AppState) -> Router {
    Router::new()
        .merge(api_routes())
        .route(
            "/hello",
            get(|| async { "Hello, World!" })
        )
        .layer(Extension(app_state.clone()))
        .layer(middleware::from_fn(log_route))
        .with_json_response_wrapper() // Apply the JSON response wrapper middleware
}
