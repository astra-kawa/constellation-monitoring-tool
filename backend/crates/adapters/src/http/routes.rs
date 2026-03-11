use crate::http::handlers::get_health;
use application::app::AppState;
use axum::{Router, routing::get};
use tower_http::cors::{Any, CorsLayer};

pub fn build_router(app_state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .route("/api/health", get(get_health))
        .with_state(app_state)
        .layer(cors)
}
