use std::sync::{Arc, Mutex};

use crate::models::user::User;
use diesel::PgConnection;
use tonic::{Request, Response, Status};

use protos::auth::{auth_server::Auth, AccessToken, LoginRequest, RegisterRequest};

mod protos;

pub struct Service {
    database: Arc<Mutex<PgConnection>>,
}

impl Service {
    pub fn new(database: PgConnection) -> Self {
        Self {
            database: Arc::new(Mutex::new(database)),
        }
    }

    fn generate_token(user: User) -> AccessToken {
        unimplemented!();
    }
}

#[tonic::async_trait]
impl Auth for Service {
    async fn login(&self, request: Request<LoginRequest>) -> Result<Response<AccessToken>, Status> {
        unimplemented!();
    }

    async fn register(
        &self,
        request: Request<RegisterRequest>,
    ) -> Result<Response<AccessToken>, Status> {
        unimplemented!();
    }
}
