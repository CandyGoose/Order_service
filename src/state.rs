use std::sync::{Arc, Mutex};
use sqlx::PgPool;
use crate::model::Order;

#[derive(Clone)]
pub struct AppState {
    pub cache: Arc<Mutex<Vec<Order>>>,
    pub db_pool: PgPool,
}
