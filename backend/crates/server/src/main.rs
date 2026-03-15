use adapters::{
    julia_services::orbit_propagator::JuliaOrbitPropagator,
    postgres::postgres_repository::PostgresRepository,
};
use application::app::AppState;
use std::{net::SocketAddr, sync::Arc};

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Backend running on http://{addr}");

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("failed to bind backend listener");

    let repo = PostgresRepository::new("postgresql://michal@localhost/constellation_monitor").await;
    let propagator = JuliaOrbitPropagator::new();

    let app =
        adapters::http::routes::build_router(AppState::new(Arc::new(repo), Arc::new(propagator)));

    axum::serve(listener, app)
        .await
        .expect("backend server failed");
}
