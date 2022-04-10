table! {
    auth (user_pubkey) {
        user_pubkey -> Varchar,
        user_address -> Varchar,
        email -> Nullable<Text>,
        twitter -> Nullable<Text>,
    }
}

table! {
    orders (uuid) {
        uuid -> Varchar,
        signer -> Text,
        collection -> Text,
        price -> Text,
        token_id -> Text,
        amount -> Text,
        end_time -> Text,
        is_order_ask -> Bool,
        signed_msg -> Text,
        makerorder -> Json,
        active -> Bool,
    }
}

allow_tables_to_appear_in_same_query!(auth, orders,);
