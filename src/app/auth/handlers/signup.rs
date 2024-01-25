use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Deserialize;

use crate::{app::auth::functions, models::ApiResponse, AppState};

#[derive(Deserialize)]
pub struct JsonPayload {
    username: String,
    password: String,
}

pub async fn handler(State(state): State<AppState>, Json(payload): Json<JsonPayload>) -> Response {
    let db = state.get_db();

    let signup_res = functions::user_create(db, &payload.username, &payload.password).await;

    match signup_res {
        Err(functions::UserCreateError::DatabaseError) => {
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
        Err(functions::UserCreateError::HashingError) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse {
                    code: 50001,
                    message: "Hashing error",
                    data: (),
                }),
            )
                .into_response()
        }
        Err(functions::UserCreateError::DuplicateUser) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(ApiResponse {
                    code: 40000,
                    message: "User already exists",
                    data: (),
                }),
            )
                .into_response()
        }
        Ok(()) => {
            return (
                StatusCode::OK,
                Json(ApiResponse {
                    code: 20000,
                    message: "User created",
                    data: (),
                }),
            )
                .into_response()
        }
    }
}
