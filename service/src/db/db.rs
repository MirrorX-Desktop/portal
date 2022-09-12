use super::entities;
use anyhow::bail;
use futures::TryStreamExt;
use once_cell::sync::OnceCell;
use sqlx::{Any, Execute, Executor, Pool, QueryBuilder, Row, Sqlite};
use std::time::Duration;

static DB_POOL: OnceCell<Pool<Any>> = OnceCell::new();

pub async fn init(url: &str) -> anyhow::Result<()> {
    let logic_processors = num_cpus::get();

    let pool = sqlx::any::AnyPoolOptions::new()
        .max_connections(logic_processors as u32 + 1)
        .max_lifetime(Duration::from_secs(30 * 60))
        .connect(url)
        .await?;

    DB_POOL
        .set(pool)
        .map_err(|_| anyhow::anyhow!("set DB_POOL cell with db pool failed"))
}

pub async fn ensure_schema() -> anyhow::Result<()> {
    if let Some(pool) = DB_POOL.get() {
        let _ = sqlx::query(
            r"
CREATE TABLE IF NOT EXISTS devices (
  id UNSIGNED BIGINT PRIMARY KEY NOT NULL, 
  finger_print char(128) NOT NULL, 
  expire BIGINT NOT NULL
)
    ",
        )
        .execute(pool)
        .await
        .map_err(|err| anyhow::anyhow!(err))?;

        Ok(())
    } else {
        bail!("db pool not initialized")
    }
}

pub async fn query_device_by_id(device_id: u64) -> anyhow::Result<Option<entities::Device>> {
    if let Some(pool) = DB_POOL.get() {
        sqlx::query_as::<_, entities::Device>(r#"SELECT * FROM devices WHERE id = ?"#)
            .bind(device_id)
            .fetch_optional(pool)
            .await
            .map_err(|err| anyhow::anyhow!(err))
    } else {
        bail!("db pool not initialized")
    }
}

pub async fn query_device_available_ids(ids: &[u64], timestamp: i64) -> anyhow::Result<Vec<u64>> {
    let mut query_builder: QueryBuilder<Sqlite> =
        QueryBuilder::new("SELECT id FROM devices WHERE expire <= ? AND id IN (");

    let mut separated = query_builder.separated(", ");
    for id in ids {
        separated.push_bind(id);
    }

    separated.push_unseparated(")");

    let mut query = query_builder.build();
    let sql = query.sql();

    if let Some(pool) = DB_POOL.get() {
        sqlx::query_as::<_, u64>(sql)
            .bind(timestamp)
            .fetch_all(pool)
            .await
            .map_err(|err| anyhow::anyhow!(err))
    } else {
        bail!("db pool not initialized")
    }
}

pub async fn insert_device(
    device_id: u64,
    device_finger_print: &str,
    expire: i64,
) -> anyhow::Result<()> {
    let res = DB_POOL
        .get()
        .ok_or(anyhow::anyhow!("db pool not initialized"))?
        .execute(
            sqlx::query(
                r#"
INSERT INTO devices(id, finger_print, expire)
VALUES (?, ?, ?)
ON CONFLICT (id) DO UPDATE SET finger_print = excluded.finger_print,
                               expire       = excluded.expire
WHERE excluded.expire > devices.expire
            "#,
            )
            .bind(device_id)
            .bind(device_finger_print)
            .bind(expire),
        )
        .await
        .map_err(|err| anyhow::anyhow!(err))?;

    if res.rows_affected() != 1 {
        Err(anyhow::anyhow!("insert_device: rows affected is zero"))
    } else {
        Ok(())
    }
}

pub async fn update_device_expire(device_id: u64, expire: i64) -> anyhow::Result<()> {
    let res = DB_POOL
        .get()
        .ok_or(anyhow::anyhow!("db pool not initialized"))?
        .execute(
            sqlx::query(r"UPDATE devices SET expire = ? WHERE id = ?")
                .bind(expire)
                .bind(device_id),
        )
        .await
        .map_err(|err| anyhow::anyhow!(err))?;

    if res.rows_affected() != 1 {
        Err(anyhow::anyhow!(
            "update_device_expire: rows affected is zero"
        ))
    } else {
        Ok(())
    }
}
