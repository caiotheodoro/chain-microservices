use std::{
    collections::BTreeMap,
    env,
    time::{SystemTime, UNIX_EPOCH},
};

use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use protos::authentication::TokenResponse;
use sha2::Sha256;

use crate::models::user::User;

use super::messages::MessageService;

pub struct GenerateClaimsError;

pub fn generate_claims(user: &User) -> Result<BTreeMap<&'static str, String>, GenerateClaimsError> {
    let mut claims: BTreeMap<&str, String> = BTreeMap::new();

    claims.insert("sub", user.id.to_string());

    let current_timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| GenerateClaimsError)?
        .as_secs();

    claims.insert("iat", current_timestamp.to_string());
    claims.insert("exp", String::from("3600"));

    Ok(claims)
}
pub struct GenerateTokenError;

pub fn generate_token(user: &User) -> Result<TokenResponse, GenerateTokenError> {
    let access_key: String = env::var("ACCESS_KEY").expect(MessageService::APP_KEY_MISSING);
    let refresh_key: String = env::var("REFRESH_KEY").expect(MessageService::APP_KEY_MISSING);

    let claims = generate_claims(user).map_err(|_| GenerateTokenError)?;

    let access_key: Hmac<Sha256> =
        Hmac::new_from_slice(access_key.as_bytes()).map_err(|_| GenerateTokenError)?;

    let refresh_key: Hmac<Sha256> =
        Hmac::new_from_slice(refresh_key.as_bytes()).map_err(|_| GenerateTokenError)?;

    let access_token = claims
        .clone()
        .sign_with_key(&access_key)
        .map_err(|_| GenerateTokenError)?;

    let refresh_token = claims
        .clone()
        .sign_with_key(&refresh_key)
        .map_err(|_| GenerateTokenError)?;

    Ok(TokenResponse {
        access_token: access_token,
        refresh_token: refresh_token,
    })
}
