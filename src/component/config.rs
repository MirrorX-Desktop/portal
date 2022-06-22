use once_cell::sync::OnceCell;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub listen_addr: String,
    pub db: DB,
}

#[derive(Deserialize)]
pub struct DB {
    pub uri: String,
}

pub static CONFIG: OnceCell<Config> = OnceCell::new();

pub fn init() -> anyhow::Result<()> {
    let content = std::fs::read_to_string("config.toml")?;
    let config: Config = toml::from_str(&content).map_err(|err| anyhow::anyhow!(err))?;
    CONFIG
        .set(config)
        .map_err(|_| anyhow::anyhow!("set config instance failed"))
}
