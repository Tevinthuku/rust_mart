use crate::{
    price::{Margin, Price},
    price_estimate::{PriceEstimate, PriceEstimator, SKU},
};

use super::PricingContracts;

impl PricingContracts {
    pub async fn estimate_price(
        &self,
        sku: SKU,
        desired_margin: Margin,
    ) -> anyhow::Result<PriceEstimate> {
        let price_estimator = PriceEstimator::new(desired_margin);
        price_estimator.estimate(sku).await
    }
}
