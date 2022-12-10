pub mod device;
pub mod entities;

use once_cell::sync::Lazy;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;

static DB_POOL: Lazy<Pool<SqliteConnectionManager>> = Lazy::new(|| {
    let manager = SqliteConnectionManager::file("signaling.db");
    r2d2::Pool::new(manager).unwrap()
});

pub async fn ensure_schema() -> anyhow::Result<()> {
    let _ = DB_POOL
        .get()
        .map_err(|err| anyhow::anyhow!(err))?
        .execute(
            r"
CREATE TABLE IF NOT EXISTS devices (
  id BIGINT PRIMARY KEY NOT NULL,
  finger_print char(128) NOT NULL,
  expire BIGINT NOT NULL
)",
            [],
        )
        .map_err(|err| anyhow::anyhow!(err))?;
    Ok(())
}
