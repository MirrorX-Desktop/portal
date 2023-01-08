use super::error::{HttpError, Response};
use crate::{SIGNALING_PORT, SUBSCRIBE_PORT};
use serde::Serialize;

#[derive(Serialize)]
pub struct IdentityResponse {
    domain: String,
    min_client_version: String,
    signaling_port: u16,
    subscribe_port: u16,
}

#[tracing::instrument]
pub async fn identity() -> Response<IdentityResponse> {
    let Ok(domain) = std::env::var("DOMAIN") else {
        tracing::error!("'Domain' not exists in env");
        return Response::Error(HttpError::Internal);
    };

    let Ok(min_client_version) = std::env::var("MIN_CLIENT_VERSION") else {
        tracing::error!("'MIN_CLIENT_VERSION' not exists in env");
        return Response::Error(HttpError::Internal);
    };

    Response::Message(IdentityResponse {
        domain,
        min_client_version,
        signaling_port: *SIGNALING_PORT,
        subscribe_port: *SUBSCRIBE_PORT,
    })
}
