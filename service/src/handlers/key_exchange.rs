use crate::handlers::CLIENTS;
use signaling_proto::message::{KeyExchangeRequest, KeyExchangeResponse};
use tonic::Status;

pub async fn handle_key_exchange(req: KeyExchangeRequest) -> Result<KeyExchangeResponse, Status> {
    let passive_device_client = CLIENTS
        .get(&req.passive_device_id)
        .ok_or_else(|| Status::not_found("passive device not found"))?;

    passive_device_client
        .call_key_exchange_request(req.active_device_id, req)
        .await
}
