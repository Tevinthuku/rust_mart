use crate::errors::ApiError;
use actix_web::web;

use repository::{product::ProductResponse, Repository};
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct ProductInput {
    name: String,
    description: String,
    quantity_on_hand: u32,
}

async fn create_product(
    data: web::Json<ProductInput>,
    repo: web::Data<Repository>,
) -> Result<web::Json<ProductResponse>, ApiError> {
    let data = data.into_inner();
    let response = repo
        .create_product(data.name, data.description, data.quantity_on_hand)
        .await?;

    Ok(web::Json(response))
}

#[derive(Deserialize)]
struct PriceUpdate {
    price: u32,
}

async fn update_price(
    product_id: web::Path<uuid::Uuid>,
    repo: web::Data<Repository>,
    data: web::Json<PriceUpdate>,
) -> Result<web::Json<ProductResponse>, ApiError> {
    let data = data.into_inner();
    let product = repo
        .update_product_price(product_id.into_inner(), data.price)
        .await?;

    Ok(web::Json(product))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/product")
            .service(web::resource("").route(web::post().to(create_product)))
            .service(
                web::scope("/{product_id}")
                    .service(web::resource("").route(web::patch().to(update_price))),
            ),
    );
}
