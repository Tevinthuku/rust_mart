use anyhow::bail;
use async_trait::async_trait;
use futures::future::join_all;
mod cheram_suppliers;

use crate::price::Price;

use super::{Sku, SupplierWithPrice};

#[async_trait]
pub trait SupplierPrice {
    fn name(&self) -> String;
    async fn price(&self, sku: Sku) -> anyhow::Result<Price>;
}

pub struct SupplierPriceExtractor;

impl SupplierPriceExtractor {
    pub async fn get_supplier_prices(sku: Sku) -> anyhow::Result<Vec<SupplierWithPrice>> {
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
