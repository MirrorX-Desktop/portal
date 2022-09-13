/// Generated client implementations.
pub mod signaling_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct SignalingClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl SignalingClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> SignalingClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> SignalingClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            SignalingClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        pub async fn register(
            &mut self,
            request: impl tonic::IntoRequest<super::super::message::RegisterRequest>,
        ) -> Result<
            tonic::Response<super::super::message::RegisterResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/service.Signaling/Register",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn heartbeat(
            &mut self,
            request: impl tonic::IntoRequest<super::super::message::HeartbeatRequest>,
        ) -> Result<
            tonic::Response<super::super::message::HeartbeatResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/service.Signaling/Heartbeat",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn visit(
            &mut self,
            request: impl tonic::IntoRequest<super::super::message::VisitRequest>,
        ) -> Result<
            tonic::Response<super::super::message::VisitResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/service.Signaling/Visit");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn visit_reply(
            &mut self,
            request: impl tonic::IntoRequest<super::super::message::VisitReplyRequest>,
        ) -> Result<
            tonic::Response<super::super::message::VisitReplyResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/service.Signaling/VisitReply",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn key_exchange(
            &mut self,
            request: impl tonic::IntoRequest<super::super::message::KeyExchangeRequest>,
        ) -> Result<
            tonic::Response<super::super::message::KeyExchangeResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/service.Signaling/KeyExchange",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn key_exchange_reply(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::message::KeyExchangeReplyRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::message::KeyExchangeReplyResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/service.Signaling/KeyExchangeReply",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn subscribe(
            &mut self,
            request: impl tonic::IntoRequest<super::super::message::SubscribeRequest>,
        ) -> Result<
            tonic::Response<
                tonic::codec::Streaming<super::super::message::PublishMessage>,
            >,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/service.Signaling/Subscribe",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod signaling_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with SignalingServer.
    #[async_trait]
    pub trait Signaling: Send + Sync + 'static {
        async fn register(
            &self,
            request: tonic::Request<super::super::message::RegisterRequest>,
        ) -> Result<
            tonic::Response<super::super::message::RegisterResponse>,
            tonic::Status,
        >;
        async fn heartbeat(
            &self,
            request: tonic::Request<super::super::message::HeartbeatRequest>,
        ) -> Result<
            tonic::Response<super::super::message::HeartbeatResponse>,
            tonic::Status,
        >;
        async fn visit(
            &self,
            request: tonic::Request<super::super::message::VisitRequest>,
        ) -> Result<
            tonic::Response<super::super::message::VisitResponse>,
            tonic::Status,
        >;
        async fn visit_reply(
            &self,
            request: tonic::Request<super::super::message::VisitReplyRequest>,
        ) -> Result<
            tonic::Response<super::super::message::VisitReplyResponse>,
            tonic::Status,
        >;
        async fn key_exchange(
            &self,
            request: tonic::Request<super::super::message::KeyExchangeRequest>,
        ) -> Result<
            tonic::Response<super::super::message::KeyExchangeResponse>,
            tonic::Status,
        >;
        async fn key_exchange_reply(
            &self,
            request: tonic::Request<super::super::message::KeyExchangeReplyRequest>,
        ) -> Result<
            tonic::Response<super::super::message::KeyExchangeReplyResponse>,
            tonic::Status,
        >;
        ///Server streaming response type for the Subscribe method.
        type SubscribeStream: futures_core::Stream<
                Item = Result<super::super::message::PublishMessage, tonic::Status>,
            >
            + Send
            + 'static;
        async fn subscribe(
            &self,
            request: tonic::Request<super::super::message::SubscribeRequest>,
        ) -> Result<tonic::Response<Self::SubscribeStream>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct SignalingServer<T: Signaling> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Signaling> SignalingServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for SignalingServer<T>
    where
        T: Signaling,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/service.Signaling/Register" => {
                    #[allow(non_camel_case_types)]
                    struct RegisterSvc<T: Signaling>(pub Arc<T>);
                    impl<
                        T: Signaling,
                    > tonic::server::UnaryService<super::super::message::RegisterRequest>
                    for RegisterSvc<T> {
                        type Response = super::super::message::RegisterResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::message::RegisterRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).register(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RegisterSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/service.Signaling/Heartbeat" => {
                    #[allow(non_camel_case_types)]
                    struct HeartbeatSvc<T: Signaling>(pub Arc<T>);
                    impl<
                        T: Signaling,
                    > tonic::server::UnaryService<
                        super::super::message::HeartbeatRequest,
                    > for HeartbeatSvc<T> {
                        type Response = super::super::message::HeartbeatResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::message::HeartbeatRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).heartbeat(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = HeartbeatSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/service.Signaling/Visit" => {
                    #[allow(non_camel_case_types)]
                    struct VisitSvc<T: Signaling>(pub Arc<T>);
                    impl<
                        T: Signaling,
                    > tonic::server::UnaryService<super::super::message::VisitRequest>
                    for VisitSvc<T> {
                        type Response = super::super::message::VisitResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::message::VisitRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).visit(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = VisitSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/service.Signaling/VisitReply" => {
                    #[allow(non_camel_case_types)]
                    struct VisitReplySvc<T: Signaling>(pub Arc<T>);
                    impl<
                        T: Signaling,
                    > tonic::server::UnaryService<
                        super::super::message::VisitReplyRequest,
                    > for VisitReplySvc<T> {
                        type Response = super::super::message::VisitReplyResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::message::VisitReplyRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).visit_reply(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = VisitReplySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/service.Signaling/KeyExchange" => {
                    #[allow(non_camel_case_types)]
                    struct KeyExchangeSvc<T: Signaling>(pub Arc<T>);
                    impl<
                        T: Signaling,
                    > tonic::server::UnaryService<
                        super::super::message::KeyExchangeRequest,
                    > for KeyExchangeSvc<T> {
                        type Response = super::super::message::KeyExchangeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::message::KeyExchangeRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).key_exchange(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = KeyExchangeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/service.Signaling/KeyExchangeReply" => {
                    #[allow(non_camel_case_types)]
                    struct KeyExchangeReplySvc<T: Signaling>(pub Arc<T>);
                    impl<
                        T: Signaling,
                    > tonic::server::UnaryService<
                        super::super::message::KeyExchangeReplyRequest,
                    > for KeyExchangeReplySvc<T> {
                        type Response = super::super::message::KeyExchangeReplyResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::message::KeyExchangeReplyRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).key_exchange_reply(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = KeyExchangeReplySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/service.Signaling/Subscribe" => {
                    #[allow(non_camel_case_types)]
                    struct SubscribeSvc<T: Signaling>(pub Arc<T>);
                    impl<
                        T: Signaling,
                    > tonic::server::ServerStreamingService<
                        super::super::message::SubscribeRequest,
                    > for SubscribeSvc<T> {
                        type Response = super::super::message::PublishMessage;
                        type ResponseStream = T::SubscribeStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::message::SubscribeRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).subscribe(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SubscribeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: Signaling> Clone for SignalingServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: Signaling> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Signaling> tonic::server::NamedService for SignalingServer<T> {
        const NAME: &'static str = "service.Signaling";
    }
}
