use serde::Serialize;

#[derive(Serialize)]
pub struct Product {
    pub(crate) id: uuid::Uuid,
    // maybe use the newtype instead of a string;
    pub(crate) sku: String,
    pub(crate) name: String,
    pub(crate) description: String,
}
