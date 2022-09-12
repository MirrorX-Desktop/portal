use crate::db::entities::Device;
use crate::handlers::CLIENTS;
use chrono::{TimeZone, Utc};
use rand::Rng;
use signaling_proto::{RegisterRequest, RegisterResponse};
use tonic::Status;
use tracing_subscriber::fmt::format;

pub async fn handle_register(req: RegisterRequest) -> Result<RegisterResponse, Status> {
    if let Some(device_id) = req.device_id {
        if CLIENTS.contains_key(&device_id) {
            return Err(Status::already_exists(None));
        }

        let entity = crate::db::query_device_by_id(device_id)
            .await
            .map_err(|_| Status::internal(None))?;

        // only renew device_id which expire is valid and device finger print equals the record
        if let Some(entity) = entity {
            if chrono::Utc::now().timestamp() <= entity.expire {
                if req.device_finger_print == entity.finger_print {
                    let new_expire = (chrono::Utc::now() + chrono::Duration::days(90)).timestamp();
                    if let Err(err) = crate::db::update_device_expire(device_id, new_expire) {
                        tracing::error!(?err, "renew device id expire failed");
                        return Err(Status::internal(None));
                    }

                    return Ok(RegisterResponse {
                        device_id,
                        expire: new_expire,
                    });
                }
            }
        }
    }

    let reserve_device_ids: Vec<u64> = rand::thread_rng()
        .sample_iter(0100000001..=8999999999)
        .take(100)
        .collect();

    let available_device_ids =
        crate::db::query_device_available_ids(&reserve_device_ids, chrono::Utc::now().timestamp())
            .await
            .map_err(|err| {
                tracing::error!(?err, "query available ids failed");
                Status::internal(None)
            })?;

    for device_id in available_device_ids {
        let expire = (chrono::Utc::now() + chrono::Duration::days(90)).timestamp();

        if let Ok(_) = crate::db::insert_device(device_id, &req.device_finger_print, expire).await {
            return Ok(RegisterResponse {
                device_id,
                expire: new_expire,
            });
        }
    }

    Err(Status::resource_exhausted(None))
}
