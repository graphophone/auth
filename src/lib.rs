use anyhow::Result;

pub mod config;

pub fn run(conf: config::Config) -> Result<()> {
    println!("config: {:?}", conf);
    Ok(())
}