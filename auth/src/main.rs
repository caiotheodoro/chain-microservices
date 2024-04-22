use dotenvy::dotenv;

use protos::authentication::{
    auth_server::AuthServer, FILE_DESCRIPTOR_SET as AUTH_FILE_DESCRIPTOR_SET,
};
use services::{authentication::Service, db::connect_db};
use tonic::transport::Server;

mod models;
mod schema;
mod services;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let db = connect_db();
    let address = "[::1]:50051".parse()?;

    let authentication_service: Service = Service::new(db);
    let service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(AUTH_FILE_DESCRIPTOR_SET)
        .build()?;

    Server::builder()
        .add_service(service)
        .add_service(AuthServer::new(authentication_service))
        .serve(address)
        .await?;

    Ok(())
}
