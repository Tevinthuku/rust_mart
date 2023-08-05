use actix_web::web;
mod product_details;
mod product_pricing;
pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.configure(product_pricing::init_routes)
        .configure(product_details::init_routes);
}
