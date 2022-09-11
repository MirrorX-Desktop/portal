mod component;
mod db;
mod network;

use anyhow::bail;
use log::{error, info};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    component::logger::init();
    component::config::init()?;

    let cfg = match component::config::CONFIG.get() {
        Some(v) => v,
        None => bail!("read config instance failed"),
    };

    db::init(cfg.db.uri.as_ref()).await?;
    db::ensuere_schema().await?;

    let listener = TcpListener::bind(cfg.listen_addr.clone()).await?;

    tokio::spawn(async move {
        if let Ok(addr) = listener.local_addr() {
            info!("server listen on: {}", addr);
        }

        loop {
            let (stream, _) = match listener.accept().await {
                Ok(endpoint) => endpoint,
                Err(err) => {
                    error!("listener accept: {:?}", err);
                    break;
                }
            };

            tokio::spawn(async move {
                if let Err(err) = network::client::Client::serve(stream).await {
                    error!("{}", err)
                }
            });
        }
    });

    tokio::signal::ctrl_c()
        .await
        .map_err(|err| anyhow::anyhow!("failed to listen for event ({})", err))
}
