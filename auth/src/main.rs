use dotenvy::dotenv;
use protos::auth::auth_server::AuthServer;
use services::auth::Service;
use services::db::connect_db;
use tonic::transport::Server;

mod models;
mod protos;
mod schema;
mod services;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let mut db = connect_db();
    let address = "[::1]:50051".parse()?;

    Server::builder()
        .add_service(AuthServer::new(Service::new(database)))
        .serve(address)
        .await?;

    Ok(())
}
