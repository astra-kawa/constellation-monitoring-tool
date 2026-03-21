use crate::http::dto::{ErrorResponse, MessageResponse, PropagationRequest, PropagationResponse};
use application::app::AppState;
use axum::{
    Json,
    extract::{Path, State, rejection::JsonRejection},
    http::StatusCode,
};
use chrono::Utc;
use domain::models::{Constellation, Satellite};

pub async fn health() -> Json<MessageResponse> {
    println!("{} | GET /health", Utc::now());

    Json(MessageResponse {
        message: "ok".to_owned(),
    })
}

pub async fn get_constellation(
    State(state): State<AppState>,
) -> Result<Json<Constellation>, (StatusCode, Json<ErrorResponse>)> {
    let constellation = state
        .constellation_repository
        .get_constellation()
        .await
        .map_err(|error| {
            (
                StatusCode::from_u16(500).unwrap(),
                Json(ErrorResponse {
                    error: format!("Constellation repository error: {error}"),
                }),
            )
        })?;

    Ok(Json(constellation))
}

pub async fn set_constellation(
    State(state): State<AppState>,
    payload: Result<Json<Constellation>, JsonRejection>,
) -> Result<Json<MessageResponse>, (StatusCode, Json<ErrorResponse>)> {
    let payload_request = payload.map(|Json(inner)| inner).map_err(|error| {
        (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: format!("invalid constellation request: {error}"),
            }),
        )
    })?;

    state
        .constellation_repository
        .set_constellation(payload_request)
        .await
        .map_err(|error| {
            (
                StatusCode::from_u16(500).unwrap(),
                Json(ErrorResponse {
                    error: format!("Constellation repository error: {error}"),
                }),
            )
        })?;

    Ok(Json(MessageResponse {
        message: "Ok".to_string(),
    }))
}

pub async fn set_satellite(
    State(state): State<AppState>,
    Path(satellite_id): Path<String>,
    payload: Result<Json<Satellite>, JsonRejection>,
) -> Result<Json<MessageResponse>, (StatusCode, Json<ErrorResponse>)> {
    let payload_request = payload.map(|Json(inner)| inner).map_err(|error| {
        (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: format!("invalid satellite request: {error}"),
            }),
        )
    })?;

    if satellite_id != payload_request.id {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: format!(
                    "Satellite id path {} does not match payload id {}",
                    &satellite_id, &payload_request.id
                ),
            }),
        ));
    }

    state
        .constellation_repository
        .set_satellite(payload_request)
        .await
        .map_err(|error| {
            (
                StatusCode::from_u16(500).unwrap(),
                Json(ErrorResponse {
                    error: format!("Constellation repository error: {error}"),
                }),
            )
        })?;

    Ok(Json(MessageResponse {
        message: "Ok".to_string(),
    }))
}

pub async fn get_satellite(
    State(state): State<AppState>,
    Path(satellite_id): Path<String>,
) -> Result<Json<Satellite>, (StatusCode, Json<ErrorResponse>)> {
    let satellite = state
        .constellation_repository
        .get_satellite(&satellite_id)
        .await
        .map_err(|error| {
            (
                StatusCode::from_u16(500).unwrap(),
                Json(ErrorResponse {
                    error: format!("Constellation repository error: {error}"),
                }),
            )
        })?;

    Ok(Json(satellite))
}

pub async fn delete_satellite(
    State(state): State<AppState>,
    Path(satellite_id): Path<String>,
) -> Result<Json<MessageResponse>, (StatusCode, Json<ErrorResponse>)> {
    state
        .constellation_repository
        .delete_satellite(&satellite_id)
        .await
        .map_err(|error| {
            (
                StatusCode::from_u16(500).unwrap(),
                Json(ErrorResponse {
                    error: format!("Constellation repository error: {error}"),
                }),
            )
        })?;

    Ok(Json(MessageResponse {
        message: format!("Deleted satellite: {}", &satellite_id).to_string(),
    }))
}

pub async fn propagate(
    State(state): State<AppState>,
    payload: Result<Json<PropagationRequest>, JsonRejection>,
) -> Result<Json<PropagationResponse>, (StatusCode, Json<ErrorResponse>)> {
    let payload_request = payload.map(|Json(inner)| inner).map_err(|error| {
        (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: format!("invalid propagation request: {error}"),
            }),
        )
    })?;

    let constellation = state
        .constellation_repository
        .get_constellation()
        .await
        .map_err(|error| {
            (
                StatusCode::from_u16(500).unwrap(),
                Json(ErrorResponse {
                    error: format!("Constellation repository error: {error}"),
                }),
            )
        })?;

    let propagate_result = state
        .propagator
        .propagate(
            constellation,
            payload_request.duration,
            payload_request.step,
        )
        .await
        .map_err(|error| {
            (
                StatusCode::from_u16(500).unwrap(),
                Json(ErrorResponse {
                    error: format!("Compute error: {error}"),
                }),
            )
        })?;

    Ok(Json(PropagationResponse {
        ephemerides: propagate_result,
    }))
}
