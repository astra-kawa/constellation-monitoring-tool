use application::app::AppState;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Backend running on http://{addr}");

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("failed to bind backend listener");

    let app = adapters::http::routes::build_router(AppState {});

    axum::serve(listener, app)
        .await
        .expect("backend server failed");
}
