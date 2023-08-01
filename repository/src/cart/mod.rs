use crate::{cart::model::CartModel, errors::RepositoryError, Repository};
use serde::Serialize;

mod model;

#[derive(Serialize)]
pub struct CartResponse {
    id: uuid::Uuid,
    cart_items: Vec<CartItemResponse>,
}

#[derive(Serialize)]
pub struct CartItemResponse {
    id: uuid::Uuid,
    product_id: uuid::Uuid,
    quantity: u32,
}

impl Repository {
    pub async fn create_cart(&self) -> Result<CartResponse, RepositoryError> {
        let repo = self.clone();
        tokio::task::spawn_blocking(move || {
            let mut conn = repo.conn()?;
            let cart = CartModel::create_cart(&mut conn)?;
            Ok(CartResponse {
                id: cart.id,
                cart_items: Default::default(),
            })
        })
        .await?
    }
}
