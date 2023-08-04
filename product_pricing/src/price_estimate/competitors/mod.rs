use async_trait::async_trait;
use futures::future::join_all;

use crate::price::Price;

use super::{CompetitionPrice, SKU};

mod go_mart;
mod js_mart;

#[async_trait]
pub trait CompetitorPrice {
    fn name(&self) -> String;
    async fn price(&self, sku: SKU) -> anyhow::Result<Price>;
}

pub struct CompetitorPriceExtractor {}

impl CompetitorPriceExtractor {
    pub async fn get_prices(sku: SKU) -> Vec<CompetitionPrice> {
        let competitors: Vec<&dyn CompetitorPrice> = vec![&go_mart::GoMart {}, &js_mart::JsMart {}];
        let competitor_futures = competitors
            .iter()
            .map(|competitor| async { (competitor.name(), competitor.price(sku.clone()).await) });

        let price_results = join_all(competitor_futures).await;
        let mut competitor_prices = Vec::with_capacity(price_results.len());
        for (competitor, price_result) in price_results {
            match price_result {
                Ok(price) => competitor_prices.push(CompetitionPrice { competitor, price }),
                Err(err) => {
                    log::error!("Failed to extract price from {competitor}: reason : {err:?}")
                }
            }
        }
        competitor_prices
    }
}
