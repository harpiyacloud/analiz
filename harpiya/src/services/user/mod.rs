pub mod user;

use sea_orm::DatabaseConnection;
use tonic::{Request, Response};

use harpiya_domain::user_service;

use crate::{database, ServiceResult};

pub struct UserService {
    db: DatabaseConnection
}

impl UserService {
    pub async fn new(db_uri: &str) -> Self {
        Self {
            db: database::new_client(db_uri).await.unwrap()
        }
    }
}

#[tonic::async_trait]
impl user_service::user_service_server::UserService for UserService {
    async fn list_user(
        &self,
        request: Request<user_service::ListUserParams>
    ) -> ServiceResult<user_service::ListUserReply> {
        let params = request.into_inner();
        let bis = user::Business::new(&self.db, params.header.to_owned().unwrap());
        let reply = bis.list(params.filter, params.sorts, params.pager).await;
        Ok(Response::new(reply)) 
    }

    async fn operate_user(
        &self,
        request: Request<user_service::OperateUserParams>,
    ) -> ServiceResult<user_service::OperateUserReply> {
        let params = request.into_inner();
        let bis = user::Business::new(&self.db, params.header.to_owned().unwrap());
        let reply = bis.operate(params.method(), params.data).await;
        Ok(Response::new(reply))
    }

    async fn batch_operate_user(
        &self,
        request: Request<user_service::BatchOperateUserParams>,
    ) -> ServiceResult<user_service::BatchOperateUserReply> {
        let params = request.into_inner();
        let bis = user::Business::new(&self.db, params.header.to_owned().unwrap());
        let reply = bis.batch_operate(params.method(), params.data).await;
        Ok(Response::new(reply))
    }
}