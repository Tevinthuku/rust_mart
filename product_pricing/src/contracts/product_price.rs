use crate::{price::Price, product};

use super::PricingContracts;

impl PricingContracts {
    pub async fn get_product_price(&self, product: uuid::Uuid) -> anyhow::Result<Option<Price>> {
        product::Product::get(product, &self.pool)
            .await
            .map(|product| product.price())
    }
}
