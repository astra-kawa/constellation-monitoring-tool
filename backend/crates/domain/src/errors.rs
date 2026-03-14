use thiserror::Error;

#[derive(Error, Debug)]
pub enum ComputeError {
    #[error("Other comute error")]
    Other,
}
