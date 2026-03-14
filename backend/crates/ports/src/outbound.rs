use crate::errors::RepositoryError;
use async_trait::async_trait;
use domain::models::Constellation;

#[async_trait]
pub trait ConstellationRepository: Send + Sync {
    async fn set_constellation(&self, constellation: Constellation) -> Result<(), RepositoryError>;
    async fn get_constellation(&self) -> Result<Constellation, RepositoryError>;
}
