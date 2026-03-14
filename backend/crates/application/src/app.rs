use ports::inbound::OrbitPropagator;
use ports::outbound::ConstellationRepository;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub constellation_repository: Arc<dyn ConstellationRepository>,
    pub propagator: Arc<dyn OrbitPropagator>,
}

impl AppState {
    pub fn new(
        constellation_repository: Arc<dyn ConstellationRepository>,
        propagator: Arc<dyn OrbitPropagator>,
    ) -> Self {
        Self {
            constellation_repository,
            propagator,
        }
    }
}
