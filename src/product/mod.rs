pub mod model;

use crate::{errors::ApiError, pool::DbPool};
use actix_web::web;
use anyhow::Context;

use serde::{Deserialize, Serialize};

use self::model::{Product, ProductInput};

#[derive(Serialize)]
struct ProductResponse {
    id: uuid::Uuid,
    name: String,
    description: String,
    quantity_available: u32,
    price: Option<u32>,
}

impl TryFrom<Product> for ProductResponse {
    type Error = ApiError;
    fn try_from(value: Product) -> Result<Self, Self::Error> {
        let quantity_available = value
            .quantity_available
            .try_into()
            .with_context(|| {
                format!(
                    "Invalid quantity_available for product with id {}",
                    value.id
                )
            })
            .map_err(ApiError::new_internal)?;
        let price = value
            .price
            .map(|price| {
                price
                    .try_into()
                    .with_context(|| format!("invalid price for product with id {}", value.id))
                    .map_err(ApiError::new_internal)
            })
            .transpose()?;
        Ok(ProductResponse {
            id: value.id,
            name: value.name,
            description: value.description,
            quantity_available,
            price,
        })
    }
}

async fn create_product(
    data: web::Json<ProductInput>,
    pool: web::Data<DbPool>,
) -> Result<web::Json<ProductResponse>, ApiError> {
    let response = web::block(move || -> Result<_, ApiError> {
        let mut conn = pool.get()?;
        let product = Product::insert(data.clone(), &mut conn)?;
        ProductResponse::try_from(product)
    })
    .await??;

    Ok(web::Json(response))
}

#[derive(Deserialize)]
struct PriceUpdate {
    price: i32,
}

async fn update_price(
    product_id: web::Path<uuid::Uuid>,
    pool: web::Data<DbPool>,
    data: web::Json<PriceUpdate>,
) -> Result<web::Json<ProductResponse>, ApiError> {
    let product = web::block(move || {
        let mut conn = pool.get()?;
        let updated_product =
            Product::update_price(product_id.into_inner(), data.price, &mut conn)?;
        ProductResponse::try_from(updated_product)
    })
    .await??;

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
