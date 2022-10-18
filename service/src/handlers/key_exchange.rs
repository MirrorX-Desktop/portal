use crate::handlers::CLIENTS;
use signaling_proto::message::{KeyExchangeRequest, KeyExchangeResponse};
use tonic::Status;

#[tracing::instrument]
pub async fn handle_key_exchange(req: KeyExchangeRequest) -> Result<KeyExchangeResponse, Status> {
    CLIENTS
        .get(&req.active_device_id)
        .ok_or_else(|| Status::not_found("passive device not found"))?
        .call_key_exchange_request(req.passive_device_id, req)
        .await
}
