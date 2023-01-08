use super::error::{HttpError, Response};
use crate::{
    message::{ServerMessage, VisitFailureReason},
    subscriber::{CALLS, SUBSCRIBERS},
};
use axum::Json;
use rand::Rng;
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
    endpoint_addr: String,
    visit_credentials: String,
    result: Result<String, VisitFailureReason>,
}

pub async fn visit(Json(req): Json<VisitRequest>) -> Response<VisitResponse> {
    let Ok(endpoints_env) = std::env::var("ENDPOINTS") else {
        return Response::Error(HttpError::Internal);
    };

    let endpoint_addrs: Vec<&str> = endpoints_env.split(',').map(|s| s.trim()).collect();

    if endpoint_addrs.is_empty() {
        return Response::Error(HttpError::Internal);
    }

    let endpoint_addr = endpoint_addrs[rand::thread_rng().gen_range(0..endpoint_addrs.len())];

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

    let mut visit_credentials = [0u8; 32];
    rand::thread_rng().fill(&mut visit_credentials);

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
            endpoint_addr: endpoint_addr.to_string(),
            password_salt,
            secret,
            secret_nonce,
            passive_visit_credentials: visit_credentials.to_vec(),
        })
        .await
        .is_err()
    {
        return Response::Error(HttpError::RemoteOffline);
    }

    match tokio::time::timeout(Duration::from_secs(60), call_rx.recv()).await {
        Ok(v) => match v {
            Some(result) => Response::Message(VisitResponse {
                endpoint_addr: endpoint_addr.to_string(),
                visit_credentials: base64::encode(visit_credentials),
                result: result.map(base64::encode),
            }),
            None => Response::Error(HttpError::Timeout),
        },
        Err(_) => Response::Error(HttpError::Timeout),
    }
}
