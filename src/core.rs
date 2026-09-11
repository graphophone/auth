use anyhow::Result;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{config::{Config, JwtConfig, RefreshTokenConfig}, redis_cache::{RedisCache, token_cacher::TokenCacher}};

pub struct TokenManager {
    redis_cache: RedisCache,
    jwt_config: JwtConfig,
    refresh_token_config: RefreshTokenConfig,
}

pub struct Tokens {
    access_token: String,
    refresh_token: String,
}

#[derive(Serialize, Deserialize)]
struct Claims {
   sub: i64,
}

impl TokenManager {
    async fn build(conf: Config) -> Result<Self> {
        let redis_cache = RedisCache::build(&conf.redis, 0)?;
        redis_cache.ping().await?;
        
        Ok(TokenManager {
            redis_cache,
            jwt_config: conf.jwt,
            refresh_token_config: conf.refresh_token,
        })
    }

    fn generate_jwt(&self, user_id: i64) -> Result<String> {
        let claims = Claims { sub: user_id };
        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_config.secret.as_ref())
        )
            .map_err(anyhow::Error::from)
    }

    fn extract_jwt_claims(&self, jwt: &str) -> Result<Claims> {
        let token_data = decode::<Claims>(
            jwt,
            &DecodingKey::from_secret(self.jwt_config.secret.as_ref()),
            &Validation::new(Algorithm::HS256)
        )
            .map_err(anyhow::Error::from)?;
        Ok(token_data.claims)
    }

    fn generate_refresh_token(&self) -> String {
        Uuid::new_v4().to_string()
    }

    async fn handle_login(&self, user_id: i64) -> Result<Tokens> {
        let access_token = self.generate_jwt(user_id)?;
        let refresh_token = self.generate_refresh_token();
        self.redis_cache.set_token(
            &refresh_token,
            user_id,
            self.refresh_token_config.expiration_time.as_secs()
        ).await?;
        Ok(Tokens { access_token, refresh_token })
    }

    async fn handle_logout(&self, refresh_token: String) -> Result<()> {
        self.redis_cache.remove_token(&refresh_token).await
    }

    async fn handle_refresh(&self, refresh_token: String) -> Result<Tokens> {
        let user_id = self.redis_cache.get_user_id(&refresh_token).await?;
        let user_id = match user_id {
            Some(v) => v,
            None => return Err(anyhow::Error::msg("refresh_token not found")),
        };
        self.redis_cache.remove_token(&refresh_token).await?;
        self.handle_login(user_id).await
    }

    async fn get_user_id(&self, access_token: String) -> Result<i64> {
        let claims = self.extract_jwt_claims(&access_token)?;
        Ok(claims.sub)
    }
}