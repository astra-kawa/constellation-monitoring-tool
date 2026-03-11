use crate::http::dto::MessageResponse;
use axum::Json;

pub async fn get_health() -> Json<MessageResponse> {
    Json(MessageResponse {
        message: "ok".to_owned(),
    })
}
