use dotenvy::dotenv;
use portal::{
    db,
    server::{api::launch_api_server, dashboard::launch_dashboard_server},
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let env_path = dotenv()?;

    tracing_subscriber::registry()
        .with(EnvFilter::from_env("LOG"))
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("load .env from {:?}", env_path);

    db::ensure_schema().await?;

    tokio::spawn(launch_api_server());
    tokio::spawn(launch_dashboard_server());

    let _ = tokio::signal::ctrl_c().await;

    Ok(())
}
