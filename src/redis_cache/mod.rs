use anyhow::Result;
use deadpool_redis::{Pool, Runtime};

use crate::config;
pub struct RedisCache {
    pool: Pool,
}

impl RedisCache {
    pub fn build(conf: &config::RedisConfig) -> Result<Self> {
        let url = format!("redis://{}:{}@{}:{}",
            conf.user,
            conf.password,
            conf.host,
            conf.port,
        );

        let cfg = deadpool_redis::Config::from_url(url);
        let pool = cfg.create_pool(Some(Runtime::Tokio1))?;

        Ok(RedisCache { pool })
    }

    pub async fn ping(&self) -> Result<()> {
        let mut con = self.pool.get().await?;
        deadpool_redis::redis::cmd("PING")
            .exec_async(&mut con)
            .await?;
        Ok(())
    }
}