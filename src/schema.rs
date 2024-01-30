// @generated automatically by Diesel CLI.

diesel::table! {
    accounts (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        type_of -> Text,
        starting_balance -> Int4,
        current_balance -> Int4,
    }
}

diesel::table! {
    transactions (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        payee -> Text,
        inflow -> Int4,
        outflow -> Int4,
        notes -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    accounts,
    transactions,
);
