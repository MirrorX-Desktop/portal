use std::ops::Add;

use log::info;

use crate::utility;

pub struct RedisStore {
    redis_client: redis::Client,
}

impl RedisStore {
    pub fn new() -> anyhow::Result<Self> {
        let cli = redis::Client::open("redis://127.0.0.1")?;
        Ok(RedisStore { redis_client: cli })
    }

    pub async fn refresh_device_id(&self, device_id: &str) -> anyhow::Result<Option<i64>> {
        let device_id_splited: Vec<&str> = device_id.split('-').collect();
        let bucket_id: u32 = device_id_splited[0].parse()?;
        let id: u32 =
            device_id_splited[1].parse::<u32>()? * 10000 + device_id_splited[2].parse::<u32>()?;

        let mut con = self.redis_client.get_tokio_connection().await?;

        // select db
        redis::cmd("SELECT")
            .arg((bucket_id % (16 - 1)) + 1)
            .query_async(&mut con)
            .await?;

        // query expire timestamp, if not exist or expired, return None;
        // otherwise renew expire timestamp and return it.
        let expire_ts: Option<i64> = redis::cmd("HGET")
            .arg(format!("device_id_bucket:{}", bucket_id))
            .arg(id)
            .query_async(&mut con)
            .await?;

        info!("{:?}", expire_ts);

        if expire_ts.is_none() {
            return Ok(None);
        }

        if expire_ts.unwrap() < chrono::Utc::now().timestamp() {
            redis::cmd("HDEL")
                .arg(format!("device_id_bucket:{}", bucket_id))
                .arg(id)
                .query_async(&mut con)
                .await?;
            return Ok(None);
        }

        let new_expire_ts = chrono::Utc::now()
            .add(chrono::Duration::days(180))
            .timestamp();

        redis::cmd("HSET")
            .arg(format!("device_id_bucket:{}", bucket_id))
            .arg(id)
            .arg(new_expire_ts)
            .query_async(&mut con)
            .await?;

        Ok(Some(new_expire_ts))
    }

    pub async fn new_device_id(&self) -> anyhow::Result<(String, i64)> {
        let mut con = self.redis_client.get_tokio_connection().await?;

        for _ in 0..10 {
            let (bucket_id, id) = utility::device_id_generator::generate();

            redis::cmd("SELECT")
                .arg((bucket_id % (16 - 1)) + 1)
                .query_async(&mut con)
                .await?;

            for i in 0..10 {
                let exists: bool = redis::cmd("HEXISTS")
                    .arg(format!("device_id_bucket:{}", bucket_id))
                    .arg(id + i)
                    .query_async(&mut con)
                    .await?;
                if !exists {
                    let new_expire_ts = chrono::Utc::now()
                        .add(chrono::Duration::days(180))
                        .timestamp();

                    let success: bool = redis::cmd("HSETNX")
                        .arg(format!("device_id_bucket:{}", bucket_id))
                        .arg(id + i)
                        .arg(new_expire_ts)
                        .query_async(&mut con)
                        .await?;

                    if success {
                        return Ok((
                            format!("{:0>2}-{:0>4}-{}", bucket_id, id / 10000, id % 10000),
                            new_expire_ts,
                        ));
                    }
                }
            }
        }

        return Err(anyhow::anyhow!("too many failures"));
    }
}
