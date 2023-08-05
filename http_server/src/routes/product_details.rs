use crate::errors::ApiError;
use actix_web::web;
use futures::try_join;
use product_metadata::Product;
use product_pricing::Price;
use serde::Serialize;

#[derive(Serialize)]
pub struct ProductResponse {
    #[serde(flatten)]
    price: Option<Price>,
    #[serde(flatten)]
    meta_data: Product,
}
async fn product_details(
    product_id: web::Path<uuid::Uuid>,
    product_pricing: web::Data<product_pricing::PricingContracts>,
    product_metadata: web::Data<product_metadata::ProductMetaDataContracts>,
) -> Result<web::Json<ProductResponse>, ApiError> {
    let product_id = product_id.into_inner();

    let (price, meta_data) = try_join!(
        product_pricing.get_product_price(product_id),
        product_metadata.get_product_metadata(product_id)
    )?;

    Ok(web::Json(ProductResponse { price, meta_data }))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/product").service(
            web::scope("/{product_id}")
                .service(web::resource("").route(web::get().to(product_details))),
        ),
    );
}
