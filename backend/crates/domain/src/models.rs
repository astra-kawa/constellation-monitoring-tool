use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct EciState {
    pub epoch: DateTime<Utc>,
    pub pos_x: f32,
    pub pos_y: f32,
    pub pos_z: f32,
    pub vel_x: f32,
    pub vel_y: f32,
    pub vel_z: f32,
}

#[derive(Deserialize, Serialize)]
pub struct Satellite {
    pub id: String,
    pub initial_state: EciState,
}

#[derive(Deserialize, Serialize)]
pub struct Constellation {
    pub satellites: Vec<Satellite>,
}

#[derive(Deserialize, Serialize)]
pub struct Ephemeris {}
