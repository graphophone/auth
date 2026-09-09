use std::{fs, time::Duration};

use anyhow::Result;
use serde::{Deserialize, Deserializer};

#[derive(Deserialize, Debug)]
pub struct Config {
    pub redis: RedisConfig,
    pub jwt: JwtConfig,
    pub refresh_token: RefreshTokenConfig,
}

impl Config {
    pub fn build(filepath: &str) -> Result<Self> {
        let content = fs::read_to_string(filepath)?;
        yaml_serde::from_str(&content)
            .map_err(anyhow::Error::from)
    }
}

#[derive(Deserialize, Debug)]
pub struct RedisConfig {
    pub host: String,
    pub port: i16,
    pub user: String,
    pub password: String,
}

#[derive(Deserialize, Debug)]
pub struct JwtConfig {
    pub secret: String,
    #[serde(deserialize_with = "parse_duration")]
    pub expiration_time: Duration,
}


#[derive(Deserialize, Debug)]
pub struct RefreshTokenConfig {
    #[serde(deserialize_with = "parse_duration")]
    pub expiration_time: Duration,
}

fn parse_duration<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
    D: Deserializer<'de>
{
    let token = String::deserialize(deserializer)?;
    let num_part = token.trim_end_matches(|c: char| !c.is_numeric());
    let unit_part = &token[num_part.len()..];
    let num: u64 = num_part
        .parse()
        .map_err(|_| serde::de::Error::custom("invalid number"))?;
    let duration = match unit_part {
        "s" => Duration::from_secs(num),
        "min" => Duration::from_mins(num),
        "h" => Duration::from_hours(num),
        _ => return Err(serde::de::Error::custom(format!("invalid unit: {unit_part}"))),
    };
    Ok(duration)
}