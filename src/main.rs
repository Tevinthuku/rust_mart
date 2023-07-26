pub mod pool;
mod product;

use std::io;

use actix_web::{web, App, HttpServer};
use pool::connection_pool;

#[actix_web::main]
async fn main() -> io::Result<()> {
    let pool = connection_pool();
    HttpServer::new(move || {
        App::new()
            .configure(product::init_routes)
            .app_data(web::Data::new(pool.clone()))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
