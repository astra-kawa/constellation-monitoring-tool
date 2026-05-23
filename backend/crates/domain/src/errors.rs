use thiserror::Error;

#[derive(Error, Debug)]
pub enum ComputeError {
    #[error("Other compute error")]
    Other,
}
