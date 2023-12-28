// @generated automatically by Diesel CLI.

diesel::table! {
    cratess (id) {
        id -> Int4,
        rustacean_id -> Int4,
        #[max_length = 64]
        code -> Varchar,
        #[max_length = 128]
        name -> Varchar,
        #[max_length = 64]
        version -> Varchar,
        description -> Nullable<Text>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    rustaceans (id) {
        id -> Int4,
        name -> Varchar,
        email -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::joinable!(cratess -> rustaceans (rustacean_id));

diesel::allow_tables_to_appear_in_same_query!(
    cratess,
    rustaceans,
);
