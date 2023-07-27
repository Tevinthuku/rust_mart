use crate::{errors::ApiError, pool::DbPool, schema::product};
use actix_web::web;
use anyhow::Context;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::Deserialize;
use serde::Serialize;

#[derive(Queryable, Selectable, Deserialize, Serialize)]
#[diesel(table_name = product)]
struct Product {
    id: uuid::Uuid,
    name: String,
    description: String,
    quantity_on_hand: i32,
    quantity_available: i32,
    price: Option<i32>,
    created_at: DateTime<Utc>,
}

#[derive(Deserialize, Insertable, Clone)]
#[diesel(table_name = product)]
struct ProductInput {
    name: String,
    description: String,
    quantity_on_hand: i32,
}

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
        let product = data
            .clone()
            .insert_into(product::table)
            .get_result::<Product>(&mut conn)?;
        ProductResponse::try_from(product)
    })
    .await??;

    Ok(web::Json(response))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/product").route(web::post().to(create_product)));
}
