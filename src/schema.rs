// @generated automatically by Diesel CLI.

diesel::table! {
    items (id) {
        id -> Int4,
        #[max_length = 255]
        item_name -> Varchar,
    }
}
