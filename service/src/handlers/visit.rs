use crate::handlers::CLIENTS;
use signaling_proto::message::{VisitRequest, VisitResponse};
use tonic::Status;

#[tracing::instrument]
pub async fn handle_visit(req: VisitRequest) -> Result<VisitResponse, Status> {
    let passive_device_client = CLIENTS
        .get(&req.passive_device_id)
        .ok_or_else(|| Status::not_found("passive device not found"))?;

    passive_device_client
        .call_visit_request(req.active_device_id, req)
        .await
}
