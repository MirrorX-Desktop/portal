use super::{entities, DB_POOL};
use rusqlite::{params, OptionalExtension};

pub async fn query_device_by_id(device_id: i64) -> anyhow::Result<Option<entities::Device>> {
    DB_POOL
        .get()
        .map_err(|err| anyhow::anyhow!(err))?
        .query_row(
            r#"SELECT * FROM devices WHERE id = ? LIMIT 1"#,
            [device_id],
            |row| {
                let id = row.get(0)?;
                let finger_print = row.get(1)?;
                let expire = row.get(2)?;

                Ok(entities::Device {
                    id,
                    finger_print,
                    expire,
                })
            },
        )
        .optional()
        .map_err(|err| anyhow::anyhow!(err))
}

pub async fn query_device_by_finger_print(
    finger_print: &str,
) -> anyhow::Result<Option<entities::Device>> {
    DB_POOL
        .get()
        .map_err(|err| anyhow::anyhow!(err))?
        .query_row(
            r#"SELECT * FROM devices WHERE finger_print = ? LIMIT 1"#,
            [finger_print],
            |row| {
                let id = row.get(0)?;
                let finger_print = row.get(1)?;
                let expire = row.get(2)?;

                Ok(entities::Device {
                    id,
                    finger_print,
                    expire,
                })
            },
        )
        .optional()
        .map_err(|err| anyhow::anyhow!(err))
}

pub async fn query_device_non_available_ids(
    ids: &[i64],
    timestamp: i64,
) -> anyhow::Result<Vec<i64>> {
    let ids_param = ids
        .iter()
        .map(|&id| id.to_string())
        .collect::<Vec<String>>()
        .join(", ");

    let sql = format!(
        "SELECT id FROM devices WHERE expire > ? AND id IN ({})",
        ids_param
    );

    let conn = DB_POOL.get().map_err(|err| anyhow::anyhow!(err))?;
    let mut stmt = conn.prepare(&sql).map_err(|err| anyhow::anyhow!(err))?;
    let result_set = stmt
        .query_map([timestamp], |row| row.get(0))
        .map_err(|err| anyhow::anyhow!(err))?;

    let mut non_available_ids = Vec::new();
    for id in result_set {
        non_available_ids.push(id?);
    }

    Ok(non_available_ids)
}

pub async fn insert_device(
    device_id: i64,
    device_finger_print: &str,
    expire: i64,
) -> anyhow::Result<()> {
    let affected_rows = DB_POOL
        .get()
        .map_err(|err| anyhow::anyhow!(err))?
        .execute(
            r#"
INSERT INTO devices(id, finger_print, expire)
VALUES (?, ?, ?)
ON CONFLICT (id) DO UPDATE SET finger_print = excluded.finger_print,
                               expire       = excluded.expire
WHERE excluded.expire > devices.expire
            "#,
            params![device_id, device_finger_print, expire],
        )
        .map_err(|err| anyhow::anyhow!(err))?;

    if affected_rows != 1 {
        anyhow::bail!("update_device_expire: rows affected is zero")
    } else {
        Ok(())
    }
}

pub async fn update_device_expire(device_id: i64, expire: i64) -> anyhow::Result<()> {
    let affected_rows = DB_POOL
        .get()
        .map_err(|err| anyhow::anyhow!(err))?
        .execute(
            r"UPDATE devices SET expire = ? WHERE id = ?",
            params![expire, device_id],
        )
        .map_err(|err| anyhow::anyhow!(err))?;

    if affected_rows != 1 {
        anyhow::bail!("update_device_expire: rows affected is zero")
    } else {
        Ok(())
    }
}
