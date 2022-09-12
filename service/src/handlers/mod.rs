mod register;

use crate::handlers::register::handle_register;
use dashmap::DashMap;
use once_cell::sync::Lazy;
use signaling_proto::{
    signaling_server::Signaling, HeartbeatRequest, HeartbeatResponse, KeyExchangeReplyRequest,
    KeyExchangeReplyResponse, KeyExchangeRequest, KeyExchangeResponse, RegisterRequest,
    RegisterResponse, SubscribeRequest, VisitReplyRequest, VisitReplyResponse, VisitRequest,
    VisitResponse,
};
use tonic::{Request, Response, Status};

static CLIENTS: Lazy<DashMap<u64, ()>> = Lazy::new(|| DashMap::new());

pub struct SignalingService {}

#[tonic::async_trait]
impl Signaling for SignalingService {
    async fn register(
        &self,
        request: Request<RegisterRequest>,
    ) -> Result<Response<RegisterResponse>, Status> {
        let req = request.into_inner();
        handle_register(req)
    }

    async fn heartbeat(
        &self,
        request: Request<HeartbeatRequest>,
    ) -> Result<Response<HeartbeatResponse>, Status> {
        Err(Status::internal("message"))
    }

    async fn visit(
        &self,
        request: Request<VisitRequest>,
    ) -> Result<Response<VisitResponse>, Status> {
        todo!()
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

    type SubscribeStream = ();

    async fn subscribe(
        &self,
        request: Request<SubscribeRequest>,
    ) -> Result<Response<Self::SubscribeStream>, Status> {
        todo!()
    }
}
