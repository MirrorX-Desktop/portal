use crate::{handler, subscriber::serve_subscriber_server, SIGNALING_PORT, SUBSCRIBE_PORT};
use axum::{
    routing::{get, post},
    Router,
};
use std::net::{Ipv4Addr, SocketAddr};

pub async fn launch_api_server() {
    let http_listen_addr: SocketAddr = (Ipv4Addr::UNSPECIFIED, *SIGNALING_PORT).into();
    let subscribe_listen_addr: SocketAddr = (Ipv4Addr::UNSPECIFIED, *SUBSCRIBE_PORT).into();

    let api = Router::new()
        .route("/api/identity", get(handler::api::identity::identity))
        .route("/api/domain/register", post(handler::api::domain::register))
        .route("/api/visit", post(handler::api::visit::visit));

    tracing::info!("http api server listening on {}", http_listen_addr);
    let http_future = axum::Server::bind(&http_listen_addr).serve(api.into_make_service());

    tracing::info!("api subscribers listening on {}", subscribe_listen_addr);
    let subscribers_future = serve_subscriber_server(subscribe_listen_addr);

    tokio::select! {
        _ = http_future => {},
        _ = subscribers_future => {},
        _ = tokio::signal::ctrl_c() => {},
    }

    tracing::info!("http api server exit");
    std::process::exit(1);
}
