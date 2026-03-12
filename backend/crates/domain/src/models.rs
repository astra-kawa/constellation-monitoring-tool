use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Serialize)]
struct EciState {
    epoch: DateTime<Utc>,
    pos_x: f64,
    pos_y: f64,
    pos_z: f64,
    vel_x: f64,
    vel_z: f64,
}

#[derive(Serialize)]
pub struct Satellite {
    id: String,
    initial_state: EciState,
}

pub struct Trajectory {}
