use async_trait::async_trait;

use crate::{price::Price, price_estimate::SKU};

use super::CompetitorPrice;

pub struct GoMart {}

#[async_trait]
impl CompetitorPrice for GoMart {
    fn name(&self) -> String {
        "Go Mart".to_string()
    }
    async fn price(&self, sku: SKU) -> anyhow::Result<Price> {
        todo!()
    }
}
