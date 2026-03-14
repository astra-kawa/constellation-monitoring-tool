use async_trait::async_trait;
use domain::{errors::ComputeError, models::Satellite};
use ports::outbound::OrbitPropagator;
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
    pub constellation: Vec<Satellite>,
    pub duration_seconds: f64,
    pub step_seconds: f64,
}

#[async_trait]
impl OrbitPropagator for JuliaOrbitPropagator {
    async fn propagate(
        &self,
        constellation: Vec<domain::models::Satellite>,
        duration: std::time::Duration,
        step: std::time::Duration,
    ) -> Result<Vec<domain::models::Ephemeris>, domain::errors::ComputeError> {
        let payload_request = JuliaPropagationRequest {
            constellation,
            duration_seconds: duration.as_secs_f64(),
            step_seconds: step.as_secs_f64(),
        };

        let _response = self
            .client
            .post("127.0.0.1:4001/propagate")
            .json(&payload_request)
            .send()
            .await
            .map_err(|_| ComputeError::Other)?;

        todo!()
    }
}
