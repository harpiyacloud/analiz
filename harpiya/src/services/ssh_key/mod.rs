use sea_orm::DatabaseConnection;
use tonic::{Request, Response};

use harpiya_domain::ssh_key_service;

use crate::{database, ServiceResult};

mod ssh_key;

pub struct SSHKeyService {
    db: DatabaseConnection,
}

impl SSHKeyService {
    pub async fn new(db_uri: &str) -> Self {
        Self {
            db: database::new_client(db_uri).await.unwrap()
        }
    }
}

#[tonic::async_trait]
impl ssh_key_service::ssh_key_service_server::SshKeyService for SSHKeyService {
    async fn list_ssh_key(
        &self,
        request: Request<ssh_key_service::ListSshKeyParams>
    ) -> ServiceResult<ssh_key_service::ListSshKeyReply> {
        let params = request.into_inner();
        let bis = ssh_key::Business::new(&self.db, params.header.to_owned().unwrap());
        let reply = bis.list(params.filter, params.sorts, params.pager).await;
        Ok(Response::new(reply))
    }

    async fn operate_ssh_key(
        &self,
        request: Request<ssh_key_service::OperateSshKeyParams>,
    ) -> ServiceResult<ssh_key_service::OperateSshKeyReply> {
        println!("{:#?}", request.metadata());
        let params = request.into_inner();
        let bis = ssh_key::Business::new(&self.db, params.header.to_owned().unwrap());
        let reply = bis.operate(params.method(), params.data).await;
        Ok(Response::new(reply))
    }

    async fn batch_operate_ssh_key(
        &self,
        request: Request<ssh_key_service::BatchOperateSshKeyParams>,
    ) -> ServiceResult<ssh_key_service::BatchOperateSshKeyReply> {
        let params = request.into_inner();
        let bis = ssh_key::Business::new(&self.db, params.header.to_owned().unwrap());
        let reply = bis.batch_operate(params.method(), params.data).await;
        Ok(Response::new(reply))
    }
}