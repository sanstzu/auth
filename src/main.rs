use app::{auth, protected};
use axum::Router;
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::{net::Ipv4Addr, sync::Arc};
use tokio::net::TcpListener;

use crate::consts::CONFIG;

mod app;
mod consts;
mod middleware;
mod models;
mod repository;

pub type AppState = Arc<InnerAppState>;

pub struct InnerAppState {
    db: PgPool,
}

impl InnerAppState {
    pub fn get_db(&self) -> &PgPool {
        &self.db
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let db = PgPoolOptions::new()
        .max_connections(5)
        .connect(CONFIG.get_database_connection())
        .await
        .unwrap();
    println!("🗄️ Connected to database");

    let state: AppState = Arc::new(InnerAppState { db });

    let router = Router::<AppState>::new()
        .nest("/auth", auth::routes())
        .nest("/protected", protected::routes())
        .with_state(state);

    let listener = TcpListener::bind((Ipv4Addr::new(0, 0, 0, 0), 8080))
        .await
        .unwrap();

    println!("🎧 Listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, router).await.unwrap();
}
