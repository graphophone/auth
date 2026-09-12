use anyhow::Result;
use tonic::transport::Server;

use crate::{core::TokenManager, service::{AuthService, auth::auth_server::AuthServer}};

pub mod config;
mod redis_cache;
mod core;
mod service;

pub async fn run(conf: config::Config) -> Result<()> {
    let addr = "0.0.0.0:8080".parse()?;

    let token_manager = TokenManager::build(conf).await
        .expect("failed to create token manager");
    let auth_service = AuthService::new(token_manager);

    Server::builder()
        .add_service(AuthServer::new(auth_service))
        .serve(addr)
        .await?;

    Ok(())
}