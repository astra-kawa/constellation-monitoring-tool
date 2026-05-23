use domain::models::SatelliteEphemeris;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

#[derive(Serialize)]
pub struct MessageResponse {
    pub message: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PropagationRequest {
    // todo - properly implement Duration in request, choose time units in ui
    pub duration_seconds: f64,
    pub step_seconds: f64,
}

#[derive(Deserialize, Serialize)]
pub struct PropagationResponse {
    pub ephemerides: Vec<SatelliteEphemeris>,
}
