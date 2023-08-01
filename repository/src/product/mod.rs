pub mod model;

use crate::{errors::RepositoryError, Repository};
use anyhow::Context;

use serde::Serialize;

use self::model::{Product, ProductInput};

#[derive(Serialize)]
pub struct ProductResponse {
    id: uuid::Uuid,
    name: String,
    description: String,
    quantity_available: u32,
    price: Option<u32>,
}

impl TryFrom<Product> for ProductResponse {
    type Error = RepositoryError;
    fn try_from(value: Product) -> Result<Self, Self::Error> {
        let quantity_available = value
            .quantity_available
            .try_into()
            .with_context(|| {
                format!(
                    "Invalid quantity_available for product with id {}",
                    value.id
                )
            })
            .map_err(RepositoryError::new_internal)?;
        let price = value
            .price
            .map(|price| {
                price
                    .try_into()
                    .with_context(|| format!("invalid price for product with id {}", value.id))
                    .map_err(RepositoryError::new_internal)
            })
            .transpose()?;
        Ok(ProductResponse {
            id: value.id,
            name: value.name,
            description: value.description,
            quantity_available,
            price,
        })
    }
}

impl Repository {
    pub async fn create_product(
        &self,
        name: String,
        description: String,
        quantity_on_hand: u32,
    ) -> Result<ProductResponse, RepositoryError> {
        let quantity_on_hand = i32::try_from(quantity_on_hand)
            .map_err(|err| anyhow::Error::new(err).context("Failed to parse quantity_on_hand"))
            .map_err(RepositoryError::new_validation)?;

        let input = ProductInput {
            name,
            description,
            quantity_on_hand,
        };

        let repo = self.clone();
        tokio::task::spawn_blocking(move || -> Result<ProductResponse, RepositoryError> {
            let mut conn = repo.conn()?;

            let product = Product::insert(input, &mut conn)?;
            ProductResponse::try_from(product)
        })
        .await?
    }

    pub async fn update_product_price(
        &self,
        product: uuid::Uuid,
        new_price: u32,
    ) -> Result<ProductResponse, RepositoryError> {
        let new_price = i32::try_from(new_price)
            .map_err(|err| anyhow::Error::new(err).context("Failed to parse quantity_on_hand"))
            .map_err(RepositoryError::new_validation)?;

        let repo = self.clone();
        tokio::task::spawn_blocking(move || {
            let mut conn = repo.conn()?;
            let updated_product = Product::update_price(product, new_price, &mut conn)?;
            ProductResponse::try_from(updated_product)
        })
        .await?
    }
}
