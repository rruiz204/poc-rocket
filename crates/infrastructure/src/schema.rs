// @generated automatically by Diesel CLI.

diesel::table! {
    products (id) {
        id -> Int4,
        name -> Text,
        price -> Numeric,
        created_at -> Timestamp,
    }
}
