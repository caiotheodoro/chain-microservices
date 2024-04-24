use dotenvy::dotenv;

use protos::authentication::{
    auth_server::AuthServer, FILE_DESCRIPTOR_SET as AUTH_FILE_DESCRIPTOR_SET,
};

use protos::message::messaging_server::MessagingServer as MessageServer;

use services::{
    authentication::authentication::Service as AuthService, db::connect_db,
    message::message::Service as MessageService,
};
use tonic::transport::Server;

mod models;
mod schema;
mod services;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let db = connect_db();
    let address = "[::1]:50051".parse()?;

    let authentication_service: AuthService = AuthService::new(db);
    let message_service: MessageService = MessageService::default();
    let service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(AUTH_FILE_DESCRIPTOR_SET)
        .build()?;

    Server::builder()
        .add_service(service)
        .add_service(MessageServer::new(message_service))
        .add_service(AuthServer::new(authentication_service))
        .serve(address)
        .await?;

    Ok(())
}
