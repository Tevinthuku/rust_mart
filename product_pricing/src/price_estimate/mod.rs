use crate::{
    price::{Margin, Price},
    price_estimate::{competitors::CompetitorPriceExtractor, supplier::SupplierPriceExtractor},
};

mod competitors;
mod supplier;

#[derive(Clone)]
pub struct SKU(String);

impl AsRef<str> for SKU {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

pub struct PriceEstimator {
    desired_margin: Margin,
}

pub struct CompetitionPrice {
    competitor: String,
    price: Price,
}

pub enum ExchangeRate {
    Eur { value: usize },
    Usd { value: usize },
}

pub struct SupplierWithPrice {
    supplier: String,
    price: Price,
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
    break_down_cost: Price,
    criteria: Criteria,
    category: BreakDownCategory,
}

pub struct PriceEstimate {
    total_estimate_price: Price,
    break_downs: Vec<BreakDown>,
}

impl PriceEstimator {
    pub fn new(desired_margin: Margin) -> Self {
        Self { desired_margin }
    }

    pub async fn estimate(&self, sku: SKU) -> anyhow::Result<PriceEstimate> {
        let supplier_breakdown = {
            let supplier_prices = SupplierPriceExtractor::get_supplier_prices(sku.clone()).await?;
            let median_supplier_price =
                supplier_prices.iter().map(|data| data.price).sum::<Price>()
                    / supplier_prices.len();
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
                    break_down_cost: Price::zero(),
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
                if !b.break_down_cost.is_zero() {
                    Some(b.break_down_cost)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        let non_zero_break_down_count = non_zero_break_down_costs.len();
        let sum = non_zero_break_down_costs.into_iter().sum::<Price>();

        let estimate_price = sum / non_zero_break_down_count;

        PriceEstimate {
            total_estimate_price: estimate_price + self.desired_margin,
            break_downs,
        }
    }
}
