use thiserror::Error;

#[derive(Error, Debug)]
pub enum RepositoryError {
    #[error("Other repository error")]
    Other,
}
