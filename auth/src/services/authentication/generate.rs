use std::{
    collections::BTreeMap,
    env,
    time::{SystemTime, UNIX_EPOCH},
};

use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use protos::authentication::AccessToken;
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

pub fn generate_token(user: &User) -> Result<AccessToken, GenerateTokenError> {
    let app_key: String = env::var("APP_KEY").expect(MessageService::APP_KEY_MISSING);
    let claims = generate_claims(user).map_err(|_| GenerateTokenError)?;
    let key: Hmac<Sha256> =
        Hmac::new_from_slice(app_key.as_bytes()).map_err(|_| GenerateTokenError)?;

    let access_token = claims.sign_with_key(&key).map_err(|_| GenerateTokenError)?;

    Ok(AccessToken {
        access_token: access_token,
    })
}
