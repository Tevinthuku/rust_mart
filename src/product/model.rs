use crate::schema::product;

use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::Deserialize;
use serde::Serialize;

#[derive(Queryable, Selectable, AsChangeset, Identifiable, Deserialize, Serialize)]
#[diesel(table_name = product)]
pub struct Product {
    pub id: uuid::Uuid,
    pub name: String,
    pub description: String,
    pub quantity_on_hand: i32,
    pub quantity_available: i32,
    pub price: Option<i32>,
    pub created_at: DateTime<Utc>,
}

#[derive(Deserialize, Insertable, Clone)]
#[diesel(table_name = product)]
pub struct ProductInput {
    name: String,
    description: String,
    quantity_on_hand: i32,
}

impl Product {
    pub fn insert(
        input: ProductInput,
        conn: &mut PgConnection,
    ) -> Result<Product, diesel::result::Error> {
        input
            .insert_into(product::table)
            .get_result::<Product>(conn)
    }

    pub fn update_price(
        product_id: uuid::Uuid,
        new_price: i32,
        conn: &mut PgConnection,
    ) -> Result<Product, diesel::result::Error> {
        diesel::update(product::table.find(product_id))
            .set(product::price.eq(Some(new_price)))
            .get_result(conn)
    }

    pub fn find_product_quantity_available_by_id(
        id: uuid::Uuid,
        conn: &mut PgConnection,
    ) -> Result<i32, diesel::result::Error> {
        product::table
            .select(product::quantity_available)
            .find(id)
            .first(conn)
    }

    pub fn increment_quantity_available(
        id: uuid::Uuid,
        increment: i32,
        conn: &mut PgConnection,
    ) -> Result<(), diesel::result::Error> {
        diesel::update(product::table.find(id))
            .set(product::quantity_available.eq(product::quantity_available + increment))
            .execute(conn)
            .map(|_| ())
    }
}
