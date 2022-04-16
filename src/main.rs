#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

mod cors;
mod diesel_postgres;
mod ds;
mod error_logging;
mod models;
mod schema;
mod secrets;
mod utils;

use reqwest::Client as reqwestClient;
use rocket::routes;
use tokio::sync::mpsc;
use rocket::http::Status;

use crate::cors::Cors;
use crate::error_logging::throw_json_error;
use crate::secrets::{query, url, lyraquery};
use crate::utils::filter_collections_for_lyra_mints;

async fn make_post_request(query_string: String, mut origin: Vec<&str>) -> Result<String, String> {
    let client = reqwestClient::new();
    origin.push("make_post_reqwest");

    let res = client
        .post(url)
        .body(query_string)
        .send()
        .await
        .map_err(|_| {
            throw_json_error(
                "reqwest",
                &origin,
                "main",
                "28",
                "failed to send POST request to graphql indexer",
            )
        })?;
    res.text().await.map_err(|_| {
        throw_json_error(
            "reqwest",
            &origin,
            "main",
            "42",
            "failed to extract text from graphql indexer response",
        )
    })
}

#[get("/collections")]
async fn collections() -> Result<String, String> {
    Ok(make_post_request(query.to_string(), vec!["collections"]).await?)
}

#[get("/collection/<name>")]
async fn collection(name: String) -> Result<String, String> {
    let collection_query = format!("{{\n\"query\": \"{{tokenContract(id: \\\"{}\\\") {{id name numTokens numOwners tokens(orderBy:mintTime,orderDirection: asc){{ id tokenURI tokenID mintTime owner {{ id }}}}}}}}\"}}", name);
    Ok(make_post_request(collection_query, vec!["collection"]).await?)
}

#[get("/lyracollections")]
async fn lyracollections() -> Result<String, String> {
    let response = make_post_request(lyraquery.to_string(), vec!["lyracollections"]).await?;
    Ok(filter_collections_for_lyra_mints(response))
}

#[get("/health-check")]
async fn health_check() -> Status {
    Status::Ok
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Cors)
        .mount("/", routes![collections, collection, lyracollections, health_check])
        .attach(diesel_postgres::stage())
}
