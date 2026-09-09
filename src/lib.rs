use anyhow::Result;

use crate::redis_cache::RedisCache;

pub mod config;
mod redis_cache;

pub fn run(conf: config::Config) -> Result<()> {
    println!("config: {:?}", conf);

    let cache = RedisCache::build(&conf.redis)?;
    cache.ping()?;

    Ok(())
}