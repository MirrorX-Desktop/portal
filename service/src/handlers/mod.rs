mod heartbeat;
mod key_exchange;
mod key_exchange_reply;
mod register;
mod subscribe;
mod visit;
mod visit_reply;

use self::{
    key_exchange::handle_key_exchange, key_exchange_reply::handle_key_exchange_reply,
    visit_reply::handle_visit_reply,
};
use crate::handlers::{
    heartbeat::handle_heartbeat, register::handle_register, subscribe::handle_subscribe,
    visit::handle_visit,
};
use dashmap::DashMap;
use moka::sync::Cache;
use once_cell::sync::Lazy;
use prost::Message;
use prost_reflect::{DynamicMessage, ReflectMessage};
use scopeguard::defer;
use signaling_proto::{
    message::{
        publish_message::InnerPublishMessage, HeartbeatRequest, HeartbeatResponse,
        KeyExchangeReplyRequest, KeyExchangeReplyResponse, KeyExchangeRequest, KeyExchangeResponse,
        PublishMessage, RegisterRequest, RegisterResponse, SubscribeRequest, VisitReplyRequest,
        VisitReplyResponse, VisitRequest, VisitResponse,
    },
    service::signaling_server::Signaling,
};
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
        let req = request.into_inner();
        handle_visit_reply(req).await.map(Response::new)
    }

    async fn key_exchange(
        &self,
        request: Request<KeyExchangeRequest>,
    ) -> Result<Response<KeyExchangeResponse>, Status> {
        let req = request.into_inner();
        handle_key_exchange(req).await.map(Response::new)
    }

    async fn key_exchange_reply(
        &self,
        request: Request<KeyExchangeReplyRequest>,
    ) -> Result<Response<KeyExchangeReplyResponse>, Status> {
        let req = request.into_inner();
        handle_key_exchange_reply(req).await.map(Response::new)
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
    tx: Sender<Result<PublishMessage, Status>>,
    call_tx_map: DashMap<(i64, String), Sender<DynamicMessage>>,
}

impl Client {
    pub fn new(device_id: i64, tx: Sender<Result<PublishMessage, Status>>) -> Self {
        Client {
            device_id,
            tx,
            call_tx_map: DashMap::new(),
        }
    }

    pub async fn call_visit_request(
        &self,
        caller_device_id: i64,
        message: VisitRequest,
    ) -> Result<VisitResponse, Status> {
        self.publish_message(
            caller_device_id,
            PublishMessage {
                inner_publish_message: Some(InnerPublishMessage::VisitRequest(message)),
            },
        )
        .await
    }

    pub async fn call_key_exchange_request(
        &self,
        caller_device_id: i64,
        message: KeyExchangeRequest,
    ) -> Result<KeyExchangeResponse, Status> {
        self.publish_message(
            caller_device_id,
            PublishMessage {
                inner_publish_message: Some(InnerPublishMessage::KeyExchangeRequest(message)),
            },
        )
        .await
    }

    async fn publish_message<ResponseMessage>(
        &self,
        caller_device_id: i64,
        message: PublishMessage,
    ) -> Result<ResponseMessage, Status>
    where
        ResponseMessage: Message + Default,
    {
        let message_name = message.descriptor().full_name().to_string();
        let tx_key = (caller_device_id, message_name.to_owned());

        if self.call_tx_map.contains_key(&tx_key) {
            return Err(Status::already_exists("disallow repeat request"));
        }

        let (resp_tx, mut resp_rx) = tokio::sync::mpsc::channel(1);

        self.call_tx_map.insert(tx_key, resp_tx);
        defer! {
            self.call_tx_map.remove(&(caller_device_id, message_name.to_owned()));
        }

        self.tx.send(Ok(message)).await.map_err(|err| {
            let device_id = self.device_id;
            tracing::error!(?device_id, ?err, "publish message failed");
            Status::internal("signaling exchange message failed")
        })?;

        let resp = tokio::time::timeout(Duration::from_secs(60), resp_rx.recv())
            .await
            .map_err(|_| Status::deadline_exceeded("request timeout"))?
            .ok_or_else(|| Status::deadline_exceeded("request timeout"))?;

        resp.transcode_to::<ResponseMessage>()
            .map_err(|_| Status::internal("internal incorrect message dispatch"))
    }
}
