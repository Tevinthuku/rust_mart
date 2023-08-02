use crate::estimate_price::supplier::SupplierPriceExtractor;

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
pub enum BreakDownReason {
    CostOfGoods(Vec<SupplierWithPrice>),
    Margin,
    Competition(Vec<CompetitionPrice>),
    CurrencyExchangeRate(ExchangeRate),
    ShippingCost,
}

pub struct BreakDown {
    break_down_cost: u64,
    reason: BreakDownReason,
}

pub struct PriceEstimate {
    price: u64,
    break_downs: Vec<BreakDown>,
}

impl PriceEstimator {
    pub fn new(desired_margin: u8) -> Self {
        Self { desired_margin }
    }

    async fn estimate(&self, sku: SKU) -> anyhow::Result<PriceEstimate> {
        let supplier_prices = SupplierPriceExtractor::get_supplier_prices(sku.clone()).await?;
        todo!()
    }
}
