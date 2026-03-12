use domain::models::Satellite;
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
    constellation: Vec<Satellite>,
    duration: Duration,
}

#[derive(Serialize)]
pub struct PropagationResponse {}
