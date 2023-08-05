use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::{
    price::Price,
    product::{self, Product},
};

use super::PricingContracts;

#[derive(Deserialize, Clone)]
pub struct DateRange {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

impl PricingContracts {
    pub async fn new_flash_sale(
        &self,
        product: uuid::Uuid,
        range: DateRange,
        flash_sale_price: Price,
    ) -> anyhow::Result<Product> {
        let product = product::Product::get(product, &self.pool).await?;
        product
            .new_flash_sale(flash_sale_price, range.start, range.end, &self.pool)
            .await
    }
}
