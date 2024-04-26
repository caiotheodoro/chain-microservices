use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use protos::client::{
    client_server::Client, ClientRequest, ClientResponse, ListUserRequest, ListUserResponse,
};
use tonic::{Request, Response, Status};

use crate::{models::user::User, services::authentication::messages::MessageService};

pub struct Service {
    database: Pool<ConnectionManager<PgConnection>>,
}

impl Service {
    pub fn new(database: Pool<ConnectionManager<PgConnection>>) -> Self {
        Self { database: database }
    }
}

#[tonic::async_trait]
impl Client for Service {
    async fn get_user(
        &self,
        request: Request<ClientRequest>,
    ) -> Result<Response<ClientResponse>, Status> {
        let mut db = self.database.get().unwrap();

        let data = request.into_inner();

        let user = User::get_user(&mut db, &data.id)
            .ok_or(Status::unauthenticated(MessageService::ERROR_GETTING_USER))?;

        Ok(Response::new(ClientResponse {
            id: user.id,
            username: user.username,
            email: user.email,
        }))
    }

    async fn list_users(
        &self,
        _request: Request<ListUserRequest>,
    ) -> Result<Response<ListUserResponse>, Status> {
        let mut db = self.database.get().unwrap();

        let users = User::list_users(&mut db)
            .map_err(|_| Status::unauthenticated(MessageService::ERROR_GETTING_USERS))?;

        let client_responses: Vec<ClientResponse> = users
            .into_iter()
            .map(|user| ClientResponse {
                id: user.id,
                username: user.username,
                email: user.email,
            })
            .collect();

        Ok(Response::new(ListUserResponse {
            users: client_responses,
        }))
    }
}
