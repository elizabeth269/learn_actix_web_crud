mod handlers;
mod models;
mod routes;

use actix_files::Files;
use actix_web::{web, App, HttpServer};
use handlers::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let data = web::Data::new(AppState {
        items: std::sync::Mutex::new(Vec::new()),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .configure(routes::configure)
            .service(Files::new("/", "./static").index_file("index.html"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
