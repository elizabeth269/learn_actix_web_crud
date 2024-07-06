use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use uuid::Uuid;

use crate::models::Item;

#[derive(Debug, Clone)]
pub struct AppState {
    items: std::sync::Mutex<Vec<Item>>,
}

#[post("/items")]
async fn create_item(item: web::Json<Item>, data: web::Data<AppState>) -> impl Responder {
    let mut items = data.items.lock().unwrap();
    let new_item = Item {
        id: Uuid::new_v4(),
        name: item.name.clone(),
    };
    items.push(new_item.clone());
    HttpResponse::Created().json(new_item)
}

#[get("/items")]
async fn get_items(data: web::Data<AppState>) -> impl Responder {
    let items = data.items.lock().unwrap();
    HttpResponse::Ok().json(items.clone())
}

#[get("/items/{id}")]
async fn get_item(path: web::Path<Uuid>, data: web::Data<AppState>) -> impl Responder {
    let items = data.items.lock().unwrap();
    let id = path.into_inner();
    if let Some(item) = items.iter().find(|&item| item.id == id) {
        HttpResponse::Ok().json(item)
    } else {
        HttpResponse::NotFound().finish()
    }
}

#[put("/items/{id}")]
async fn update_item(
    path: web::Path<Uuid>,
    item: web::Json<Item>,
    data: web::Data<AppState>,
) -> impl Responder {
    let mut items = data.items.lock().unwrap();
    let id = path.into_inner();
    if let Some(existing_item) = items.iter_mut().find(|item| item.id == id) {
        existing_item.name = item.name.clone();
        HttpResponse::Ok().json(existing_item)
    } else {
        HttpResponse::NotFound().finish()
    }
}

#[delete("/items/{id}")]
async fn delete_item(path: web::Path<Uuid>, data: web::Data<AppState>) -> impl Responder {
    let mut items = data.items.lock().unwrap();
    let id = path.into_inner();
    if let Some(pos) = items.iter().position(|item| item.id == id) {
        items.remove(pos);
        HttpResponse::NoContent().finish()
    } else {
        HttpResponse::NotFound().finish()
    }
}
