use crate::handlers::CLIENTS;
use prost_reflect::ReflectMessage;
use signaling_proto::message::{
    KeyExchangeReplyRequest, KeyExchangeReplyResponse, KeyExchangeResponse,
};
use tonic::Status;

pub async fn handle_key_exchange_reply(
    req: KeyExchangeReplyRequest,
) -> Result<KeyExchangeReplyResponse, Status> {
    let passive_device_client = CLIENTS
        .get(&req.passive_device_id)
        .ok_or_else(|| Status::not_found("active device not found"))?;

    let response = KeyExchangeResponse {
        active_device_id: req.active_device_id,
        passive_device_id: req.passive_device_id,
        key_exchange_result: req.key_exchange_result,
    };

    let tx_key = (
        req.active_device_id,
        response.descriptor().full_name().to_owned(),
    );

    if let Some(tx) = passive_device_client.call_tx_map.get(&tx_key) {
        let _ = tx.send(response.transcode_to_dynamic());
    }

    Ok(KeyExchangeReplyResponse {})
}
