use std::sync::Arc;

use actix_web::{web, HttpResponse};
use log::error;
use ring::{digest, hmac};
use serde::{Deserialize, Serialize};

use crate::{state::State, utility};

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterReq {
    pub device_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterResp {
    pub token: String,
}

pub async fn register(
    req: web::Json<RegisterReq>,
    state: web::Data<Arc<State>>,
) -> Result<HttpResponse, actix_web::Error> {
    if let Some(device_id) = &req.device_id {
        match state.redis.refresh_device_id(device_id) {
            Ok(Some(expire_at)) => {
                let sign_value =
                    sign_token(device_id.to_owned(), expire_at.to_string()).map_err(|err| {
                        error!("sign_token error: {}", err);
                        actix_web::error::ErrorInternalServerError(err)
                    })?;
                return Ok(HttpResponse::Ok().json(RegisterResp { token: sign_value }));
            }
            Err(err) => {
                error!("device_id_renew: {:?}", err);
                return Err(actix_web::error::ErrorInternalServerError(err));
            }
            _ => {}
        };
    }

    // allocate a new device id

    let mut failure_counter = 0;

    loop {
        // alphabet without 0, O, I, L
        let new_device_id = utility::device_id_generator::generate();

        match state.redis.pub_new_device_id(&new_device_id) {
            Ok(Some(expire_at)) => {
                let sign_value =
                    sign_token(new_device_id, expire_at.to_string()).map_err(|err| {
                        error!("sign_token error: {}", err);
                        actix_web::error::ErrorInternalServerError(err)
                    })?;
                return Ok(HttpResponse::Ok().json(RegisterResp { token: sign_value }));
            }
            Ok(None) => continue,
            Err(err) => {
                // only error increase fail counter
                failure_counter += 1;
                if failure_counter < 10 {
                    continue;
                }

                error!("too many failures, lastest error: {:?}", err);
                return Err(actix_web::error::ErrorInternalServerError(err));
            }
        };
    }
}

const SIGN_KEY: &'static [u8; digest::SHA256_OUTPUT_LEN] = b"MIRRORXAPI_SIGN_KEY_FOR_REGISTER";

fn sign_token(device_id: String, utc_timestamp: String) -> anyhow::Result<String> {
    let mut msg = vec![device_id, utc_timestamp].join(".");

    let skey = hmac::Key::new(hmac::HMAC_SHA256, SIGN_KEY);
    let tag = hmac::sign(&skey, msg.as_bytes());
    let sign_value = base64::encode(tag.as_ref());

    msg.push('.');
    msg.push_str(&sign_value);

    Ok(msg)
}
