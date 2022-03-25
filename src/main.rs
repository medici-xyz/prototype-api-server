#[macro_use]
extern crate rocket;

mod cors;
mod data_structures;
mod db;
mod secrets;

use data_structures::MakeOrderStorageStruct;
use db::{add_collection_to_table, fetch_orders_from_table, fetch_all_orders_from_table};
use postgres::Client as PostgresClient;
use reqwest::Client as reqwestClient;
use rocket::routes;
use rocket::serde::{json::Json, Deserialize};
use rocket_sync_db_pools::database;

use crate::cors::Cors;
use crate::secrets::{query, url};

#[database("orders_db")]
struct OrdersDBConn(PostgresClient);

async fn make_post_request(query_string: String) -> String {
    let client = reqwestClient::new();

    let res = client.post(url).body(query_string).send().await.unwrap();
    res.text().await.unwrap()
}

#[get("/collections")]
async fn collections() -> String {
    make_post_request(query.to_string()).await
}

#[get("/collection/<name>")]
async fn collection(name: String) -> String {
    let collection_query = format!("{{\n\"query\": \"{{tokenContract(id: \\\"{}\\\") {{id name numTokens numOwners tokens(orderBy:mintTime,orderDirection: asc){{ id tokenURI tokenID mintTime owner {{ id }}}}}}}}\"}}", name);
    make_post_request(collection_query).await
}

#[post("/makeorder", format = "json", data = "<order>")]
async fn makeorder(conn: OrdersDBConn, order: Json<MakeOrderStorageStruct>) {
    let fields_string = vec![
        "signer",
        "collection",
        "price",
        "token_id",
        "end_time",
        "is_order_ask",
        "signed_msg",
        "makeorder_struct",
    ];
    conn.run(|c| add_collection_to_table(c, fields_string, order.into_inner())).await.unwrap();
}

#[get("/vieworders/<collection>/<token>")]
async fn view_orders(conn: OrdersDBConn, collection: String, token: String) {
    conn.run(|c| fetch_orders_from_table(c, collection, token)).await.unwrap();
}

#[get("/viewallorders")]
async fn view_all_orders(conn: OrdersDBConn) {
    conn.run(|c| fetch_all_orders_from_table(c)).await.unwrap();
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    rocket::build()
        .attach(Cors)
        .attach(OrdersDBConn::fairing())
        .mount("/", routes![collections, collection, makeorder, view_orders, view_all_orders])
        .launch()
        .await
}
