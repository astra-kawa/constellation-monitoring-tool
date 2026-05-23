use async_trait::async_trait;
use domain::{
    errors::ComputeError,
    models::{Constellation, Ephemeris},
};
use ports::inbound::OrbitPropagator;
use serde::{Deserialize, Serialize};

pub struct JuliaOrbitPropagator {
    pub client: reqwest::Client,
}

impl JuliaOrbitPropagator {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }
}

impl Default for JuliaOrbitPropagator {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Deserialize, Serialize)]
pub struct JuliaPropagationRequest {
    pub constellation: Constellation,
    pub duration_seconds: f64,
    pub step_seconds: f64,
}

#[derive(Deserialize)]
pub struct JuliaPropagationResponse {
    pub satellites: Vec<Ephemeris>,
}

#[async_trait]
impl OrbitPropagator for JuliaOrbitPropagator {
    async fn propagate(
        &self,
        constellation: Constellation,
        duration: std::time::Duration,
        step: std::time::Duration,
    ) -> Result<Vec<domain::models::Ephemeris>, domain::errors::ComputeError> {
        let payload_request = JuliaPropagationRequest {
            constellation,
            duration_seconds: duration.as_secs_f64(),
            step_seconds: step.as_secs_f64(),
        };

        let response = self
            .client
            .post("http://127.0.0.1:4001/propagate")
            .json(&payload_request)
            .send()
            .await
            .map_err(|err| {
                println!("OrbitPropagator request error: {}", err);
                ComputeError::Other
            })?
            .error_for_status()
            .map_err(|err| {
                println!("OrbitPropagator response error: {}", err);
                ComputeError::Other
            })?;

        let payload = response
            .json::<JuliaPropagationResponse>()
            .await
            .map_err(|err| {
                println!("OrbitPropagator response parse error: {}", err);
                ComputeError::Other
            })?;

        Ok(payload.satellites)
    }
}
