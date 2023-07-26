use actix_web::{web, HttpRequest};
use chrono::{DateTime, Utc};

struct Product {
    id: uuid::Uuid,
    name: String,
    quantity_on_hand: u32,
    quantity_available: u32,
    price: u64,
    created_at: DateTime<Utc>,
}

async fn create_product(_req: HttpRequest) -> String {
    "Hello world!".to_owned()
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/product").route(web::post().to(create_product)));
}
