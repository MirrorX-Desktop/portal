#[derive(sqlx::FromRow)]
pub struct Device {
    pub id: u64,
    pub finger_print: String,
    pub expire: i64,
}
