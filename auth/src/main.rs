use dotenvy::dotenv;

use protos::authentication::{
    auth_server::AuthServer, FILE_DESCRIPTOR_SET as AUTH_FILE_DESCRIPTOR_SET,
};

use protos::{client::client_server::ClientServer, message::messaging_server::MessagingServer};
use services::db::get_pool;
use services::{
    authentication::authentication::Service as AuthService,
    client::client::Service as ClientService, db::connect_db,
    message::message::Service as MessageService,
};
use tonic::transport::Server;

mod models;
mod schema;
mod services;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let pool = get_pool();
    let _conn = connect_db();
    let address = "[::1]:50051".parse()?;

    let authentication_service: AuthService = AuthService::new(pool.clone());
    let client_service: ClientService = ClientService::new(pool.clone());
    let message_service: MessageService = MessageService::default();
    let service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(AUTH_FILE_DESCRIPTOR_SET)
        .build()?;

    Server::builder()
        .add_service(service)
        .add_service(MessagingServer::new(message_service))
        .add_service(ClientServer::new(client_service))
        .add_service(AuthServer::new(authentication_service))
        .serve(address)
        .await?;

    Ok(())
}
