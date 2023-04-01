// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Binary,
        google_id -> Varchar,
        provider -> Varchar,
        email -> Varchar,
        first_name -> Varchar,
        last_name -> Varchar,
        photo -> Varchar,
        verified -> Bool,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}
