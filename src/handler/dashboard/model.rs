use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct StatDetails<'a> {
    pub domain: &'a str,
    pub allocated: u64,
    pub bytes_transferred: u64,
    pub client_snapshot: Vec<StreamingClientStat>,
}

#[derive(Serialize, Deserialize)]
pub struct EndPointStatResponse {
    pub bytes_transferred: u64,
    pub client_snapshot: Vec<StreamingClientStat>,
}

#[derive(Serialize, Deserialize)]
pub struct StreamingClientStat {
    pub active_device_id: i64,
    pub active_addr: String,
    pub passive_device_id: i64,
    pub passive_addr: String,
    pub timestamp: i64,
}
