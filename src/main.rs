mod db;
mod state;
mod model;
mod handlers;

use axum::{routing::get, Router};
use std::sync::{Arc, Mutex};
use sqlx::PgPool;
use crate::state::AppState;
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    env_logger::init();
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_pool = PgPool::connect(&*db_url).await.expect("Failed to connect to the database");

    let cache = Arc::new(Mutex::new(Vec::new()));
    let app_state = AppState {
        cache: Arc::clone(&cache),
        db_pool,
    };

    let app = Router::new()
        .route("/orders/:id", get(handlers::get_order_by_id))
        .with_state(app_state);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
