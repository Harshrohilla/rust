// @generated automatically by Diesel CLI.

diesel::table! {
    rust_user (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar,
        password_hash -> Varchar,
    }
}
