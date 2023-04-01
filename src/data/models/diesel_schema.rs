// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Binary,
        google_id -> Varchar,
        email -> Varchar,
        first_name -> Varchar,
        last_name -> Varchar,
        password -> Varchar,
        role -> Varchar,
        photo -> Varchar,
        verified -> Bool,
        provider -> Varchar,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}
