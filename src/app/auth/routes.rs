use axum::{routing::post, Router};

use crate::AppState;

use super::handlers::{login, refresh, signup};

pub fn init_routes() -> Router<AppState> {
    Router::<AppState>::new()
        .route("/login", post(login))
        .route("/signup", post(signup))
        .route("/refresh", post(refresh))
}
