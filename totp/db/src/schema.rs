// @generated automatically by Diesel CLI.

diesel::table! {
    tokens (id) {
        id -> Int4,
        account_name -> Varchar,
        issuer -> Varchar,
        secret -> Varchar,
    }
}
