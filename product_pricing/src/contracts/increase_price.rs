use crate::{
    price::Price,
    product::{self, Product},
};

use super::PricingContracts;

impl PricingContracts {
    pub async fn increase_price(
        &self,
        product: uuid::Uuid,
        new_price: Price,
    ) -> anyhow::Result<Product> {
        let product = product::Product::get(product, &self.pool).await?;
        product.increase_price(new_price, &self.pool).await
    }
}
