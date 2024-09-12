use axum::{extract::{Path, State}, Json, http::StatusCode};
use crate::state::AppState;
use log::{info, error};
use tokio::time::{timeout, Duration};
use serde_json::json;
use crate::db::fetch_order_by_id_from_db;

pub async fn get_order_by_id(
    Path(order_uid): Path<String>,
    State(state): State<AppState>
) -> (StatusCode, Json<serde_json::Value>) {
    info!("Received request for order with id: {}", order_uid);
    // Валидация формата ID (например, только алфавитно-цифровые символы)
    if !order_uid.chars().all(|c| c.is_alphanumeric()) {
        error!("Invalid order ID format: {}", order_uid);
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Invalid order ID format"})),
        );
    }

    // Попробуем найти заказ в кэше
    if let Some(order) = state.cache.lock().unwrap().iter().find(|o| o.order_uid == order_uid) {
        info!("Order found in cache with id {}", order.order_uid);
        return (
            StatusCode::OK,
            Json(json!(order))
        );
    }

    // Если в кэше нет, то ищем в базе данных
    match timeout(Duration::from_secs(5), fetch_order_by_id_from_db(&state.db_pool, order_uid.clone())).await {
        Ok(result) => match result {
            Ok(order) => {
                if let Some(order) = order {
                    info!("Successfully fetched order with id {}", order.order_uid);

                    state.cache.lock().unwrap().push(order.clone());

                    (
                        StatusCode::OK,
                        Json(json!(order))
                    )
                } else {
                    info!("No order found with id {}", order_uid);
                    (
                        StatusCode::NOT_FOUND,
                        Json(json!({"error": "Order not found"}))
                    )
                }
            },
            Err(e) => {
                error!("Failed to fetch order from the database: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": "Failed to fetch order from the database"}))
                )
            }
        },
        Err(_) => {
            error!("Database fetch timed out");
            (
                StatusCode::REQUEST_TIMEOUT,
                Json(json!({"error": "Database fetch timed out"}))
            )
        }
    }
}
