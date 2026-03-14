use async_trait::async_trait;
use domain::{
    errors::ComputeError,
    models::{Constellation, Ephemeris},
};
use std::time::Duration;

#[async_trait]
pub trait OrbitPropagator: Send + Sync {
    async fn propagate(
        &self,
        constellation: Constellation,
        duration: Duration,
        step: Duration,
    ) -> Result<Vec<Ephemeris>, ComputeError>;
}
