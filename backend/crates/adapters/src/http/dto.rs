use domain::models::Ephemeris;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

#[derive(Serialize)]
pub struct MessageResponse {
    pub message: String,
}

#[derive(Deserialize, Serialize)]
pub struct PropagationRequest {
    pub duration: Duration,
    pub step: Duration,
}

#[derive(Deserialize, Serialize)]
pub struct PropagationResponse {
    pub ephemerides: Vec<Ephemeris>,
}
