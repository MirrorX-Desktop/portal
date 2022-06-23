use super::entities;
use anyhow::bail;
use futures::TryStreamExt;
use once_cell::sync::OnceCell;
use sqlx::{Any, Executor, Pool, Row};
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

pub async fn ensuere_schema() -> anyhow::Result<()> {
    if let Some(pool) = DB_POOL.get() {
        let _ = sqlx::query(
            r"
CREATE TABLE IF NOT EXISTS devices (
  id BIGINT PRIMARY KEY NOT NULL, 
  device_hash char(128) NOT NULL, 
  expire INT NOT NULL
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

pub async fn query_device_by_id(device_id: i64) -> anyhow::Result<Option<entities::Device>> {
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

pub async fn query_free_ids(device_id_range_min: i64) -> anyhow::Result<Vec<i64>> {
    let ids: Vec<i64> = (device_id_range_min..device_id_range_min + 20).collect();

    let mut q = sqlx::query(
        r#"SELECT id FROM devices WHERE id IN (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
    );

    for i in &ids {
        q = q.bind(*i);
    }

    if let Some(pool) = DB_POOL.get() {
        let mut rows = q.fetch(pool);
        let mut exist_ids = Vec::new();

        while let Some(row) = rows.try_next().await? {
            let id: i64 = row.try_get("id")?;
            exist_ids.push(id);
        }

        let free_ids = ids
            .iter()
            .filter(|v| !exist_ids.contains(*v))
            .map(|v| *v)
            .collect::<Vec<_>>();

        Ok(free_ids)
    } else {
        bail!("db pool not initialized")
    }
}

pub async fn insert_device(entity: &entities::Device) -> anyhow::Result<()> {
    let res = DB_POOL
        .get()
        .ok_or(anyhow::anyhow!("db pool not initialized"))?
        .execute(
            sqlx::query(r"INSERT INTO devices(id, device_hash, expire) VALUES(?, ?, ?)")
                .bind(entity.id)
                .bind(entity.device_hash.clone())
                .bind(entity.expire),
        )
        .await
        .map_err(|err| anyhow::anyhow!(err))?;

    if res.rows_affected() != 1 {
        Err(anyhow::anyhow!("insert_device: rows affected is zero"))
    } else {
        Ok(())
    }
}

pub async fn update_device_expire(device_id: i64, expire: i32) -> anyhow::Result<()> {
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
