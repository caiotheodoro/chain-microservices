use dotenvy::dotenv;
use services::db::connect_db;

mod models;
mod schema;
mod services;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let mut db = connect_db();
    Ok(())
}
