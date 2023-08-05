use std::{env, sync::Arc};

use actix_web::{middleware::Logger, web, App, HttpServer};
use anyhow::Context;
pub mod errors;
mod routes;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");

    let max_connections = env::var("MAX_CONNECTIONS")
        .unwrap_or("10".to_string())
        .parse::<u32>()
        .context("Failed to parse MAX_CONNECTIONS")?;

    let pool = pool_and_migrations::pool::Pool::new(max_connections)
        .await
        .map(Arc::new)?;
    let product_pricing_contracts = product_pricing::contracts::PricingContracts::new(pool.clone());

    env_logger::init();
    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .service(web::scope("/api").configure(routes::init_routes))
            .app_data(web::Data::new(product_pricing_contracts.clone()))
    })
    .bind(("127.0.0.1", 8080))
    .context("Failed to bind http server")?
    .run()
    .await
    .context("Failed to run server")
}
