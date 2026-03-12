use domain::{
    errors::ComputeError,
    models::{Ephemeris, Satellite},
};
use std::time::Duration;

pub trait OrbitPropagator: Send + Sync {
    fn propagate(
        &self,
        constellation: Vec<Satellite>,
        duration: Duration,
        step: Duration,
    ) -> Result<Vec<Ephemeris>, ComputeError>;
}
