use crate::services::authentication::messages::MessageService;

extern crate redis;

pub fn connect_redis() -> redis::Client {
    let url = std::env::var("REDIS_URL").expect(MessageService::ERROR_REDIS_SETUP);

    let client = redis::Client::open(url);
    let client = match client {
        Ok(client) => client,
        Err(e) => {
            panic!("Error: {}", e);
        }
    };

    client
}
