use thiserror::Error;

#[derive(Error, Debug)]
pub enum RepositoryError {
    #[error("Repository query error: {0}")]
    QueryError(String),
    #[error("Other repository error")]
    Other,
}
