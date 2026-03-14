use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct EciState {
    epoch: DateTime<Utc>,
    pos_x: f64,
    pos_y: f64,
    pos_z: f64,
    vel_x: f64,
    vel_z: f64,
}

#[derive(Deserialize, Serialize)]
pub struct Satellite {
    id: String,
    initial_state: EciState,
}

#[derive(Deserialize, Serialize)]
pub struct Constellation {
    pub satellites: Vec<Satellite>,
}

#[derive(Deserialize, Serialize)]
pub struct Ephemeris {}
