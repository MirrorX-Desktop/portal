use super::model::{EndPointStatResponse, StatDetails};
use crate::{
    component::metrics::SystemMetrics, db::device::query_device_id_count, DOMAIN,
    ENDPOINT_API_BASE_ADDRS,
};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use reqwest::Url;
use std::sync::Arc;

pub async fn details() -> impl IntoResponse {
    let allocated = match query_device_id_count().await {
        Ok(count) => count,
        Err(err) => {
            tracing::error!(?err, "query device id count failed");
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    let mut bytes_transferred = 0u64;
    let mut clients = None;

    for addr in ENDPOINT_API_BASE_ADDRS.iter() {
        let url = match Url::parse(addr) {
            Ok(url) => url,
            Err(err) => {
                tracing::error!(?err, "parse url failed");
                return StatusCode::INTERNAL_SERVER_ERROR.into_response();
            }
        };

        let url = match url.join("/api/stat") {
            Ok(url) => url,
            Err(err) => {
                tracing::error!(?err, "join url segments failed");
                return StatusCode::INTERNAL_SERVER_ERROR.into_response();
            }
        };

        let Ok(resp) = reqwest::get(url).await else {
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        };

        let Ok(resp) = resp.json::<EndPointStatResponse>().await else {
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        };

        bytes_transferred += resp.bytes_transferred;
        if clients.is_none() && !resp.client_snapshot.is_empty() {
            clients = Some(resp.client_snapshot);
        }
    }

    let response = Json(StatDetails {
        domain: &DOMAIN,
        allocated,
        bytes_transferred,
        client_snapshot: clients.unwrap_or_default(),
    });

    (StatusCode::OK, response).into_response()
}

pub async fn system_stat(State(metrics): State<Arc<SystemMetrics>>) -> impl IntoResponse {
    let metrics_info = metrics.get().await;
    let response = Json(metrics_info);
    (StatusCode::OK, response).into_response()
}
