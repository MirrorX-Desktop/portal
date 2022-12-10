use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum HttpError {
    Internal,
    Timeout,
    InvalidArgs,
    ResourceExhausted,
    RemoteOffline,
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum Response<T>
where
    T: Serialize,
{
    Message(T),
    Error(HttpError),
}

impl<T: Serialize> IntoResponse for Response<T> {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}
