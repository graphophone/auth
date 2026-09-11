use anyhow::Result;

pub mod config;
mod redis_cache;
mod core;
mod service;

pub async fn run(conf: config::Config) -> Result<()> {
    println!("config: {:?}", conf);

    Ok(())
}