use ports::outbound::OrbitPropagator;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub client: reqwest::Client,
    pub propagator: Arc<dyn OrbitPropagator>,
}
