use actix_web::web;
use repository::{cart::CartResponse, Repository};

use crate::errors::ApiError;

async fn create_cart(repo: web::Data<Repository>) -> Result<web::Json<CartResponse>, ApiError> {
    repo.create_cart()
        .await
        .map(web::Json)
        .map_err(ApiError::from)
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/cart").service(web::resource("/new").route(web::post().to(create_cart))),
    );
}
