use actix_web::{web, App, HttpServer};
use handlers::{create_item, delete_item, get_item, get_items, update_item, AppState}; // Import AppState
use std::sync::Arc;
use std::sync::Mutex;

mod handlers;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let data = web::Data::new(AppState {
        items: Arc::new(Mutex::new(Vec::new())),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(create_item)
            .service(get_items)
            .service(get_item)
            .service(update_item)
            .service(delete_item)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
