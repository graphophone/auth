use anyhow::Result;
use r2d2::Pool;
use redis::Client;

use crate::config;
pub struct RedisCache {
    pool: Pool<Client>,
}

impl RedisCache {
    pub fn build(conf: &config::RedisConfig) -> Result<Self> {
        let client = Client::open(
            format!(
                "redis://{}:{}@{}:{}",
                conf.user,
                conf.password,
                conf.host,
                conf.port,
            ),
        )?;
        let pool = Pool::builder()
            .build(client)?;

        Ok(RedisCache { pool })
    }

    pub fn ping(&self) -> Result<()> {
        let mut con = self.pool.get()?;
        redis::cmd("PING")
            .exec(&mut con)?;
        Ok(())
    }
}