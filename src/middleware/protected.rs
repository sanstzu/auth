use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{header, request::Parts, StatusCode},
    response::{IntoResponse, Response},
    Json,
};

use crate::{app::auth::functions::verify_access, models::ApiResponse};

pub struct RequireAuth;

fn unauthorized_response() -> Response {
    (
        StatusCode::UNAUTHORIZED,
        Json(ApiResponse {
            code: 40100,
            data: (),
            message: "Unauthorized access",
        }),
    )
        .into_response()
}

#[async_trait]
impl<S> FromRequestParts<S> for RequireAuth
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get(header::AUTHORIZATION)
            .and_then(|token| token.to_str().ok());

        let header_parts: Vec<String> = match auth_header {
            Some(header) => header
                .trim_end()
                .split(" ")
                .map(|x| x.to_string())
                .collect(),
            _ => return Err(unauthorized_response()),
        };

        match header_parts.get(0) {
            Some(string) => {
                if string != "Bearer" {
                    return Err(unauthorized_response());
                }
            }
            _ => return Err(unauthorized_response()),
        };

        let token: String = match header_parts.get(1) {
            Some(token) => token.clone(),
            None => return Err(unauthorized_response()),
        };

        match verify_access(token) {
            Ok(_) => Ok(Self),
            _ => Err(unauthorized_response()),
        }
    }
}
