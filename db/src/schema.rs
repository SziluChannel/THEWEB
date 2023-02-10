// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Uuid,
        name -> Varchar,
        email -> Varchar,
        password -> Varchar,
        admin -> Bool,
        confirmation_token -> Varchar,
        confirmed -> Bool,
    }
}
