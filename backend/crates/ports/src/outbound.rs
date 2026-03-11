use domain::{
    errors::ComputeError,
    models::{Satellite, Trajectory},
};
use std::time::Duration;

pub trait OrbitPropagator: Send + Sync {
    fn propagate(
        &self,
        constellation: [&Satellite],
        duration: Duration,
    ) -> Result<Trajectory, ComputeError>;
}
