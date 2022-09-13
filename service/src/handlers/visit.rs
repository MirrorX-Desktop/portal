use crate::handlers::{Client, CLIENTS};
use dashmap::mapref::one::Ref;
use signaling_proto::message::publish_message::InnerPublishMessage;
use signaling_proto::message::{PublishMessage, VisitRequest, VisitResponse};
use tonic::Status;

pub async fn handle_visit(req: VisitRequest) -> Result<VisitResponse, Status> {
    let active_device_client = match CLIENTS.get(&req.active_device_id) {
        Some(client) => client,
        None => return Err(Status::not_found("active device not found")),
    };

    let passive_device_client = match CLIENTS.get(&req.passive_device_id) {
        Some(client) => client,
        None => return Err(Status::not_found("passive device not found")),
    };

    let passive_publish_message = PublishMessage {
        inner_publish_message: Some(InnerPublishMessage::VisitRequest(req.clone())),
    };

    passive_device_client
        .push(Ok(passive_publish_message))
        .await?;

    todo!()
}
