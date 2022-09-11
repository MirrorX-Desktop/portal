use super::{
    client::CLIENTS,
    message::{
        HandshakeRequest, HandshakeResponse, HeartBeatRequest, HeartBeatResponse,
        SignalingMessageError,
    },
};
use chrono::TimeZone;
use log::error;
use rand::Rng;

pub async fn handle_heartbeat(
    _: HeartBeatRequest,
) -> Result<HeartBeatResponse, SignalingMessageError> {
    Ok(HeartBeatResponse { time_stamp: 0 })
}

pub async fn handle_handshake(
    req: HandshakeRequest,
) -> Result<HandshakeResponse, SignalingMessageError> {
    if let Some(device_id) = req.device_id {
        if CLIENTS.contains_key(&device_id) {
            error!("client with device id '{}' already exists", device_id);
            return Err(SignalingMessageError::Internal);
        }

        let device_id_num = device_id.parse::<i64>().map_err(|err| {
            error!(
                "handle_handshake: can not convert device id from String to i32 ({:?})",
                err
            );
            SignalingMessageError::Internal
        })?;

        if let Some(entity) = crate::db::query_device_by_id(device_id_num)
            .await
            .map_err(|err| {
                error!("handle_handshake: query device by id failed ({:?})", err);
                SignalingMessageError::Internal
            })?
        {
            let dt = chrono::Utc::now()
                .signed_duration_since(chrono::Utc.ymd(2022, 1, 1).and_hms(0, 0, 0))
                + chrono::Duration::days(90);

            if entity.device_hash == req.device_hash {
                for _ in 0..5 {
                    if let Ok(_) =
                        crate::db::update_device_expire(device_id_num, dt.num_seconds() as i32)
                            .await
                    {
                        return Ok(HandshakeResponse {
                            device_id,
                            expire: dt.num_seconds() as i32,
                        });
                    }

                    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                }

                error!("handle_handshake: too many failures while update device expire time");
                return Err(SignalingMessageError::Internal);
            }
        }
    }

    for _ in 0..10 {
        let device_id_range_min: i64 = rand::thread_rng().gen_range(0100000001..=9999999979);
        let free_ids = crate::db::query_free_ids(device_id_range_min)
            .await
            .map_err(|err| {
                error!("handle_handshake: query free ids failed ({:?})", err);
                SignalingMessageError::Internal
            })?;

        for id in free_ids {
            let expire = (chrono::Utc::now()
                .signed_duration_since(chrono::Utc.ymd(2022, 1, 1).and_hms(0, 0, 0))
                + chrono::Duration::days(90))
            .num_seconds() as i32;

            match crate::db::insert_device(&crate::db::entities::Device {
                id: id.clone(),
                device_hash: req.device_hash.clone(),
                expire,
            })
            .await
            {
                Ok(_) => {
                    return Ok(HandshakeResponse {
                        device_id: format!("{:0>10}", id),
                        expire,
                    })
                }
                Err(err) => {
                    error!("{}", err);
                }
            }
        }

        tokio::time::sleep(std::time::Duration::from_millis(10)).await;
    }

    error!("handle_handshake: too many failures while generate new device_id");
    Err(SignalingMessageError::Internal)
}
