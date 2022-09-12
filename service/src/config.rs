use once_cell::sync::Lazy;
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

pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    let content = std::fs::read_to_string("config.toml").unwrap();
    toml::from_str(&content).unwrap()
});
