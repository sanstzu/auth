use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};

use crate::{
    app::auth::functions,
    models::{ApiResponse, User},
    AppState,
};

#[derive(Deserialize)]
pub struct JsonPayload {
    refresh_token: String,
}

pub async fn handler(State(state): State<AppState>, Json(payload): Json<JsonPayload>) -> Response {
    let db = state.get_db();

    let verify_res = functions::verify(payload.refresh_token);

    match verify_res {
        Err(functions::VerifyTokenError::InvalidToken) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(ApiResponse {
                    code: 40100,
                    message: "Invalid token",
                    data: (),
                }),
            )
                .into_response()
        }
        Err(functions::VerifyTokenError::ExpiredToken) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(ApiResponse {
                    code: 40101,
                    message: "Expired token",
                    data: (),
                }),
            )
                .into_response()
        }
        Ok(claims) => {
            let user_res = User::query_by_id(db, claims.get_id()).await;

            match user_res {
                Err(_) => {
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(ApiResponse {
                            code: 50000,
                            message: "Database error",
                            data: (),
                        }),
                    )
                        .into_response()
                }
                Ok(None) => {
                    return (
                        StatusCode::UNAUTHORIZED,
                        Json(ApiResponse {
                            code: 40100,
                            message: "Invalid token",
                            data: (),
                        }),
                    )
                        .into_response()
                }
                Ok(Some(user)) => {
                    let access_token = functions::generate_access_token(&user);

                    match access_token {
                        Err(functions::GenerateAccessTokenError::HashingError) => {
                            return (
                                StatusCode::INTERNAL_SERVER_ERROR,
                                Json(ApiResponse {
                                    code: 50000,
                                    message: "Hashing error",
                                    data: (),
                                }),
                            )
                                .into_response()
                        }
                        Ok(access_token) => {
                            #[derive(Serialize)]
                            struct Response {
                                access_token: String,
                            }

                            let resp = Response { access_token };
                            return (
                                StatusCode::OK,
                                Json(ApiResponse {
                                    code: 20000,
                                    message: "",
                                    data: resp,
                                }),
                            )
                                .into_response();
                        }
                    }
                }
            }
        }
    }
}
