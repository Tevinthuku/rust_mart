use async_trait::async_trait;

use crate::{price::Price, price_estimate::Sku};

use super::CompetitorPrice;

pub struct JsMart {}

#[async_trait]
impl CompetitorPrice for JsMart {
    fn name(&self) -> String {
        "JS Mart".to_string()
    }
    async fn price(&self, _sku: Sku) -> anyhow::Result<Price> {
        todo!()
    }
}
