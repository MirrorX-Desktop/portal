mod db;
mod handlers;
mod message;
mod subscriber;

use crate::subscriber::serve_subscriber_server;
use axum::{
    routing::{get, post},
    Router,
};
use dotenvy::dotenv;
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let env_path = dotenv()?;

    tracing_subscriber::registry()
        .with(EnvFilter::from_env("LOG"))
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("load .env from {:?}", env_path);

    let http_listen_addr: SocketAddr = std::env::var("HTTP_LISTEN_ADDR")?.parse()?;
    let subscribe_listen_addr: SocketAddr = std::env::var("SUBSCRIBE_LISTEN_ADDR")?.parse()?;

    db::ensure_schema().await?;

    let app = Router::new()
        .route("/api/identity", get(handlers::identity::identity))
        .route("/api/domain/register", post(handlers::domain::register))
        .route("/api/visit", post(handlers::visit::visit));

    tracing::info!("http listening on {}", http_listen_addr);
    let http_future = axum::Server::bind(&http_listen_addr).serve(app.into_make_service());

    tracing::info!("subscribers listening on {}", subscribe_listen_addr);
    let subscribers_future = serve_subscriber_server(subscribe_listen_addr);

    tokio::select! {
        _ = http_future => {},
        _ = subscribers_future => {},
        _ = tokio::signal::ctrl_c() => {},
    }

    Ok(())
}
