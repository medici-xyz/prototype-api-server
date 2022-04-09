use crate::schema::{auth, orders};
use serde_json::Value as JsonValue;

#[derive(Queryable, Debug, Insertable)]
#[table_name = "orders"]
pub struct Orders {
    pub uuid: String,
    pub signer: String,
    pub collection: String,
    pub price: String,
    pub token_id: String,
    pub amount: String,
    pub end_time: String,
    pub is_order_ask: bool,
    pub signed_msg: String,
    pub makerorder: JsonValue,
    pub active: bool,
}

#[derive(Queryable, Debug, Insertable)]
#[table_name = "auth"]
pub struct Auth {
    pub user_pubkey: String,
    pub user_address: String,
    pub email: Option<String>,
    pub twitter: Option<String>,
}
