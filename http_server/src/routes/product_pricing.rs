use actix_web::web;
use product_pricing::{price::Margin, DateRange, Price};
use serde::{Deserialize, Serialize};

use crate::errors::ApiError;

#[derive(Deserialize)]
struct PriceInput {
    price: u16,
}

async fn increase_price(
    product_id: web::Path<uuid::Uuid>,
    product_pricing: web::Data<product_pricing::PricingContracts>,
    data: web::Json<PriceInput>,
) -> Result<web::Json<product_pricing::Product>, ApiError> {
    let new_price = product_pricing::Price::new(data.price)?;
    let updated_product = product_pricing
        .increase_price(product_id.into_inner(), new_price)
        .await?;

    Ok(web::Json(updated_product))
}

#[derive(Deserialize)]
struct FlashSaleInput {
    date_range: DateRange,
    price: u16,
}

async fn new_flash_sale(
    product_id: web::Path<uuid::Uuid>,
    product_pricing: web::Data<product_pricing::PricingContracts>,
    data: web::Json<FlashSaleInput>,
) -> Result<web::Json<product_pricing::Product>, ApiError> {
    let flash_sale = product_pricing::Price::new(data.price)?;
    let product = product_pricing
        .new_flash_sale(product_id.into_inner(), data.date_range.clone(), flash_sale)
        .await?;

    Ok(web::Json(product))
}

#[derive(Deserialize)]
struct MarginInput {
    margin: u16,
}

async fn price_estimate(
    sku: web::Path<String>,
    product_pricing: web::Data<product_pricing::PricingContracts>,
    data: web::Json<MarginInput>,
) -> Result<web::Json<product_pricing::PriceEstimate>, ApiError> {
    let sku = sku.into_inner().as_str().into();
    let desired_margin = Margin::new(data.margin)?;
    product_pricing
        .estimate_price(sku, desired_margin)
        .await
        .map(web::Json)
        .map_err(ApiError::from)
}

#[derive(Serialize)]
pub struct ProductPriceResponse {
    price: Option<Price>,
}

async fn product_price(
    product_id: web::Path<uuid::Uuid>,
    product_pricing: web::Data<product_pricing::PricingContracts>,
) -> Result<web::Json<ProductPriceResponse>, ApiError> {
    product_pricing
        .get_product_price(product_id.into_inner())
        .await
        .map(|price| web::Json(ProductPriceResponse { price }))
        .map_err(ApiError::from)
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/product_pricing")
            .service(
                web::scope("/{product_id}")
                    .service(
                        web::resource("/increase_price").route(web::patch().to(increase_price)),
                    )
                    .service(web::resource("/flash_sale").route(web::post().to(new_flash_sale)))
                    .service(web::resource("").route(web::get().to(product_price))),
            )
            .service(
                web::scope("/{sku}").service(
                    web::resource("/price_estimate").route(web::post().to(price_estimate)),
                ),
            ),
    );
}
