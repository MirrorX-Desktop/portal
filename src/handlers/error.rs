use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

pub enum HttpError {
    Internal,
    Timeout,
    ResourceExhausted,
    RemoteOffline,
}

impl IntoResponse for HttpError {
    fn into_response(self) -> axum::response::Response {
        let error_code = match self {
            HttpError::Internal => 1,
            HttpError::Timeout => 2,
            HttpError::ResourceExhausted => 3,
            HttpError::RemoteOffline => 4,
        };

        let body = Json(json!({ "error_code": error_code }));

        (StatusCode::OK, body).into_response()
    }
}
