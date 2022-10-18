use crate::handlers::CLIENTS;
use signaling_proto::message::{
    KeyExchangeReplyRequest, KeyExchangeReplyResponse, KeyExchangeResponse,
};
use tonic::Status;

#[tracing::instrument]
pub async fn handle_key_exchange_reply(
    req: KeyExchangeReplyRequest,
) -> Result<KeyExchangeReplyResponse, Status> {
    let response = KeyExchangeResponse {
        active_device_id: req.active_device_id,
        passive_device_id: req.passive_device_id,
        key_exchange_result: req.key_exchange_result,
    };

    CLIENTS
        .get(&req.active_device_id)
        .ok_or_else(|| Status::not_found("active device not found"))?
        .reply_call(req.active_device_id, Ok(response))
        .await;

    Ok(KeyExchangeReplyResponse {})
}
