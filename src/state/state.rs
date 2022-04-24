use super::redis::RedisStore;

pub struct State {
    pub redis: RedisStore,
}

impl State {
    pub fn new() -> anyhow::Result<Self> {
        let redis_store = RedisStore::new()?;

        Ok(State { redis: redis_store })
    }
}
