use crate::http::handlers::{
    delete_satellite, get_constellation, get_satellite, health, propagate, set_constellation,
    set_satellite,
};
use application::app::AppState;
use axum::{
    Router,
    routing::{delete, get, post, put},
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
        .route("/api/constellation", put(set_constellation))
        .route("/api/constellation/{satellite_id}", get(get_satellite))
        .route("/api/constellation/{satellite_id}", put(set_satellite))
        .route(
            "/api/constellation/{satellite_id}",
            delete(delete_satellite),
        )
        .route("/api/propagate", post(propagate))
        .with_state(app_state)
        .layer(cors)
}
