#[derive(sqlx::FromRow)]
pub struct Device {
    pub id: i64,
    pub device_hash: String,
    pub expire: i32, // timestamp from 2022-01-01T00:00:00Z UTC
}
