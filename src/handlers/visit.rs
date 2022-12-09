use super::error::{HttpError, Response};
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

pub async fn visit(Json(req): Json<VisitRequest>) -> Response<VisitResponse> {
    let Some((mutex, subscribe_tx)) = SUBSCRIBERS.get(&req.passive_device_id) else {
        return Response::Error(HttpError::RemoteOffline);
    };

    let Ok(password_salt) = base64::decode(req.password_salt) else {
        return Response::Error(HttpError::InvalidArgs);
    };

    let Ok(secret) = base64::decode(req.secret) else {
        return Response::Error(HttpError::InvalidArgs);
    };

    let Ok(secret_nonce) = base64::decode(req.secret_nonce) else {
        return Response::Error(HttpError::InvalidArgs);
    };

    // hold mutex until timeout or visit call replied
    let Ok(_) = tokio::time::timeout(Duration::from_secs(60), mutex.lock()).await else {
        return Response::Error(HttpError::Timeout)
    };

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
        return Response::Error(HttpError::RemoteOffline);
    }

    match tokio::time::timeout(Duration::from_secs(60), call_rx.recv()).await {
        Ok(v) => match v {
            Some(result) => Response::Message(VisitResponse {
                result: result.map(base64::encode),
            }),
            None => Response::Error(HttpError::Timeout),
        },
        Err(_) => Response::Error(HttpError::Timeout),
    }
}
