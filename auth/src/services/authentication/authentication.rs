use crate::models::user::{NewUser, User};
use bcrypt::verify;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use protos::authentication::{
    auth_server::Auth, AccessToken, LoginByEmailRequest, LoginByUsernameRequest, RegisterRequest,
};
use tonic::{Request, Response, Status};

use super::generate::generate_token;
use super::messages::MessageService;
pub struct Service {
    database: Pool<ConnectionManager<PgConnection>>,
}

impl Service {
    pub fn new(database: Pool<ConnectionManager<PgConnection>>) -> Self {
        Self { database: database }
    }
}

#[tonic::async_trait]
impl Auth for Service {
    async fn login_by_email(
        &self,
        request: Request<LoginByEmailRequest>,
    ) -> Result<Response<AccessToken>, Status> {
        let data = request.into_inner();

        let mut db = self.database.get().unwrap();

        let user = User::find_by_email(&mut db, &data.email).ok_or(Status::unauthenticated(
            MessageService::INVALID_EMAIL_OR_PASSWORD,
        ))?;

        match verify(data.password, &user.password) {
            Ok(true) => (),
            Ok(false) | Err(_) => {
                return Err(Status::unauthenticated(
                    MessageService::INVALID_EMAIL_OR_PASSWORD,
                ))
            }
        };

        let reply = generate_token(user)
            .map_err(|_| Status::unauthenticated(MessageService::UNAUTHENTICATED))?;

        Ok(Response::new(reply))
    }

    async fn login_by_username(
        &self,
        request: Request<LoginByUsernameRequest>,
    ) -> Result<Response<AccessToken>, Status> {
        let data = request.into_inner();

        let mut db = self.database.get().unwrap();

        let user = User::find_by_username(&mut db, &data.username).ok_or(
            Status::unauthenticated(MessageService::INVALID_USERNAME_OR_PASSWORD),
        )?;

        match verify(data.password, &user.password) {
            Ok(true) => (),
            Ok(false) | Err(_) => {
                return Err(Status::unauthenticated(
                    MessageService::INVALID_USERNAME_OR_PASSWORD,
                ))
            }
        };

        let reply = generate_token(user)
            .map_err(|_| Status::unauthenticated(MessageService::UNAUTHENTICATED))?;

        Ok(Response::new(reply))
    }

    async fn register(
        &self,
        request: Request<RegisterRequest>,
    ) -> Result<Response<AccessToken>, Status> {
        let mut db = self.database.get().unwrap();
        let data = request.into_inner();

        let password = bcrypt::hash(&data.password, 10)
            .map_err(|_| Status::unknown(MessageService::ERROR_CREATING_USER))?;

        let user = NewUser {
            username: &data.username,
            email: &data.email,
            password: &password,
        };

        let user = User::create(&mut db, user)
            .map_err(|_| Status::already_exists(MessageService::USER_ALREADY_EXISTS))?;

        let token = generate_token(user)
            .map_err(|_| Status::unknown(MessageService::ERROR_GENERATING_TOKEN))?;

        Ok(Response::new(token))
    }
}
