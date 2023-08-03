use crate::estimate_price::{
    competitors::CompetitorPriceExtractor, supplier::SupplierPriceExtractor,
};

mod competitors;
mod supplier;

#[derive(Clone)]
pub struct SKU(String);

pub struct PriceEstimator {
    desired_margin: u8,
}

pub struct CompetitionPrice {
    competitor: String,
    price: u64,
}

pub enum ExchangeRate {
    Eur { value: usize },
    Usd { value: usize },
}

pub struct SupplierWithPrice {
    supplier: String,
    price: u64,
}

pub enum BreakDownCategory {
    CostOfGoods(Vec<SupplierWithPrice>),
    Competition(Vec<CompetitionPrice>),
}

pub enum Criteria {
    MeanPrice,
    MinimumPrice,
    Custom(String),
}

pub struct BreakDown {
    break_down_cost: u64,
    criteria: Criteria,
    category: BreakDownCategory,
}

pub struct PriceEstimate {
    total_estimate_price: u64,
    mark_up: u64,
    break_downs: Vec<BreakDown>,
}

impl PriceEstimator {
    pub fn new(desired_margin: u8) -> Self {
        Self { desired_margin }
    }

    pub async fn estimate(&self, sku: SKU) -> anyhow::Result<PriceEstimate> {
        let supplier_breakdown = {
            let supplier_prices = SupplierPriceExtractor::get_supplier_prices(sku.clone()).await?;
            let median_supplier_price = supplier_prices.iter().map(|data| data.price).sum::<u64>()
                / supplier_prices.len() as u64;
            BreakDown {
                break_down_cost: median_supplier_price,
                criteria: Criteria::MeanPrice,
                category: BreakDownCategory::CostOfGoods(supplier_prices),
            }
        };

        let competitor_prices_break_down = {
            let competitor_prices = CompetitorPriceExtractor::get_prices(sku).await;
            let min_price = competitor_prices
                .iter()
                .min_by(|x, y| x.price.cmp(&y.price));
            match min_price {
                Some(CompetitionPrice { price, .. }) => BreakDown {
                    break_down_cost: *price,
                    criteria: Criteria::MinimumPrice,
                    category: BreakDownCategory::Competition(competitor_prices),
                },
                None => BreakDown {
                    break_down_cost: 0,
                    criteria: Criteria::Custom(
                        "We couldn't find any competitor prices on the product".to_string(),
                    ),
                    category: BreakDownCategory::Competition(competitor_prices),
                },
            }
        };

        let break_downs = vec![supplier_breakdown, competitor_prices_break_down];
        Ok(self.compute_estimate_price(break_downs))
    }

    fn compute_estimate_price(&self, break_downs: Vec<BreakDown>) -> PriceEstimate {
        let non_zero_break_down_costs = break_downs
            .iter()
            .filter_map(|b| {
                if b.break_down_cost != 0 {
                    Some(b.break_down_cost)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        let count = non_zero_break_down_costs.len();
        let sum = non_zero_break_down_costs.iter().sum::<u64>();

        let estimate_price = sum / count as u64;

        let mark_up = estimate_price * (self.desired_margin / 100) as u64;

        PriceEstimate {
            total_estimate_price: estimate_price + mark_up,
            mark_up,
            break_downs,
        }
    }
}
