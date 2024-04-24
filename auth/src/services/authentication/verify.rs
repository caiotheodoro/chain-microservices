use std::{collections::HashMap, env};

use hmac::{Hmac, Mac};
use jwt::VerifyWithKey;
use sha2::Sha256;

use super::messages::MessageService;

pub struct VerifyTokenError;

pub fn verify_token(token: &str) -> Result<bool, VerifyTokenError> {
    let app_key: String = env::var("APP_KEY").expect(MessageService::APP_KEY_MISSING);

    let key: Hmac<Sha256> =
        Hmac::new_from_slice(app_key.as_bytes()).map_err(|_| VerifyTokenError)?;

    Ok(token
        .verify_with_key(&key)
        .map(|_: HashMap<String, String>| true)
        .unwrap_or(false))
}
