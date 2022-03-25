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
