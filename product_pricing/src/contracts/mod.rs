use std::sync::Arc;

use pool_and_migrations::pool::Pool;

mod increase_price;
mod new_flash_sale;
mod price_estimate;
mod product_price;

pub use new_flash_sale::DateRange;

#[derive(Clone)]
pub struct PricingContracts {
    pool: Arc<Pool>,
}

impl PricingContracts {
    pub fn new(pool: Arc<Pool>) -> Self {
        Self { pool }
    }
}
