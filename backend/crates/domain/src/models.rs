use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct EciState {
    pub epoch: DateTime<Utc>,
    pub pos_x: f64,
    pub pos_y: f64,
    pub pos_z: f64,
    pub vel_x: f64,
    pub vel_y: f64,
    pub vel_z: f64,
}

#[derive(Deserialize, Serialize)]
pub struct SatelliteData {
    pub initial_state: EciState,
}

#[derive(Deserialize, Serialize)]
pub struct Satellite {
    pub id: String,
    pub data: SatelliteData,
}

#[derive(Deserialize, Serialize)]
pub struct Constellation {
    pub satellites: Vec<Satellite>,
}

#[derive(Deserialize, Serialize)]
pub struct Ephemeris {}
