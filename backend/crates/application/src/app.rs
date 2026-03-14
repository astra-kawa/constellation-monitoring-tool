use ports::outbound::OrbitPropagator;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub propagator: Arc<dyn OrbitPropagator>,
}

impl AppState {
    pub fn new(propagator: Arc<dyn OrbitPropagator>) -> Self {
        Self { propagator }
    }
}
