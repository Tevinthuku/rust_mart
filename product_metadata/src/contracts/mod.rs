use pool_and_migrations::pool::Pool;
use std::sync::Arc;
mod get_product_metadata;

#[derive(Clone)]
pub struct ProductMetaDataContracts {
    pool: Arc<Pool>,
}

impl ProductMetaDataContracts {
    pub fn new(pool: Arc<Pool>) -> Self {
        Self { pool }
    }
}
