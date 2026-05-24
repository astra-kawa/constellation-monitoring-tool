use crate::errors::RepositoryError;
use async_trait::async_trait;
use domain::models::{Constellation, Satellite, SatelliteEphemeris};

#[async_trait]
pub trait ConstellationRepository: Send + Sync {
    async fn set_constellation(&self, constellation: Constellation) -> Result<(), RepositoryError>;
    async fn get_constellation(&self) -> Result<Constellation, RepositoryError>;
    async fn set_satellite(&self, satellite: Satellite) -> Result<(), RepositoryError>;
    async fn get_satellite(&self, satellite_id: &str) -> Result<Satellite, RepositoryError>;
    async fn delete_satellite(&self, satellite_id: &str) -> Result<(), RepositoryError>;
    async fn set_constellation_ephemerides(
        &self,
        ephemerides: &[SatelliteEphemeris],
    ) -> Result<(), RepositoryError>;
    async fn get_constellation_ephemerides(
        &self,
    ) -> Result<Vec<SatelliteEphemeris>, RepositoryError>;
}
