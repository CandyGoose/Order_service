use actix_web::{get, web, Responder, HttpResponse};
use crate::state::AppState;

// API для получения данных о заказе
#[get("/orders/{order_uid}")]
pub async fn get_order(data: web::Data<AppState>, order_uid: web::Path<String>) -> impl Responder {
    let orders = data.orders.lock().unwrap();
    match orders.iter().find(|order| order.order_uid == *order_uid) {
        Some(order) => HttpResponse::Ok().json(order),
        None => HttpResponse::NotFound().body("Order not found"),
    }
}
