mod db;
mod network;
mod utility;

use env_logger::{Builder, Target};
use log::{error, LevelFilter};
use std::io::Write;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_logger();
    db::init("".to_string()).await?;
    db::ensuere_schema().await?;

    let (exit_tx, exit_rx) = std::sync::mpsc::channel();
    ctrlc::set_handler(move || {
        let _ = exit_tx.send(());
    })
    .map_err(|err| anyhow::anyhow!(err))?;

    // let state = Arc::new(state::State::new()?);

    let listener = TcpListener::bind("127.0.0.1:2345").await?;

    tokio::spawn(async move {
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

    exit_rx.recv().map_err(|err| anyhow::anyhow!(err))
}

fn init_logger() {
    Builder::new()
        .filter_level(LevelFilter::Info)
        .format(|buf, record| {
            writeln!(
                buf,
                "[{}] [{}({}#{})] {} {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S.%3f"),
                record.module_path().unwrap_or(""),
                record.file().unwrap_or(""),
                record.line().unwrap_or(0),
                record.level(),
                record.args(),
            )
        })
        .target(Target::Stdout)
        .init();
}
