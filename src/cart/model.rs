use crate::product::model::Product;
use crate::{
    errors::ApiError,
    schema::{cart, cart_item},
};
use chrono::{DateTime, Utc};
use diesel::{insert_into, prelude::*};
use diesel::{Identifiable, Insertable, PgConnection, QueryDsl};
use serde::Deserialize;
#[derive(Identifiable, Deserialize, Selectable, Queryable)]
#[diesel(table_name = cart)]
struct Cart {
    id: uuid::Uuid,
}

impl Cart {
    fn new_cart(conn: &mut PgConnection) -> Result<Cart, diesel::result::Error> {
        insert_into(cart::table)
            .default_values()
            .get_result::<Cart>(conn)
    }
    fn remove_cart_item(
        cart_item: uuid::Uuid,
        conn: &mut PgConnection,
    ) -> Result<(), diesel::result::Error> {
        conn.transaction(|conn| {
            let (cart_quantity, product_id) = cart_item::table
                .find(cart_item)
                .select((cart_item::quantity, cart_item::product_id))
                .first::<(i32, uuid::Uuid)>(conn)?;

            Product::increment_quantity_available(product_id, cart_quantity, conn)?;

            diesel::delete(cart_item::table.find(cart_item))
                .execute(conn)
                .map(|_| ())
        })
    }

    fn add_cart_item(
        cart_item: CartItemInput,
        conn: &mut PgConnection,
    ) -> Result<CartItem, ApiError> {
        let product_available_quantity =
            Product::find_product_quantity_available_by_id(cart_item.product_id, conn)?;
        if product_available_quantity < cart_item.quantity {
            return Err(ApiError::new_validation(
                "Quantity requested is not available",
            ));
        }
        cart_item
            .insert_into(cart_item::table)
            .get_result::<CartItem>(conn)
            .map_err(ApiError::from)
    }
}

#[derive(Identifiable, Insertable)]
#[diesel(table_name = cart_item)]
pub struct CartItemInput {
    id: uuid::Uuid,
    cart_id: uuid::Uuid,
    product_id: uuid::Uuid,
    quantity: i32,
}

#[derive(Identifiable, Insertable, Queryable)]
#[diesel(table_name = cart_item)]
pub struct CartItem {
    id: uuid::Uuid,
    cart_id: uuid::Uuid,
    product_id: uuid::Uuid,
    quantity: i32,
    added_at: DateTime<Utc>,
}

#[derive(AsChangeset, Identifiable, Copy, Clone)]
#[diesel(table_name = cart_item)]
struct CartItemQuantityUpdate {
    id: uuid::Uuid,
    quantity: i32,
}

impl CartItem {
    fn update_quantity(
        update: CartItemQuantityUpdate,
        conn: &mut PgConnection,
    ) -> Result<Self, ApiError> {
        use crate::schema::product;
        use crate::schema::product::dsl::*;

        let (product_quantity_available, existing_cart_item_quantity) = cart_item::table
            .find(update.id)
            .inner_join(product)
            .select((product::quantity_available, cart_item::quantity))
            .first::<(i32, i32)>(conn)?;
        let overall_quantity_available = product_quantity_available + existing_cart_item_quantity;
        if update.quantity > overall_quantity_available {
            return Err(ApiError::new_validation(
                "The quantity requested is not available",
            ));
        }
        update
            .save_changes::<CartItem>(conn)
            .map_err(ApiError::from)
    }
}

pub struct CartItemModel {
    pub id: uuid::Uuid,
    pub quantity: u32,
    pub product_id: uuid::Uuid,
    pub added_at: DateTime<Utc>,
}

pub struct CartModel {
    pub id: uuid::Uuid,
    pub cart_items: Vec<CartItemModel>,
}

impl CartModel {
    pub fn create_cart(conn: &mut PgConnection) -> Result<CartModel, diesel::result::Error> {
        let cart = Cart::new_cart(conn)?;
        Ok(CartModel {
            id: cart.id,
            cart_items: Default::default(),
        })
    }
}
