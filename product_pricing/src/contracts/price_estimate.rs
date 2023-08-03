use crate::estimate_price::{PriceEstimate, PriceEstimator, SKU};

use super::PricingContracts;

impl PricingContracts {
    pub async fn estimate_price(
        &self,
        sku: SKU,
        desired_margin: u8,
    ) -> anyhow::Result<PriceEstimate> {
        let price_estimator = PriceEstimator::new(desired_margin);
        price_estimator.estimate(sku).await
    }
}
