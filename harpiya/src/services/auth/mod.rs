mod auth;

use harpiya_domain::auth_service;
use sea_orm::DatabaseConnection;
use tonic::{Request, Status, Response};

use crate::{ServiceResult, utils::{verify_password, Jwt}, database};

pub struct AuthService {
    db: DatabaseConnection
}

impl AuthService {
    pub async fn new(db_uri: &str) -> Self {
        Self {
            db: database::new_client(db_uri).await.unwrap()
        }
    }
}

#[tonic::async_trait]
impl auth_service::auth_service_server::AuthService for AuthService {
    async fn sign_in(
        &self,
        request: Request<auth_service::SignInRequest>
    ) -> ServiceResult<auth_service::AuthorizationResponse> {
        let request = request.into_inner();
        let user = match request.username.is_empty() {
            true => auth::Business::new(&self.db).get_user_by_email(&request.email).await,
            false => auth::Business::new(&self.db).get_user_by_username(&request.username).await
        }.map_err(|err| Status::not_found(err))?;

        if !verify_password(&request.password, &user.password).map_err(|_| Status::internal("cannot verify password (hasher error)"))? {
            return Err(Status::unauthenticated("password is incorrect"));
        }

        let token = auth::Business::new(&self.db).create_new_access_token(&user.name, &user.username)
            .await
            .map_err(|msg| Status::internal(msg))?;
        
        Ok(Response::new(
            auth_service::AuthorizationResponse { 
                username: user.username, 
                token: token, 
                refresh_token: "".to_owned() 
            }
        ))
    }
}