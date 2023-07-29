// @generated automatically by Diesel CLI.

diesel::table! {
    cart (id) {
        id -> Uuid,
    }
}

diesel::table! {
    cart_item (id) {
        id -> Uuid,
        cart_id -> Uuid,
        product_id -> Uuid,
        quantity -> Int4,
        added_at -> Timestamptz,
    }
}

diesel::table! {
    product (id) {
        id -> Uuid,
        name -> Text,
        description -> Text,
        quantity_on_hand -> Int4,
        quantity_available -> Int4,
        price -> Nullable<Int4>,
        created_at -> Timestamptz,
    }
}

diesel::joinable!(cart_item -> cart (cart_id));
diesel::joinable!(cart_item -> product (product_id));

diesel::allow_tables_to_appear_in_same_query!(
    cart,
    cart_item,
    product,
);
