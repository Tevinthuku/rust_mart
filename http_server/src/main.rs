pub mod cart;
pub mod errors;
mod product;
use std::io;

use actix_web::{middleware::Logger, web, App, HttpServer};

#[actix_web::main]
async fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");
    let repo = repository::Repository::new().expect("Repository to be initialized");
    env_logger::init();
    HttpServer::new(move || {
        let logger = Logger::default();

        App::new()
            .configure(product::init_routes)
            .configure(cart::init_routes)
            .wrap(logger)
            .app_data(web::Data::new(repo.clone()))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
