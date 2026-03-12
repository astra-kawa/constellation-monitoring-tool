use crate::http::dto::{ErrorResponse, MessageResponse, PropagationRequest, PropagationResponse};
use application::app::AppState;
use axum::{
    Json,
    extract::{State, rejection::JsonRejection},
    http::StatusCode,
};
use chrono::Utc;

pub async fn health() -> Json<MessageResponse> {
    println!("{} | GET /health", Utc::now());

    Json(MessageResponse {
        message: "ok".to_owned(),
    })
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

    let propagate_result = state.propagator.propagate(
        payload_request.constellation,
        payload_request.duration,
        payload_request.step,
    );

    // let _response = state
    //     .client
    //     .post("test")
    //     .json(&payload_request)
    //     .send()
    //     .await
    //     .map_err(|error| {
    //         (
    //             StatusCode::BAD_GATEWAY,
    //             Json(ErrorResponse {
    //                 error: format!("failed to reach orbit propagator: {error}"),
    //             }),
    //         )
    //     })?;

    // todo: if ComputeError, return relevant HTTP error

    Ok(Json(PropagationResponse {
        ephemerides: propagate_result.unwrap(),
    }))
}
