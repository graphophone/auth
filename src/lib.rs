use anyhow::Result;

pub mod config;
mod redis_cache;
mod core;

pub async fn run(conf: config::Config) -> Result<()> {
    println!("config: {:?}", conf);

    Ok(())
}