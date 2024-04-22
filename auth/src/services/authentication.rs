use std::sync::{Arc, Mutex};

use crate::models::user::{NewUser, User};
use diesel::PgConnection;
use protos::authentication::{auth_server::Auth, AccessToken, LoginRequest, RegisterRequest};
use tonic::{Request, Response, Status};
pub struct Service {
    database: Arc<Mutex<PgConnection>>,
}

impl Service {
    pub fn new(database: PgConnection) -> Self {
        Self {
            database: Arc::new(Mutex::new(database)),
        }
    }
}

#[tonic::async_trait]
impl Auth for Service {
    async fn login(
        &self,
        _request: Request<LoginRequest>,
    ) -> Result<Response<AccessToken>, Status> {
        Ok(Response::new(AccessToken {
            access_token: "token".to_string(),
        }))
    }

    async fn register(
        &self,
        request: Request<RegisterRequest>,
    ) -> Result<Response<AccessToken>, Status> {
        let database = self.database.lock();
        let data = request.into_inner();

        let password = bcrypt::hash(&data.password, 10)
            .map_err(|_| Status::unknown("Error while creating the user"))?;

        let user = NewUser {
            username: &data.username,
            email: &data.email,
            password: &password,
        };

        let _user = User::create(&mut database.unwrap(), user);

        Ok(Response::new(AccessToken {
            access_token: "".to_string(),
        }))
    }
}
