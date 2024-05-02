use redis::{Client, Commands, Connection, RedisError};
use std::error::Error;

use crate::models::user::IUserCache;

pub struct RedisCache {
    pub redis_client: Client,
}

impl RedisCache {
    pub fn get_conn(&self) -> Result<Connection, RedisError> {
        self.redis_client.get_connection()
    }
}

#[tonic::async_trait]
impl IUserCache for RedisCache {
    async fn set_exp(
        &self,
        key: &String,
        value: &String,
        seconds: usize,
    ) -> Result<(), Box<dyn Error>> {
        let mut connection = self.get_conn()?;
        let _: () = connection.set_ex(key, value, seconds)?;
        Ok(())
    }

    async fn del_val_for_key(&self, key: &String) -> Result<u64, Box<dyn Error>> {
        let mut connection = self.get_conn()?;
        let result = connection.del(key)?;
        Ok(result)
    }

    async fn get_val(&self, key: &String) -> Result<Option<String>, Box<dyn Error>> {
        let mut connection = self.get_conn()?;
        let result = connection.get(key)?;
        match result {
            Some(value) => Ok(Some(value)),
            None => Ok(None),
        }
    }
}
