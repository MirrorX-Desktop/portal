use std::{ops::Add, time::Duration};

pub struct RedisStore {
    redis_client: redis::Client,
}

impl RedisStore {
    pub fn new() -> anyhow::Result<Self> {
        let cli = redis::Client::open("redis://127.0.0.1")?;
        Ok(RedisStore { redis_client: cli })
    }

    pub fn refresh_device_id(&self, device_id: &str) -> anyhow::Result<Option<u32>> {
        let mut con = self
            .redis_client
            .get_connection_with_timeout(Duration::from_secs(1))?;

        let key = format!("device_id:{}", device_id);
        let success = redis::cmd("SET")
            .arg(key)
            .arg("")
            .arg("EX")
            .arg::<u32>(90 * 24 * 60 * 60)
            .arg("XX")
            .query::<bool>(&mut con)?;

        if success {
            Ok(Some(
                chrono::Utc::now()
                    .add(chrono::Duration::days(90))
                    .timestamp() as u32,
            ))
        } else {
            Ok(None)
        }
    }

    pub fn pub_new_device_id(&self, device_id: &str) -> anyhow::Result<Option<u32>> {
        let mut con = self
            .redis_client
            .get_connection_with_timeout(Duration::from_secs(1))?;

        let key = format!("device_id:{}", device_id);
        let success = redis::cmd("SET")
            .arg(key)
            .arg("")
            .arg("EX")
            .arg::<u32>(90 * 24 * 60 * 60)
            .arg("NX")
            .query::<bool>(&mut con)?;

        if success {
            Ok(Some(
                chrono::Utc::now()
                    .add(chrono::Duration::days(90))
                    .timestamp() as u32,
            ))
        } else {
            Ok(None)
        }
    }
}
