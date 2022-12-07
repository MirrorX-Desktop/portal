use super::error::HttpError;
use axum::Json;
use rand::{distributions::Uniform, Rng};
use serde::{Deserialize, Serialize};
use std::ops::Range;

const DEVICE_ID_RANGE: Range<i64> = 1000000001..9999999999;

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    device_id: i64,
    device_finger_print: String,
}

#[derive(Debug, Serialize)]
pub struct RegisterResponse {
    device_id: i64,
    expire: i64,
}

#[tracing::instrument]
pub async fn register(
    Json(req): Json<RegisterRequest>,
) -> Result<Json<RegisterResponse>, HttpError> {
    if DEVICE_ID_RANGE.contains(&req.device_id) {
        let entity = crate::db::device::query_device_by_id(req.device_id)
            .await
            .map_err(|err| {
                tracing::error!(?err, "query_device_by_id");
                HttpError::Internal
            })?;

        // only renew device_id which expire is valid and device finger print equals the record
        if let Some(entity) = entity {
            if chrono::Utc::now().timestamp() <= entity.expire
                && req.device_finger_print == entity.finger_print
            {
                let new_expire = (chrono::Utc::now() + chrono::Duration::days(90)).timestamp();
                if let Err(err) =
                    crate::db::device::update_device_expire(req.device_id, new_expire).await
                {
                    tracing::error!(?err, "update_device_expire");
                    return Err(HttpError::Internal);
                }

                return Ok(Json(RegisterResponse {
                    device_id: req.device_id,
                    expire: new_expire,
                }));
            }
        }
    }

    if let Some(entity) = crate::db::device::query_device_by_finger_print(&req.device_finger_print)
        .await
        .map_err(|err| {
            tracing::error!(?err, "query_device_by_finger_print");
            HttpError::Internal
        })?
    {
        if entity.expire > chrono::Utc::now().timestamp() {
            let new_expire = (chrono::Utc::now() + chrono::Duration::days(90)).timestamp();
            if let Err(err) = crate::db::device::update_device_expire(entity.id, new_expire).await {
                tracing::error!(?err, "update_device_expire");
                return Err(HttpError::Internal);
            }

            return Ok(Json(RegisterResponse {
                device_id: entity.id,
                expire: new_expire,
            }));
        }
    };

    let reserve_device_ids: Vec<i64> = rand::thread_rng()
        .sample_iter(Uniform::from(DEVICE_ID_RANGE))
        .take(100)
        .collect();

    let non_available_device_ids = crate::db::device::query_device_non_available_ids(
        &reserve_device_ids,
        chrono::Utc::now().timestamp(),
    )
    .await
    .map_err(|err| {
        tracing::error!(?err, "query_device_non_available_ids");
        HttpError::Internal
    })?;

    let available_device_ids: Vec<i64> = reserve_device_ids
        .into_iter()
        .filter(|id| !non_available_device_ids.contains(id))
        .collect();

    for device_id in available_device_ids {
        let expire = (chrono::Utc::now() + chrono::Duration::days(90)).timestamp();

        if (crate::db::device::insert_device(device_id, &req.device_finger_print, expire).await)
            .is_ok()
        {
            return Ok(Json(RegisterResponse { device_id, expire }));
        }
    }

    Err(HttpError::ResourceExhausted)
}
