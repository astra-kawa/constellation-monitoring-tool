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
pub struct SatelliteEphemeris {
    pub id: String,
    pub cartesian_ephemeris: Vec<CartesianState>,
    pub keplerian_ephemeris: Vec<KeplerianState>,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct CartesianState {
    pub datetime: DateTime<Utc>,
    pub pos_x: f64,
    pub pos_y: f64,
    pub pos_z: f64,
    pub vel_x: f64,
    pub vel_y: f64,
    pub vel_z: f64,
    pub reference_frame: ReferenceFrame,
    pub source: EphemerisSource,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct KeplerianState {
    pub datetime: DateTime<Utc>,
    pub sma_km: f64,
    pub eccentricity: f64,
    pub inclination_deg: f64,
    pub raan_deg: f64,
    pub arg_periapsis_deg: f64,
    pub true_anomaly_deg: f64,
    pub reference_frame: ReferenceFrame,
    pub source: EphemerisSource,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum ReferenceFrame {
    J2000,
    Itrf,
    Teme,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum EphemerisSource {
    Predicted,
    Definitive,
    Reconstructed,
}
