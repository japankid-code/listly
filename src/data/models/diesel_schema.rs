// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Varchar,
        google_id -> Varchar,
        provider -> Varchar,
        email -> Varchar,
        first_name -> Varchar,
        last_name -> Varchar,
        photo -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
