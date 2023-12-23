// @generated automatically by Diesel CLI.

diesel::table! {
    comm (id) {
        id -> Int4,
        uuid -> Text,
        in_body -> Text,
    }
}

diesel::table! {
    todos (id) {
        id -> Int4,
        todotext -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        user_id -> Varchar,
        hashed_user_pw -> Text,
        user_status -> Bool,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    comm,
    todos,
    users,
);
