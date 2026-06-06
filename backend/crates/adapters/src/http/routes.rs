use std::time::Duration;

use crate::http::handlers::{
    delete_satellite, get_constellation, get_satellite, health, propagate, set_constellation,
    set_satellite,
};
use application::app::AppState;
use axum::{
    Router,
    body::Body,
    extract::Request,
    routing::{delete, get, post, put},
};
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing::info;

pub fn build_router(app_state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(|request: &Request<_>| {
            tracing::info_span!(
                "request",
                method = %request.method(),
                uri = %request.uri(),
            )
        })
        .on_request(|request: &Request<Body>, span: &tracing::Span| {
            if span.field("uri").is_some() {
                info!("-> {} {}", request.method(), request.uri());
            }
        })
        .on_response(
            |response: &axum::http::Response<Body>, latency: Duration, span: &tracing::Span| {
                if span.field("uri").is_some() {
                    info!("<- {} in {:?}", response.status(), latency,);
                }
            },
        );

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
        .layer(trace_layer)
        .layer(cors)
}
