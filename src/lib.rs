pub mod component;
pub mod db;
pub mod handler;
pub mod message;
pub mod server;
pub mod subscriber;

use once_cell::sync::Lazy;

pub(crate) static DOMAIN: Lazy<String> = Lazy::new(|| std::env::var("DOMAIN").unwrap());

pub(crate) static SIGNALING_PORT: Lazy<u16> =
    Lazy::new(|| std::env::var("SIGNALING_PORT").unwrap().parse().unwrap());

pub(crate) static SUBSCRIBE_PORT: Lazy<u16> =
    Lazy::new(|| std::env::var("SUBSCRIBE_PORT").unwrap().parse().unwrap());

pub(crate) static DASHBOARD_PORT: Lazy<u16> =
    Lazy::new(|| std::env::var("DASHBOARD_PORT").unwrap().parse().unwrap());

pub(crate) static ENDPOINT_API_BASE_ADDRS: Lazy<Vec<String>> = Lazy::new(|| {
    std::env::var("ENDPOINT_API_BASE_ADDRS")
        .unwrap()
        .split(',')
        .map(|v| v.trim())
        .map(String::from)
        .collect()
});
