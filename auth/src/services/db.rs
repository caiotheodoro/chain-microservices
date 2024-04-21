use diesel::{Connection, PgConnection};
use std::env;

pub fn connect_db() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL missing!");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Not abled to connect to {}", database_url))
}
