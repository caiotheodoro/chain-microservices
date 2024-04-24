use protos::message::{messaging_server::Messaging, MessageRequest, MessageResponse};
use tonic::{Request, Response, Status};

use crate::services::authentication::{messages::MessageService, verify::verify_token};

#[derive(Default)]
pub struct Service {}

#[tonic::async_trait]
impl Messaging for Service {
    async fn message(
        &self,
        request: Request<MessageRequest>,
    ) -> Result<Response<MessageResponse>, Status> {
        let token = request
            .metadata()
            .get("x-authorization")
            .ok_or(Status::unauthenticated(
                MessageService::NO_ACCESS_TOKEN_SPECIFIED,
            ))?
            .to_str()
            .map_err(|_| Status::unauthenticated(MessageService::NO_ACCESS_TOKEN_SPECIFIED))?;

        match verify_token(token) {
            Ok(true) => (),
            Err(_) | Ok(false) => {
                return Err(Status::unauthenticated(MessageService::INVALID_TOKEN))
            }
        }

        let data = request.into_inner();

        Ok(Response::new(MessageResponse {
            message: data.message,
        }))
    }
}
