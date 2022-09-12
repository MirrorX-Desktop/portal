mod config;
mod db;
mod handlers;

use crate::handlers::SignalingService;
use config::CONFIG;
use signaling_proto::signaling_server::SignalingServer;
use std::time::Duration;
use tonic::{codegen::CompressionEncoding, transport::server::TcpIncoming, transport::Server};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    db::init(&CONFIG.db.uri).await?;
    db::ensure_schema().await?;

    let incoming = TcpIncoming::new(
        CONFIG.listen_addr.parse()?,
        true,
        Some(Duration::from_secs(60 * 10)),
    )
    .map_err(|err| anyhow::anyhow!(err))?;

    let service = SignalingServer::new(SignalingService {})
        .accept_compressed(CompressionEncoding::Gzip)
        .send_compressed(CompressionEncoding::Gzip);

    Server::builder()
        .add_service(service)
        .serve_with_incoming_shutdown(incoming, async {
            let _ = tokio::signal::ctrl_c().await;
        })
        .await?;

    Ok(())
}
