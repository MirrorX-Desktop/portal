use crate::handlers::CLIENTS;
use rand::distributions::Uniform;
use rand::Rng;
use signaling_proto::message::{RegisterRequest, RegisterResponse};
use tonic::Status;

#[tracing::instrument]
pub async fn handle_register(req: RegisterRequest) -> Result<RegisterResponse, Status> {
    let domain = std::env::var("DOMAIN").map_err(|_| Status::internal("server internal error"))?;

    if let Some(device_id) = req.device_id {
        if CLIENTS.contains_key(&device_id) {
            return Err(Status::already_exists(
                "register device id already connected to signaling server",
            ));
        }

        let entity = crate::db::device::query_device_by_id(device_id)
            .await
            .map_err(|err| {
                tracing::error!(?err, "query_device_by_id");
                Status::internal("internal database query error")
            })?;

        // only renew device_id which expire is valid and device finger print equals the record
        if let Some(entity) = entity {
            if chrono::Utc::now().timestamp() <= entity.expire
                && req.device_finger_print == entity.finger_print
            {
                let new_expire = (chrono::Utc::now() + chrono::Duration::days(90)).timestamp();
                if let Err(err) =
                    crate::db::device::update_device_expire(device_id, new_expire).await
                {
                    tracing::error!(?err, "update_device_expire");
                    return Err(Status::internal("internal database query error"));
                }

                return Ok(RegisterResponse {
                    domain,
                    device_id,
                    expire: new_expire,
                });
            }
        }
    }

    if let Some(entity) = crate::db::device::query_device_by_finger_print(&req.device_finger_print)
        .await
        .map_err(|err| {
            tracing::error!(?err, "query_device_by_finger_print");
            Status::internal("internal database query error")
        })?
    {
        if entity.expire > chrono::Utc::now().timestamp() {
            let new_expire = (chrono::Utc::now() + chrono::Duration::days(90)).timestamp();
            if let Err(err) = crate::db::device::update_device_expire(entity.id, new_expire).await {
                tracing::error!(?err, "update_device_expire");
                return Err(Status::internal("internal database query error"));
            }

            return Ok(RegisterResponse {
                domain,
                device_id: entity.id,
                expire: new_expire,
            });
        }
    };

    let device_id_range = Uniform::from(1000000001..9999999999);
    let reserve_device_ids: Vec<i64> = rand::thread_rng()
        .sample_iter(device_id_range)
        .take(100)
        .collect();

    let non_available_device_ids = crate::db::device::query_device_non_available_ids(
        &reserve_device_ids,
        chrono::Utc::now().timestamp(),
    )
    .await
    .map_err(|err| {
        tracing::error!(?err, "query_device_non_available_ids");
        Status::internal("internal database query error")
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
            return Ok(RegisterResponse {
                domain,
                device_id,
                expire,
            });
        }
    }

    Err(Status::resource_exhausted("too many failures"))
}
