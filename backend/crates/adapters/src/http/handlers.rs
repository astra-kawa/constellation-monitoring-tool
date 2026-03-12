use crate::http::dto::{ErrorResponse, MessageResponse, PropagationRequest, PropagationResponse};
use application::app::AppState;
use axum::{Json, debug_handler, extract::State, http::StatusCode};

pub async fn health() -> Json<MessageResponse> {
    Json(MessageResponse {
        message: "ok".to_owned(),
    })
}

#[debug_handler]
pub async fn propagate(
    State(state): State<AppState>,
    payload: Option<Json<PropagationRequest>>,
) -> Result<Json<PropagationResponse>, (StatusCode, Json<ErrorResponse>)> {
    let payload_request = payload.map(|Json(inner)| inner).unwrap();

    let response = state
        .client
        .post("test")
        .json(&payload_request)
        .send()
        .await
        .map_err(|error| {
            (
                StatusCode::BAD_GATEWAY,
                Json(ErrorResponse {
                    error: format!("failed to reach orbit engine: {error}"),
                }),
            )
        })?;

    Ok(Json(PropagationResponse {}))
}
