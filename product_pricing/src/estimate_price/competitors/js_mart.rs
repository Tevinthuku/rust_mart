use async_trait::async_trait;

use crate::estimate_price::SKU;

use super::CompetitorPrice;

pub struct JsMart {}

#[async_trait]
impl CompetitorPrice for JsMart {
    fn name(&self) -> String {
        "JS Mart".to_string()
    }
    async fn price(&self, sku: SKU) -> anyhow::Result<u64> {
        todo!()
    }
}
