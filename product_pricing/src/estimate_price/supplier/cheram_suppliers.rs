use async_trait::async_trait;

use crate::estimate_price::SKU;

use super::SupplierPrice;
pub struct CheramSuppliers;

#[async_trait]
impl SupplierPrice for CheramSuppliers {
    fn name(&self) -> std::string::String { todo!() }
    async fn price(&self, sku: SKU) -> anyhow::Result<u64> {
        // Possibly intergrate with Cherams API to get the price
        todo!()
    }
}
