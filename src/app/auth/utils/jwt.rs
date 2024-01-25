use hmac::{Hmac, Mac};
use jwt::{token::Signed, Header, SignWithKey, Token, VerifyWithKey};
use sha2::Sha384;

use axum::BoxError;

use crate::models::JwtClaims;

pub type SignedJwtToken = Token<Header, JwtClaims, Signed>;

pub fn sign(claims: JwtClaims, key: &str) -> Result<SignedJwtToken, BoxError> {
    let hash_key: Hmac<Sha384> = Hmac::new_from_slice(key.as_bytes())?;
    let header = Header {
        algorithm: jwt::AlgorithmType::Hs384,
        ..Default::default()
    };

    let token = Token::new(header, claims).sign_with_key(&hash_key)?;

    Ok(token)
}

pub fn verify(token_string: String, key: &str) -> Result<JwtClaims, BoxError> {
    let hash_key: Hmac<Sha384> = Hmac::new_from_slice(key.as_bytes())?;

    let claims: JwtClaims = token_string.verify_with_key(&hash_key)?;

    Ok(claims)
}
