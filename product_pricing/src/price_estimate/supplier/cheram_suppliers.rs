use async_trait::async_trait;

use crate::{price::Price, price_estimate::Sku};

use super::SupplierPrice;
pub struct CheramSuppliers;

#[async_trait]
impl SupplierPrice for CheramSuppliers {
    fn name(&self) -> std::string::String {
        todo!()
    }
    async fn price(&self, _sku: Sku) -> anyhow::Result<Price> {
        // Possibly intergrate with Cherams API to get the price
        todo!()
    }
}
