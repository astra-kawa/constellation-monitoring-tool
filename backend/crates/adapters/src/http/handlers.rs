use std::time::Duration;

use crate::http::dto::{ErrorResponse, MessageResponse, PropagationRequest, PropagationResponse};
use application::app::AppState;
use axum::{
    Json,
    extract::{Path, State, rejection::JsonRejection},
    http::StatusCode,
};
use domain::models::{Constellation, Satellite};
use tracing::{error, info, instrument};

#[instrument(skip_all)]
pub async fn health() -> Json<MessageResponse> {
    // todo: eventually implement actual health checks
    let status = "ok".to_owned();
    info!("Health status: {status}");

    Json(MessageResponse { message: status })
}

#[instrument(skip_all)]
pub async fn get_constellation(
    State(state): State<AppState>,
) -> Result<Json<Constellation>, (StatusCode, Json<ErrorResponse>)> {
    let constellation = state
        .constellation_repository
        .get_constellation()
        .await
        .map_err(|error| {
            let error = format!("Constellation repository error: {error}");
            error!("Failed to get constellation: {error}");

            (
                StatusCode::from_u16(500).unwrap(),
                Json(ErrorResponse { error }),
            )
        })?;

    let constellation_ids = constellation
        .satellites
        .iter()
        .map(|s| s.id.clone())
        .collect::<Vec<_>>();
    info!("Retrieved constellation: {constellation_ids:?}");

    Ok(Json(constellation))
}

#[instrument(skip_all)]
pub async fn set_constellation(
    State(state): State<AppState>,
    payload: Result<Json<Constellation>, JsonRejection>,
) -> Result<Json<MessageResponse>, (StatusCode, Json<ErrorResponse>)> {
    let payload_request = payload.map(|Json(inner)| inner).map_err(|error| {
        let error = format!("Invalid constellation request: {error}");
        error!(error);

        (StatusCode::BAD_REQUEST, Json(ErrorResponse { error }))
    })?;

    state
        .constellation_repository
        .set_constellation(payload_request)
        .await
        .map_err(|error| {
            let error = format!("Constellation repository error: {error}");
            error!(error);

            (
                StatusCode::from_u16(500).unwrap(),
                Json(ErrorResponse { error }),
            )
        })?;

    info!("Constellation set successfully");

    Ok(Json(MessageResponse {
        message: "Ok".to_string(),
    }))
}

#[instrument(skip_all)]
pub async fn set_satellite(
    State(state): State<AppState>,
    Path(satellite_id): Path<String>,
    payload: Result<Json<Satellite>, JsonRejection>,
) -> Result<Json<MessageResponse>, (StatusCode, Json<ErrorResponse>)> {
    let payload_request = payload.map(|Json(inner)| inner).map_err(|error| {
        let error = format!("invalid satellite request: {error}");
        error!(error);

        (StatusCode::BAD_REQUEST, Json(ErrorResponse { error }))
    })?;

    if satellite_id != payload_request.id {
        let error = format!(
            "Satellite id path {} does not match payload id {}",
            &satellite_id, &payload_request.id
        );
        error!(error);

        return Err((StatusCode::BAD_REQUEST, Json(ErrorResponse { error })));
    }

    state
        .constellation_repository
        .set_satellite(payload_request)
        .await
        .map_err(|error| {
            let error = format!("Constellation repository error: {error}");
            error!(error);

            (
                StatusCode::from_u16(500).unwrap(),
                Json(ErrorResponse { error }),
            )
        })?;

    info!("Satellite {} updated successfully", &satellite_id);

    Ok(Json(MessageResponse {
        message: "Ok".to_string(),
    }))
}

#[instrument(skip_all)]
pub async fn get_satellite(
    State(state): State<AppState>,
    Path(satellite_id): Path<String>,
) -> Result<Json<Satellite>, (StatusCode, Json<ErrorResponse>)> {
    let satellite = state
        .constellation_repository
        .get_satellite(&satellite_id)
        .await
        .map_err(|error| {
            let error = format!("Constellation repository error: {error}");
            error!(error);

            (
                StatusCode::from_u16(500).unwrap(),
                Json(ErrorResponse { error }),
            )
        })?;

    info!("Retrieved satellite: {}", &satellite.id);

    Ok(Json(satellite))
}

#[instrument(skip_all)]
pub async fn delete_satellite(
    State(state): State<AppState>,
    Path(satellite_id): Path<String>,
) -> Result<Json<MessageResponse>, (StatusCode, Json<ErrorResponse>)> {
    state
        .constellation_repository
        .delete_satellite(&satellite_id)
        .await
        .map_err(|error| {
            let error = format!("Constellation repository error: {error}");
            error!(error);

            (
                StatusCode::from_u16(500).unwrap(),
                Json(ErrorResponse { error }),
            )
        })?;

    let message = format!("Deleted satellite: {}", &satellite_id);
    info!(message);

    Ok(Json(MessageResponse { message }))
}

#[instrument(skip_all)]
pub async fn propagate(
    State(state): State<AppState>,
    payload: Result<Json<PropagationRequest>, JsonRejection>,
) -> Result<Json<PropagationResponse>, (StatusCode, Json<ErrorResponse>)> {
    let payload_request = payload.map(|Json(inner)| inner).map_err(|error| {
        let error = format!("Invalid propagation request: {error}");
        error!("Failed to parse payload: {error}");

        (StatusCode::BAD_REQUEST, Json(ErrorResponse { error }))
    })?;

    if payload_request.duration_seconds <= 0.0 {
        let error = "Invalid propagation request: duration_seconds must be > 0.0".to_string();
        error!(error);

        return Err((StatusCode::BAD_REQUEST, Json(ErrorResponse { error })));
    }

    if payload_request.step_seconds <= 0.0 {
        let error = "Invalid propagation request: step_seconds must be > 0.0".to_string();
        error!(error);

        return Err((StatusCode::BAD_REQUEST, Json(ErrorResponse { error })));
    }

    let constellation = state
        .constellation_repository
        .get_constellation()
        .await
        .map_err(|error| {
            let error = format!("Constellation repository error: {error}");
            error!(error);

            (
                StatusCode::from_u16(500).unwrap(),
                Json(ErrorResponse { error }),
            )
        })?;

    let propagate_result = state
        .propagator
        .propagate(
            constellation,
            Duration::from_secs_f64(payload_request.duration_seconds),
            Duration::from_secs_f64(payload_request.step_seconds),
        )
        .await
        .map_err(|error| {
            let error = format!("Compute error: {error}");
            error!(error);

            (
                StatusCode::from_u16(500).unwrap(),
                Json(ErrorResponse { error }),
            )
        })?;

    state
        .constellation_repository
        .set_constellation_ephemerides(&propagate_result)
        .await
        .map_err(|error| {
            let error = format!("Constellation repository error: {error}");
            error!(error);

            (
                StatusCode::from_u16(500).unwrap(),
                Json(ErrorResponse { error }),
            )
        })?;

    info!("Generated ephemerides successfully");

    Ok(Json(PropagationResponse {
        ephemerides: propagate_result,
    }))
}
