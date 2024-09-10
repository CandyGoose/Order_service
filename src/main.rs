mod handlers;
mod models;
mod state;

use actix_web::{web, App, HttpServer};
use state::{AppState, example_order};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let orders = vec![example_order()];
    let state = web::Data::new(AppState {
        orders: std::sync::Mutex::new(orders),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(handlers::get_order)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
