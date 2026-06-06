use std::{fmt, str::FromStr};

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
    Icrf,
}

impl FromStr for ReferenceFrame {
    type Err = ();

    fn from_str(input: &str) -> Result<ReferenceFrame, Self::Err> {
        match input {
            "J2000" => Ok(ReferenceFrame::J2000),
            "Itrf" => Ok(ReferenceFrame::Itrf),
            "Teme" => Ok(ReferenceFrame::Teme),
            "Icrf" => Ok(ReferenceFrame::Icrf),
            _ => Err(()),
        }
    }
}

impl fmt::Display for ReferenceFrame {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ReferenceFrame::J2000 => write!(f, "J2000"),
            ReferenceFrame::Itrf => write!(f, "Itrf"),
            ReferenceFrame::Teme => write!(f, "Teme"),
            ReferenceFrame::Icrf => write!(f, "Icrf"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum EphemerisSource {
    Predicted,
    Definitive,
    Reconstructed,
}

impl FromStr for EphemerisSource {
    type Err = ();

    fn from_str(input: &str) -> Result<EphemerisSource, Self::Err> {
        match input {
            "Predicted" => Ok(EphemerisSource::Predicted),
            "Definitive" => Ok(EphemerisSource::Definitive),
            "Reconstructed" => Ok(EphemerisSource::Reconstructed),
            _ => Err(()),
        }
    }
}

impl fmt::Display for EphemerisSource {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EphemerisSource::Predicted => write!(f, "Predicted"),
            EphemerisSource::Definitive => write!(f, "Definitive"),
            EphemerisSource::Reconstructed => write!(f, "Reconstructed"),
        }
    }
}
