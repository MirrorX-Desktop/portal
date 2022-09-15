use crate::handlers::CLIENTS;
use signaling_proto::message::{VisitReplyRequest, VisitReplyResponse, VisitResponse};
use tonic::Status;

#[tracing::instrument]
pub async fn handle_visit_reply(req: VisitReplyRequest) -> Result<VisitReplyResponse, Status> {
    let passive_device_client = CLIENTS
        .get(&req.passive_device_id)
        .ok_or_else(|| Status::not_found("active device not found"))?;

    let response = VisitResponse {
        domain: req.domain,
        allow: req.allow,
    };

    passive_device_client
        .reply_call(req.active_device_id, response)
        .await;

    Ok(VisitReplyResponse {})
}
