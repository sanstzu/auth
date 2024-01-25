use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};

use crate::{app::auth::functions, AppState};
use crate::{app::auth::functions::LoginError, models::ApiResponse};

#[derive(Deserialize)]
pub struct JsonPayload {
    username: String,
    password: String,
}

pub async fn handler(State(state): State<AppState>, Json(payload): Json<JsonPayload>) -> Response {
    let db = state.get_db();

    let login_result = functions::user_login(&payload.username, &payload.password, &db).await;

    match login_result {
        Err(LoginError::UserNotExist) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(ApiResponse {
                    code: 40100,
                    message: "Incorrect user or password",
                    data: (),
                }),
            )
                .into_response()
        }
        Err(LoginError::PasswordIncorrect) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(ApiResponse {
                    code: 40101,
                    message: "Incorrect user or password",
                    data: (),
                }),
            )
                .into_response()
        }
        Err(LoginError::DatabaseError) => {
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
        Err(LoginError::VerifyError) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse {
                    code: 50001,
                    message: "Verify error",
                    data: (),
                }),
            )
                .into_response()
        }
        Ok(user) => {
            #[derive(Serialize)]
            struct ResponseData<'a> {
                access_token: &'a str,
                refresh_token: &'a str,
            }

            let access_token = functions::generate_access_token(&user);
            if let Err(_) = access_token {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiResponse {
                        code: 50002,
                        message: "Generate token error",
                        data: (),
                    }),
                )
                    .into_response();
            }

            let refresh_token = functions::generate_refresh_token(&user);
            if let Err(_) = refresh_token {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiResponse {
                        code: 50002,
                        message: "Generate token error",
                        data: (),
                    }),
                )
                    .into_response();
            }

            let response_data = ResponseData {
                access_token: &access_token.unwrap(),
                refresh_token: &refresh_token.unwrap(),
            };

            return (
                StatusCode::OK,
                Json(ApiResponse {
                    code: 20000,
                    message: "",
                    data: response_data,
                }),
            )
                .into_response();
        }
    }
}
