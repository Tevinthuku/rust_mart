use chrono::{DateTime, Utc};

use crate::{
    price::Price,
    product::{self, Product},
};

use super::PricingContracts;

pub struct DateRange {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

impl PricingContracts {
    async fn new_flash_sale(
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
