// @generated automatically by Diesel CLI.

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
