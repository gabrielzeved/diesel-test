// @generated automatically by Diesel CLI.

diesel::table! {
    key_pairs (id) {
        id -> Int4,
        key -> Varchar,
        token -> Varchar,
        name -> Varchar,
        permission -> Int4,
    }
}
