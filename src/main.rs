#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

mod cors;
mod diesel_postgres;
mod ds;
mod models;
mod schema;
mod secrets;

use postgres::Client as PostgresClient;
use reqwest::Client as reqwestClient;
use rocket::routes;
use rocket_sync_db_pools::database;

use crate::cors::Cors;
use crate::secrets::{query, url};

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

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Cors)
        .mount("/", routes![collections, collection])
        .attach(diesel_postgres::stage())
}
