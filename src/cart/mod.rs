use crate::{cart::model::CartModel, errors::ApiError, pool::DbPool};
use actix_web::web;
use serde::Serialize;

mod model;

#[derive(Serialize)]
struct CartResponse {
    id: uuid::Uuid,
    cart_items: Vec<CartItemResponse>,
}

#[derive(Serialize)]
struct CartItemResponse {
    id: uuid::Uuid,
    product_id: uuid::Uuid,
    quantity: u32,
}

async fn create_cart(pool: web::Data<DbPool>) -> Result<web::Json<CartResponse>, ApiError> {
    let response = web::block(move || -> Result<_, ApiError> {
        let mut conn = pool.get()?;
        CartModel::create_cart(&mut conn).map_err(ApiError::from)
    })
    .await??;

    Ok(web::Json(CartResponse {
        id: response.id,
        cart_items: Default::default(),
    }))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/cart").service(web::resource("/new").route(web::post().to(create_cart))),
    );
}
