use time::{Duration, OffsetDateTime};

use crate::app::auth::utils;
use crate::consts::CONFIG;
use crate::models::{JwtClaims, User};

#[derive(Debug)]
pub enum GenerateRefreshTokenError {
    HashingError,
}

pub fn generate_refresh_token(user: &User) -> Result<String, GenerateRefreshTokenError> {
    let expiry = OffsetDateTime::now_utc().saturating_add(Duration::days(30));
    let claims = JwtClaims::new(user.get_id().to_string(), expiry);

    let res = utils::jwt::sign(claims, CONFIG.get_refresh_token_secret());

    match res {
        Err(_) => Err(GenerateRefreshTokenError::HashingError),
        Ok(res) => Ok(res.as_str().to_owned()),
    }
}
