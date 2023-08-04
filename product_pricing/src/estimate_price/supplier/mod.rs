use anyhow::bail;
use async_trait::async_trait;
use futures::future::join_all;
mod cheram_suppliers;

use super::{SupplierWithPrice, SKU};

#[async_trait]
pub trait SupplierPrice {
    fn name(&self) -> String;
    async fn price(&self, sku: SKU) -> anyhow::Result<u64>;
}

pub struct SupplierPriceExtractor;

impl SupplierPriceExtractor {
    pub async fn get_supplier_prices(sku: SKU) -> anyhow::Result<Vec<SupplierWithPrice>> {
        let suppliers: Vec<&dyn SupplierPrice> = vec![&cheram_suppliers::CheramSuppliers];
        
        let supplier_price_futures = suppliers
            .iter()
            .map(|supplier| async { (supplier.name(), supplier.price(sku.clone()).await) });

        let supplier_pricing = join_all(supplier_price_futures).await;

        let mut results = Vec::with_capacity(supplier_pricing.len());

        for (supplier, pricing_result) in supplier_pricing {
            match pricing_result {
                Ok(price) => {
                    results.push(SupplierWithPrice { supplier, price });
                }
                Err(err) => {
                    log::error!("Received error from supplier: {err:?} ")
                }
            }
        }
        if results.is_empty() {
            bail!("Received no prices from suppliers")
        }
        Ok(results)
    }
}
