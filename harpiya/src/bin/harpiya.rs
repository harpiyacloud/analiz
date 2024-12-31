use harpiya::{services::{ssh_key, user, auth}, config};
use harpiya_domain::{
    ssh_key_service::ssh_key_service_server::SshKeyServiceServer, 
    user_service::user_service_server::UserServiceServer, 
    auth_service::auth_service_server::AuthServiceServer
};

#[tokio::main]
async fn main()  {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_test_writer()
        .init();

    let config = config::Config::load();

    let ssh_key_srv = ssh_key::SSHKeyService::new(config.db_url.as_str()).await;
    let user_srv = user::UserService::new(config.db_url.as_str()).await;
    let aut_srv = auth::AuthService::new(&config.db_url.as_str()).await;

    tonic::transport::Server::builder()
        .add_service(SshKeyServiceServer::new(ssh_key_srv))
        .add_service(UserServiceServer::new(user_srv))
        .add_service(AuthServiceServer::new(aut_srv))
        .serve(config.srv_addr.parse().unwrap())
        .await
        .unwrap();
}