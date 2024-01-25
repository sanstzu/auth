use time::{Duration, OffsetDateTime};

use crate::app::auth::utils;
use crate::consts::CONFIG;
use crate::models::{JwtClaims, User};

#[derive(Debug)]
pub enum GenerateAccessTokenError {
    HashingError,
}

pub fn generate_access_token(user: &User) -> Result<String, GenerateAccessTokenError> {
    let expiry = OffsetDateTime::now_utc().saturating_add(Duration::minutes(30));
    let claims = JwtClaims::new(user.get_id().to_string(), expiry);

    let res = utils::jwt::sign(claims, CONFIG.get_access_token_secret());

    match res {
        Err(_) => Err(GenerateAccessTokenError::HashingError),
        Ok(res) => Ok(res.as_str().to_owned()),
    }
}
