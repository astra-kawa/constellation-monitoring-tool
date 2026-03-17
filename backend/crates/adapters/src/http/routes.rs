use crate::http::handlers::{get_constellation, health, propagate};
use application::app::AppState;
use axum::{
    Router,
    routing::{get, post},
};
use tower_http::cors::{Any, CorsLayer};

pub fn build_router(app_state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .route("/api/health", get(health))
        .route("/api/constellation", get(get_constellation))
        .route("/api/propagate", post(propagate))
        .with_state(app_state)
        .layer(cors)
}
