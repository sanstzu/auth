use time::OffsetDateTime;

use crate::{app::auth::utils, consts::CONFIG, models::JwtClaims};

pub enum VerifyTokenError {
    InvalidToken,
    ExpiredToken,
}

pub fn verify(token: String) -> Result<JwtClaims, VerifyTokenError> {
    let res = utils::jwt::verify(token, CONFIG.get_refresh_token_secret());

    match res {
        Err(_) => Err(VerifyTokenError::InvalidToken),
        Ok(claims) => {
            if claims.get_expiry().clone() < OffsetDateTime::now_utc() {
                println!("Expired token");
                println!("{}", claims.get_expiry().clone());
                println!("{}", OffsetDateTime::now_utc());
                Err(VerifyTokenError::ExpiredToken)
            } else {
                Ok(claims)
            }
        }
    }
}
