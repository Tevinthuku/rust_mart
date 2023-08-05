use async_trait::async_trait;

use crate::{price::Price, price_estimate::Sku};

use super::CompetitorPrice;

pub struct GoMart {}

#[async_trait]
impl CompetitorPrice for GoMart {
    fn name(&self) -> String {
        "Go Mart".to_string()
    }
    async fn price(&self, _sku: Sku) -> anyhow::Result<Price> {
        todo!()
    }
}
