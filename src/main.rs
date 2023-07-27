pub mod errors;
pub mod pool;
mod product;
pub mod schema;
use std::io;

use actix_web::{middleware::Logger, web, App, HttpServer};
use pool::connection_pool;

#[actix_web::main]
async fn main() -> io::Result<()> {
    let pool = connection_pool();
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    HttpServer::new(move || {
        let logger = Logger::default();

        App::new()
            .configure(product::init_routes)
            .wrap(logger)
            .app_data(web::Data::new(pool.clone()))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
