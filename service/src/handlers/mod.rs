mod get_domain;
mod key_exchange;
mod key_exchange_reply;
mod register;
mod subscribe;
mod visit;
mod visit_reply;

use self::{
    get_domain::handle_get_domain, key_exchange::handle_key_exchange,
    key_exchange_reply::handle_key_exchange_reply, visit_reply::handle_visit_reply,
};
use crate::handlers::{
    register::handle_register, subscribe::handle_subscribe, visit::handle_visit,
};
use dashmap::DashMap;
use futures::Stream;
use once_cell::sync::Lazy;
use prost_reflect::{DynamicMessage, ReflectMessage};
use scopeguard::defer;
use signaling_proto::{message::*, service::signaling_server::Signaling};
use std::{collections::HashMap, time::Duration};
use tokio::sync::{
    mpsc::{Receiver, Sender},
    Mutex,
};
use tonic::{Request, Response, Status};

static CLIENTS: Lazy<DashMap<i64, Client>> = Lazy::new(DashMap::new);

#[derive(Debug, Default)]
pub struct SignalingService {}

#[tonic::async_trait]
impl Signaling for SignalingService {
    #[tracing::instrument]
    async fn get_domain(
        &self,
        request: Request<GetDomainRequest>,
    ) -> Result<Response<GetDomainResponse>, Status> {
        let req = request.into_inner();
        handle_get_domain(req).await.map(Response::new)
    }

    #[tracing::instrument]
    async fn register(
        &self,
        request: Request<RegisterRequest>,
    ) -> Result<Response<RegisterResponse>, Status> {
        let req = request.into_inner();
        handle_register(req).await.map(Response::new)
    }

    #[tracing::instrument]
    async fn visit(
        &self,
        request: Request<VisitRequest>,
    ) -> Result<Response<VisitResponse>, Status> {
        let req = request.into_inner();
        handle_visit(req).await.map(Response::new)
    }

    #[tracing::instrument]
    async fn visit_reply(
        &self,
        request: Request<VisitReplyRequest>,
    ) -> Result<Response<VisitReplyResponse>, Status> {
        let req = request.into_inner();
        handle_visit_reply(req).await.map(Response::new)
    }

    #[tracing::instrument]
    async fn key_exchange(
        &self,
        request: Request<KeyExchangeRequest>,
    ) -> Result<Response<KeyExchangeResponse>, Status> {
        let req = request.into_inner();
        handle_key_exchange(req).await.map(Response::new)
    }

    #[tracing::instrument]
    async fn key_exchange_reply(
        &self,
        request: Request<KeyExchangeReplyRequest>,
    ) -> Result<Response<KeyExchangeReplyResponse>, Status> {
        let req = request.into_inner();
        handle_key_exchange_reply(req).await.map(Response::new)
    }

    type SubscribeStream = ObserveStream<Result<PublishMessage, tonic::Status>>;

    #[tracing::instrument]
    async fn subscribe(
        &self,
        request: Request<SubscribeRequest>,
    ) -> Result<Response<Self::SubscribeStream>, Status> {
        let req = request.into_inner();
        handle_subscribe(req)
            .await
            .map(|(device_id, rx)| Response::new(ObserveStream::new(device_id, rx)))
    }
}

struct Client {
    device_id: i64,
    tx: Sender<Result<PublishMessage, Status>>,
    call_tx_map: Mutex<HashMap<(i64, String), Sender<Result<DynamicMessage, Status>>>>,
}

impl Client {
    pub fn new(device_id: i64, tx: Sender<Result<PublishMessage, Status>>) -> Self {
        Client {
            device_id,
            tx,
            call_tx_map: Mutex::new(HashMap::new()),
        }
    }

    pub async fn call_visit_request(
        &self,
        remote_device_id: i64,
        message: VisitRequest,
    ) -> Result<VisitResponse, Status> {
        self.publish_message(
            remote_device_id,
            PublishMessage {
                inner_publish_message: Some(publish_message::InnerPublishMessage::VisitRequest(
                    message,
                )),
            },
        )
        .await
    }

    pub async fn call_key_exchange_request(
        &self,
        remote_device_id: i64,
        message: KeyExchangeRequest,
    ) -> Result<KeyExchangeResponse, Status> {
        self.publish_message(
            remote_device_id,
            PublishMessage {
                inner_publish_message: Some(
                    publish_message::InnerPublishMessage::KeyExchangeRequest(message),
                ),
            },
        )
        .await
    }

    async fn reply_call<ResponseMessage>(
        &self,
        reply_device_id: i64,
        reply: Result<ResponseMessage, Status>,
    ) where
        ResponseMessage: ReflectMessage + Default,
    {
        let tx_key = (
            reply_device_id,
            ResponseMessage::default()
                .descriptor()
                .full_name()
                .to_owned(),
        );

        if let Some(tx) = self.call_tx_map.lock().await.remove(&tx_key) {
            let reply = reply.map(|message| message.transcode_to_dynamic());
            if tx.send(reply).await.is_err() {
                tracing::warn!(?tx_key, "tx send failed");
            }
        } else {
            tracing::warn!(?tx_key, "reply tx not exists");
        }
    }

    async fn publish_message<ResponseMessage>(
        &self,
        remote_device_id: i64,
        message: PublishMessage,
    ) -> Result<ResponseMessage, Status>
    where
        ResponseMessage: ReflectMessage + Default,
    {
        let response_message_name = ResponseMessage::default()
            .descriptor()
            .full_name()
            .to_string();

        let tx_key = (remote_device_id, response_message_name.to_owned());
        let (resp_tx, mut resp_rx) = tokio::sync::mpsc::channel(1);

        let mut call_tx_map = self.call_tx_map.lock().await;

        if call_tx_map.contains_key(&tx_key) {
            return Err(Status::already_exists("disallow repeat request"));
        }

        call_tx_map.insert(tx_key.clone(), resp_tx);

        drop(call_tx_map);

        if let Some(remote_client) = CLIENTS.get(&remote_device_id) {
            if let Err(err) = remote_client.tx.try_send(Ok(message)) {
                tracing::error!(
                    from_device = self.device_id,
                    to_device = remote_client.device_id,
                    ?err,
                    "publish message failed"
                );
                return Err(Status::internal("signaling exchange message failed"));
            }
        } else {
            return Err(Status::aborted("remote device disconnected"));
        }

        let resp = tokio::time::timeout(Duration::from_secs(60), resp_rx.recv())
            .await
            .map_err(|_| Status::deadline_exceeded("request timeout"))?
            .unwrap_or_else(|| Err(Status::aborted("remote device disconnected")))?;

        let resp = resp
            .transcode_to::<ResponseMessage>()
            .map_err(|_| Status::internal("internal incorrect message dispatch"))?;

        Ok(resp)
    }
}

pub struct ObserveStream<T> {
    device_id: i64,
    inner: Receiver<T>,
}

impl<T> ObserveStream<T> {
    pub fn new(device_id: i64, receiver: Receiver<T>) -> Self {
        ObserveStream {
            device_id,
            inner: receiver,
        }
    }
}

impl<T> Stream for ObserveStream<T> {
    type Item = T;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        self.inner.poll_recv(cx)
    }
}

impl<T> Drop for ObserveStream<T> {
    fn drop(&mut self) {
        let device_id = self.device_id;
        tokio::spawn(async move {
            // cancel all client's calls otherwise they'll cause DEADLOCK to CLIENTS behind DashMap!
            if let Some(client) = CLIENTS.get(&device_id) {
                for tx in client.call_tx_map.lock().await.values() {
                    tx.closed().await;
                }
            }

            // Be careful here will be DEADLOCK if any rpc call is awaiting
            let _ = CLIENTS.remove(&device_id);
            tracing::debug!(?device_id, "client drop");
        });
    }
}
