table! {
    chats (id) {
        id -> Int4,
        title -> Nullable<Varchar>,
        id_creator -> Nullable<Bpchar>,
        picture -> Nullable<Bytea>,
        is_dialog -> Nullable<Bool>,
    }
}

table! {
    message_content (id) {
        id -> Int4,
        content -> Nullable<Text>,
        type_content -> Nullable<Text>,
    }
}

table! {
    messages (id) {
        id -> Int4,
        chat_id -> Nullable<Int4>,
        sender_id -> Nullable<Bpchar>,
        date_send -> Nullable<Timestamp>,
        content_id -> Nullable<Int4>,
    }
}

table! {
    users (id) {
        id -> Bpchar,
        username -> Nullable<Varchar>,
        password -> Nullable<Varchar>,
        phone -> Nullable<Varchar>,
        email -> Nullable<Varchar>,
        avatar -> Nullable<Bytea>,
    }
}

table! {
    users_chats (user_id, chat_id) {
        user_id -> Bpchar,
        chat_id -> Int4,
    }
}

joinable!(chats -> users (id_creator));
joinable!(messages -> chats (chat_id));
joinable!(messages -> message_content (content_id));
joinable!(messages -> users (sender_id));
joinable!(users_chats -> chats (chat_id));
joinable!(users_chats -> users (user_id));

allow_tables_to_appear_in_same_query!(
    chats,
    message_content,
    messages,
    users,
    users_chats,
);
