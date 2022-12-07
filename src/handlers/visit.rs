use super::error::HttpError;
use crate::{
    message::{ServerMessage, VisitFailureReason},
    subscriber::{CALLS, SUBSCRIBERS},
};
use axum::Json;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Deserialize)]
pub struct VisitRequest {
    active_device_id: i64,
    passive_device_id: i64,
    visit_desktop: bool,
    password_salt: String,
    secret: String,
    secret_nonce: String,
}

#[derive(Serialize)]
pub struct VisitResponse {
    result: Result<String, VisitFailureReason>,
}

pub async fn visit(Json(req): Json<VisitRequest>) -> Result<Json<VisitResponse>, HttpError> {
    let Some((mutex, subscribe_tx)) = SUBSCRIBERS.get(&req.passive_device_id) else {
        return Err(HttpError::RemoteOffline);
    };

    let password_salt = base64::decode(req.password_salt).map_err(|_| HttpError::Internal)?;
    let secret = base64::decode(req.secret).map_err(|_| HttpError::Internal)?;
    let secret_nonce = base64::decode(req.secret_nonce).map_err(|_| HttpError::Internal)?;

    // hold mutex until timeout or visit call replied
    let _ = tokio::time::timeout(Duration::from_secs(60), mutex.lock())
        .await
        .map_err(|_| HttpError::Timeout)?;

    // register visit call
    let (call_tx, mut call_rx) = tokio::sync::mpsc::channel(1);

    CALLS
        .insert((req.active_device_id, req.passive_device_id), call_tx)
        .await;

    if subscribe_tx
        .send(ServerMessage::VisitRequest {
            active_device_id: req.active_device_id,
            passive_device_id: req.passive_device_id,
            visit_desktop: req.visit_desktop,
            password_salt,
            secret,
            secret_nonce,
        })
        .await
        .is_err()
    {
        return Err(HttpError::RemoteOffline);
    }

    let result = tokio::time::timeout(Duration::from_secs(60), call_rx.recv())
        .await
        .map_err(|_| HttpError::Timeout)?
        .ok_or(HttpError::Timeout)?
        .map(base64::encode);

    Ok(Json(VisitResponse { result }))
}
