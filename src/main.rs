#[macro_use]
extern crate rocket;

mod cors;
mod secrets;

use reqwest::Client as reqwestClient;

use crate::cors::Cors;
use crate::secrets::{query, query2, url};

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

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    rocket::build()
        .attach(Cors)
        .mount("/", routes![collections, collection])
        .launch()
        .await
}
