use diesel::{
    r2d2::{ConnectionManager, Pool},
    Connection, PgConnection,
};
use std::env;

pub fn connect_db() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL missing!");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Not abled to connect to {}", database_url))
}

pub fn get_pool() -> Pool<ConnectionManager<PgConnection>> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL missing!");
    let manager = ConnectionManager::<PgConnection>::new(&database_url);
    let db_pool = match Pool::builder().build(manager) {
        Ok(pool) => pool,
        Err(_e) => {
            std::process::exit(1);
        }
    };

    db_pool
}
