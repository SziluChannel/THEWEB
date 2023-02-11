// @generated automatically by Diesel CLI.

diesel::table! {
    chat_connector (id) {
        id -> Int4,
        user_id -> Uuid,
        chat_id -> Uuid,
        joined -> Timestamp,
    }
}

diesel::table! {
    chats (id) {
        id -> Uuid,
        name -> Varchar,
        created -> Timestamp,
    }
}

diesel::table! {
    messages (id) {
        id -> Int4,
        user_id -> Uuid,
        chat_id -> Uuid,
        content -> Text,
        created -> Timestamp,
    }
}

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

diesel::joinable!(chat_connector -> chats (chat_id));
diesel::joinable!(chat_connector -> users (user_id));
diesel::joinable!(messages -> chats (chat_id));
diesel::joinable!(messages -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    chat_connector,
    chats,
    messages,
    users,
);
