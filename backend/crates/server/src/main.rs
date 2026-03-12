use application::app::AppState;
use domain::{
    errors::ComputeError,
    models::{Satellite, Trajectory},
};
use ports::outbound::OrbitPropagator;
use std::{net::SocketAddr, sync::Arc, time::Duration};

struct NoopOrbitPropagator;

impl OrbitPropagator for NoopOrbitPropagator {
    fn propagate(
        &self,
        _constellation: Vec<Satellite>,
        _duration: Duration,
    ) -> Result<Trajectory, ComputeError> {
        Ok(Trajectory {})
    }
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Backend running on http://{addr}");

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("failed to bind backend listener");

    let app = adapters::http::routes::build_router(AppState::new(Arc::new(NoopOrbitPropagator)));

    axum::serve(listener, app)
        .await
        .expect("backend server failed");
}
