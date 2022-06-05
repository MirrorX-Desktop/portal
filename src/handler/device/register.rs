use crate::state::State;
use actix_web::{web, HttpResponse};
use log::error;
use once_cell::sync::Lazy;
use regex::Regex;
use ring::{digest, hmac};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

static DEVICE_ID_REGEX: Lazy<Regex> =
    Lazy::new(|| regex::Regex::new(r"^[0-9]{2}-[0-9]{4}-[0-9]{4}$").unwrap());

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
        if !DEVICE_ID_REGEX.is_match(device_id) {
            return Err(actix_web::error::ErrorBadRequest("device_id is invalid"));
        }

        let res = match state.redis.refresh_device_id(device_id).await {
            Ok(res) => res,
            Err(err) => {
                error!("refresh_device_id: {:?}", err);
                return Err(actix_web::error::ErrorInternalServerError(err));
            }
        };

        if let Some(new_expire_ts) = res {
            match sign_token(device_id.to_owned(), new_expire_ts.to_string()) {
                Ok(sign_value) => {
                    return Ok(HttpResponse::Ok().json(RegisterResp { token: sign_value }));
                }
                Err(err) => {
                    error!("sign_token error: {}", err);
                    return Err(actix_web::error::ErrorInternalServerError(err));
                }
            }
        }
    }

    match state.redis.new_device_id().await {
        Ok(res) => match sign_token(res.0.to_owned(), res.1.to_string()) {
            Ok(sign_value) => {
                return Ok(HttpResponse::Ok().json(RegisterResp { token: sign_value }));
            }
            Err(err) => {
                error!("sign_token error: {}", err);
                return Err(actix_web::error::ErrorInternalServerError(err));
            }
        },
        Err(err) => {
            error!("new_device_id: {:?}", err);
            return Err(actix_web::error::ErrorInternalServerError(err));
        }
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
