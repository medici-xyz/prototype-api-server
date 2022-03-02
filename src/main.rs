#[macro_use]
extern crate rocket;

mod cors;
mod secrets;

use reqwest::Client as reqwestClient;

use crate::cors::Cors;
use crate::secrets::{query, url};

async fn make_post_request() -> String {
    let client = reqwestClient::new();

    let res = client.post(url).body(query).send().await.unwrap();
    res.text().await.unwrap()
}

#[get("/collections")]
async fn collections() -> String {
    make_post_request().await
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    rocket::build()
        .attach(Cors)
        .mount("/", routes![collections])
        .launch()
        .await
}
