use time::OffsetDateTime;

use crate::{app::auth::utils, consts::CONFIG, models::JwtClaims};

pub enum VerifyTokenError {
    InvalidToken,
    ExpiredToken,
}

pub fn verify_access(token: String) -> Result<JwtClaims, VerifyTokenError> {
    let res = utils::jwt::verify(token, CONFIG.get_access_token_secret());

    match res {
        Err(_) => Err(VerifyTokenError::InvalidToken),
        Ok(claims) => {
            if claims.get_expiry().clone() < OffsetDateTime::now_utc() {
                Err(VerifyTokenError::ExpiredToken)
            } else {
                Ok(claims)
            }
        }
    }
}

pub fn verify_refresh(token: String) -> Result<JwtClaims, VerifyTokenError> {
    let res = utils::jwt::verify(token, CONFIG.get_refresh_token_secret());

    match res {
        Err(_) => Err(VerifyTokenError::InvalidToken),
        Ok(claims) => {
            if claims.get_expiry().clone() < OffsetDateTime::now_utc() {
                Err(VerifyTokenError::ExpiredToken)
            } else {
                Ok(claims)
            }
        }
    }
}
