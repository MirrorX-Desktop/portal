use crate::handlers::CLIENTS;
use signaling_proto::message::{VisitReplyRequest, VisitReplyResponse, VisitResponse};
use tonic::Status;

#[tracing::instrument]
pub async fn handle_visit_reply(req: VisitReplyRequest) -> Result<VisitReplyResponse, Status> {
    let response = VisitResponse { allow: req.allow };

    CLIENTS
        .get(&req.active_device_id)
        .ok_or_else(|| Status::not_found("active device not found"))?
        .reply_call(req.passive_device_id, Ok(response))
        .await;

    Ok(VisitReplyResponse {})
}
