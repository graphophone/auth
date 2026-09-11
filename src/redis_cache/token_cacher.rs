use anyhow::Result;
use redis::{AsyncCommands, cmd};

use crate::redis_cache::RedisCache;

pub trait TokenCacher {
    async fn add_token(&self, token: &str, user_id: i64) -> Result<()>;
    async fn remove_token(&self, token: &str) -> Result<()>;
    async fn get_user_id(&self, token: &str) -> Result<Option<i64>>;
}

impl TokenCacher for RedisCache {
    async fn add_token(&self, token: &str, user_id: i64) -> Result<()> {
        let mut con = self.pool.get().await?;
        cmd("SET")
            .arg(token)
            .arg(user_id)
            .query_async::<()>(&mut con)
            .await?;
        Ok(())
    }

    async fn remove_token(&self, token: &str) -> Result<()> {
        let mut con = self.pool.get().await?;
        cmd("DEL")
            .arg(token)
            .query_async::<()>(&mut con)
            .await?;
        Ok(())
    }

    async fn get_user_id(&self, token: &str) -> Result<Option<i64>> {
        let mut con = self.pool.get().await?;
        let user_id: i64 = cmd("GET")
            .arg(token)
            .query_async(&mut con)
            .await?;
        Ok(Some(user_id))
    }
}