use tonic::{Request, Response, Status};

use crate::{core::TokenManager, service::auth::{AccessToken, Empty, RefreshToken, Tokens, UserId, auth_server::Auth}};

pub mod auth {
    tonic::include_proto!("auth");
}

pub struct AuthService {
    token_manager: TokenManager,
}

impl AuthService {
    pub fn new(token_manager: TokenManager) -> Self {
        AuthService { token_manager }
    }
}

#[tonic::async_trait]
impl Auth for AuthService {
    async fn login(&self, req: Request<UserId>) -> Result<Response<Tokens>, Status> {
        let req = req.into_inner();
        let tokens = self.token_manager.handle_login(req.user_id).await;
        let tokens = match tokens {
            Ok(v) => v,
            Err(_) => return Err(Status::internal("failed to handle login")),
        };
        Ok(Response::new(Tokens {
            access_token: tokens.access_token,
            refresh_token: tokens.refresh_token,
        }))
    }

    async fn logout(&self, req: Request<RefreshToken>) -> Result<Response<Empty>, Status> {
        let req = req.into_inner();
        let res =  self.token_manager.handle_logout(req.refresh_token).await;
        if let Err(_) = res {
            return Err(Status::internal("failed to handle logout"));
        }
        return Ok(Response::new(Empty { }))
    }

    async fn refresh_tokens(&self, req: Request<RefreshToken>) -> Result<Response<Tokens>, Status> {
        let req = req.into_inner();
        let tokens = self.token_manager.handle_refresh(req.refresh_token).await;
        let tokens = match tokens {
            Ok(v) => v,
            Err(_) => return Err(Status::internal("failed to handle refresh")),
        };
        Ok(Response::new(Tokens {
            access_token: tokens.access_token,
            refresh_token: tokens.refresh_token,
        }))
    }

    async fn extract_user_id(&self, req: Request<AccessToken>) -> Result<Response<UserId>, Status> {
        let req = req.into_inner();
        let user_id = self.token_manager.get_user_id(req.access_token).await;
        let user_id = match user_id {
            Ok(v) => v,
            Err(e) => return Err(Status::unauthenticated(e.to_string())),
        };
        Ok(Response::new(UserId { user_id }))
    }
}