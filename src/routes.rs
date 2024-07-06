use actix_web::web;

use crate::handlers::{create_item, delete_item, get_item, get_items, update_item};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(create_item)
        .service(get_items)
        .service(get_item)
        .service(update_item)
        .service(delete_item);
}
