use axum::{
    http::StatusCode, middleware::from_extractor, response::IntoResponse, routing::get, Router,
};

use crate::{middleware::RequireAuth, AppState};

pub fn init_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/ping",
            get(|| async { (StatusCode::OK, "Pong").into_response() }),
        )
        .route_layer(from_extractor::<RequireAuth>())
}
