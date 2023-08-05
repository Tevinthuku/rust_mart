use super::ProductMetaDataContracts;
use crate::product::Product;
use anyhow::Context;

impl ProductMetaDataContracts {
    pub async fn get_product_metadata(&self, product_id: uuid::Uuid) -> anyhow::Result<Product> {
        sqlx::query!(
            r#"
                SELECT id, name, description, sku FROM product WHERE id = $1
            "#,
            product_id
        )
        .fetch_one(self.pool.get())
        .await
        .context("DB error encountered when fetching product_metatata")
        .map(|row| Product {
            id: row.id,
            sku: row.sku,
            name: row.name,
            description: row.description,
        })
    }
}
