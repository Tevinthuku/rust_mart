use crate::{
    price::Margin,
    price_estimate::{PriceEstimate, PriceEstimator, Sku},
};

use super::PricingContracts;

impl PricingContracts {
    pub async fn estimate_price(
        &self,
        sku: Sku,
        desired_margin: Margin,
    ) -> anyhow::Result<PriceEstimate> {
        let price_estimator = PriceEstimator::new(desired_margin);
        price_estimator.estimate(sku).await
    }
}
