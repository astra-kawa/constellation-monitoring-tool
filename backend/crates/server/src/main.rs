use adapters::{
    lox_propagator::propagator::LoxOrbitPropagator,
    postgres::postgres_repository::PostgresRepository,
};
use application::app::AppState;
use dotenv::dotenv;
use std::env;
use std::{net::SocketAddr, sync::Arc};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Backend running on http://{addr}");

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("failed to bind backend listener");

    let repo = PostgresRepository::new(&env::var("DATABASE_URL").unwrap()).await;
    let propagator = LoxOrbitPropagator {};

    let app =
        adapters::http::routes::build_router(AppState::new(Arc::new(repo), Arc::new(propagator)));

    axum::serve(listener, app)
        .await
        .expect("backend server failed");
}
