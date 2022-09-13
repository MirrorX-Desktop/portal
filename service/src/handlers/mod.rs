mod heartbeat;
mod register;
mod subscribe;
mod visit;

use crate::handlers::heartbeat::handle_heartbeat;
use crate::handlers::register::handle_register;
use crate::handlers::subscribe::handle_subscribe;
use crate::handlers::visit::handle_visit;
use dashmap::DashMap;
use moka::sync::Cache;
use once_cell::sync::Lazy;
use prost_reflect::ReflectMessage;
use signaling_proto::message::publish_message::InnerPublishMessage;
use signaling_proto::message::{
    HeartbeatRequest, HeartbeatResponse, KeyExchangeReplyRequest, KeyExchangeReplyResponse,
    KeyExchangeRequest, KeyExchangeResponse, PublishMessage, RegisterRequest, RegisterResponse,
    SubscribeRequest, VisitReplyRequest, VisitReplyResponse, VisitRequest, VisitResponse,
};
use signaling_proto::service::signaling_server::Signaling;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc::Sender;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};

static CLIENTS: Lazy<DashMap<i64, Client>> = Lazy::new(DashMap::new);

#[derive(Debug, Default)]
pub struct SignalingService {}

#[tonic::async_trait]
impl Signaling for SignalingService {
    #[tracing::instrument]
    async fn register(
        &self,
        request: Request<RegisterRequest>,
    ) -> Result<Response<RegisterResponse>, Status> {
        let req = request.into_inner();
        handle_register(req).await.map(Response::new)
    }

    #[tracing::instrument]
    async fn heartbeat(
        &self,
        request: Request<HeartbeatRequest>,
    ) -> Result<Response<HeartbeatResponse>, Status> {
        let req = request.into_inner();
        handle_heartbeat(req).await.map(Response::new)
    }

    async fn visit(
        &self,
        request: Request<VisitRequest>,
    ) -> Result<Response<VisitResponse>, Status> {
        let req = request.into_inner();
        handle_visit(req).await.map(Response::new)
    }

    async fn visit_reply(
        &self,
        request: Request<VisitReplyRequest>,
    ) -> Result<Response<VisitReplyResponse>, Status> {
        todo!()
    }

    async fn key_exchange(
        &self,
        request: Request<KeyExchangeRequest>,
    ) -> Result<Response<KeyExchangeResponse>, Status> {
        todo!()
    }

    async fn key_exchange_reply(
        &self,
        request: Request<KeyExchangeReplyRequest>,
    ) -> Result<Response<KeyExchangeReplyResponse>, Status> {
        todo!()
    }

    type SubscribeStream = ReceiverStream<Result<PublishMessage, tonic::Status>>;

    async fn subscribe(
        &self,
        request: Request<SubscribeRequest>,
    ) -> Result<Response<Self::SubscribeStream>, Status> {
        let req = request.into_inner();
        handle_subscribe(req)
            .await
            .map(|rx| Response::new(ReceiverStream::new(rx)))
    }
}

struct Client {
    device_id: i64,
    finger_print: String,
    tx: Sender<Result<PublishMessage, Status>>,
    call_tx_map: Cache<(i64, String), Arc<tokio::sync::oneshot::Sender<InnerPublishMessage>>>,
}

impl Client {
    pub fn new(
        device_id: i64,
        finger_print: String,
        tx: Sender<Result<PublishMessage, Status>>,
    ) -> Self {
        let call_tx_map = Cache::builder()
            .time_to_live(Duration::from_secs(60 * 3))
            .build();

        Client {
            device_id,
            finger_print,
            tx,
            call_tx_map,
        }
    }

    // pub async fn call_visit_request(&self, message: VisitRequest) -> Result<VisitResponse, Status> {
    //     let message_name = message.descriptor().full_name().to_string();
    //
    //     let publish_message = PublishMessage {
    //         inner_publish_message: Some(InnerPublishMessage::VisitRequest(message)),
    //     };
    //
    //     let (tx, rx) = tokio::sync::oneshot::channel();
    //
    //     self.call_tx_map
    //         .insert((self.device_id, message_name), Arc::new(tx));
    //
    //     todo!()
    // }

    pub async fn push(&self, message: Result<PublishMessage, Status>) -> Result<(), Status> {
        self.tx.send(message).await.map_err(|err| {
            let device_id = self.device_id;
            tracing::error!(?device_id, ?err, "push message failed");
            Status::internal("signaling exchange message failed")
        })
    }
}
