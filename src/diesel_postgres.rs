use diesel::prelude::*;
use ethsign::{PublicKey, Signature};
use rocket::fairing::AdHoc;
use rocket::serde::json::Json;
use rocket::{Build, Rocket};
use rocket_sync_db_pools::database;
use serde_json::{to_string, to_value};

use crate::ds::{MakeOrderStorageStruct, UserAuthenticationSetup};
use crate::error_logging::throw_json_error;
use crate::models::{Auth, Orders};
use crate::schema::auth::dsl::*;
use crate::schema::orders::dsl::*;
use web3::signing::{keccak256, recover};

#[database("orders_db")]
pub struct OrdersDb(diesel::PgConnection);

async fn run_migrations(rocket: Rocket<Build>) -> Rocket<Build> {
    // This macro from `diesel_migrations` defines an `embedded_migrations`
    // module containing a function named `run` that runs the migrations in the
    // specified directory, initializing the database.
    embed_migrations!("migrations");

    let conn = OrdersDb::get_one(&rocket)
        .await
        .expect("database connection");
    conn.run(|c| embedded_migrations::run_with_output(c, &mut std::io::stdout()))
        .await
        .expect("diesel migrations");

    rocket
}

pub fn eth_message(message: String) -> [u8; 32] {
    keccak256(
        format!(
            "{}{}{}",
            "\x19Ethereum Signed Message:\n",
            message.len(),
            message
        )
        .as_bytes(),
    )
}

#[post("/auth", format = "json", data = "<authdata>")]
async fn register_user(
    conn: OrdersDb,
    authdata: Json<UserAuthenticationSetup>,
) -> Result<(), String> {
    use crate::schema::auth;

    let authdata = authdata.into_inner();

    // TODO: verify that the address calculated from the pubkey is the same
    // as the one passed through - otherwise throw an error
    // let address = public_key.address();

    let signature = hex::decode(authdata.signature).map_err(|_| {
        throw_json_error(
            "hex",
            &vec!["register_user"],
            "diesel_postgres",
            "60",
            "failed to decode signature into hexcode",
        )
    })?;
    let message = eth_message(authdata.signed_msg);
    let pubkey = recover(&message, &signature[..64], 0).map_err(|_| {
        throw_json_error(
            "web3",
            &vec!["register_user"],
            "diesel_postgres",
            "70",
            "failed to recover public key from message and signature",
        )
    });
    let pubkey = format!("{:02X?}", pubkey);

    if pubkey != authdata.user_pubkey.to_string() {
        return Err(throw_json_error(
            "_",
            &vec!["register_user"],
            "diesel_postgres",
            "78",
            "public key given is not the same as the extracted public key",
        ));
    }

    let postgres_write: Result<(), String> = conn.run(move |c| {
        let results: Vec<String> = auth
            .select(user_address)
            .filter(user_address.eq(authdata.user_address.to_string()))
            .load::<String>(c)
            .map_err(|_| {
                throw_json_error(
                    "Diesel",
                    &vec!["register_user"],
                    "diesel_postgres",
                    "90",
                    "Failed to query select statement from Postgres",
                )
            })?;

        if results.len() == 0 {
            let new_login = Auth {
                user_pubkey: authdata.user_pubkey.to_string(),
                user_address: authdata.user_address.to_string(),
                email: authdata.email.clone(),
                twitter: authdata.twitter.clone(),
            };
            diesel::insert_into(auth::table)
                .values(new_login)
                .execute(c)
                .map_err(|_| {
                    throw_json_error(
                        "Diesel",
                        &vec!["register_user"],
                        "diesel_postgres",
                        "90",
                        "Failed to insert user to table in Postgres",
                    )
                })?;
        }
        else {
        // TODO: return saying that user has already been registered
        }
    })?.await?;

    postgres_write
    // TODO: should we return something?
}

#[post("/makeorder", format = "json", data = "<order>")]
async fn makeorder(conn: OrdersDb, order: Json<MakeOrderStorageStruct>) -> Result<(), String> {
    use crate::schema::orders;

    let order_struct = order.into_inner();
    let new_uuid = ::uuid::Uuid::new_v4().to_simple().to_string();

    let postgres_fetch: Result<(), String> = conn.run(move |c| {
        let new_order = Orders {
            uuid: new_uuid,
            signer: order_struct.order_data.signer.clone(),
            collection: order_struct.order_data.collection.clone(),
            price: order_struct.order_data.price.to_string(),
            token_id: order_struct.order_data.token_id.to_string(),
            amount: order_struct.order_data.amount.to_string(),
            end_time: order_struct.order_data.end_time.to_string(),
            is_order_ask: order_struct.order_data.is_order_ask,
            signed_msg: format!("{:#?}", order_struct.signed_msg),
            active: true,
            makerorder: to_value(&order_struct.order_data).map_err(|_| {
                throw_json_error(
                    "serde",
                    &vec!["makeorder"],
                    "diesel_postgres",
                    "153",
                    "failed to deserialize given JSON",
                )
            })?,
        };
        diesel::insert_into(orders::table)
            .values(new_order)
            .execute(c)
            .map_err(|c| {
                throw_json_error(
                    "Diesel",
                    &vec!["makeorder"],
                    "diesel_postgres",
                    "163",
                    "failed to insert makeorder into table",
                )
            })?;
    })
    .await?;

    postgres_fetch
}

#[get("/vieworders/<collection_id>/<token>")]
async fn view_orders(
    conn: OrdersDb,
    collection_id: String,
    token: String,
) -> Result<String, String> {
    let return_data = conn
        .run(|c| {
            let all_orders = orders
                .filter(collection.eq(collection_id))
                .filter(token_id.eq(token))
                .filter(active.eq(true))
                .load::<Orders>(c)
                .map_err(|c| {
                    throw_json_error(
                        "Diesel",
                        &vec![format!("vieworders/{}/{}", collection_id, token).as_str()],
                        "diesel_postgres",
                        "187",
                        "failed to fetch orders from Postgres",
                    )
                })?;
            let mut relevant_fields = Vec::new();

            for order in all_orders {
                relevant_fields.push((order.signed_msg, order.makerorder))
            }

            relevant_fields
        })
        .await?;

    to_string(&return_data).map_err(|c| {
        throw_json_error(
            "Serde",
            &vec![format!("vieworders/{}/{}", collection_id, token).as_str()],
            "diesel_postgres",
            "206",
            "error in converting fetched data to JSON",
        )
    })?;
}

#[get("/viewallorders")]
async fn view_all_orders(conn: OrdersDb) -> String {
    let return_data = conn
        .run(|c| {
            let all_orders = orders.load::<Orders>(c).map_err(|c| {
                    throw_json_error(
                        "Diesel",
                        &vec!["view_all_orders"],
                        "diesel_postgres",
                        "229",
                        "failed to fetch orders from Postgres",
                    )
                })?;
            let mut relevant_fields = Vec::new();

            for order in all_orders {
                relevant_fields.push((order.signed_msg, order.makerorder))
            }

            relevant_fields
        })
        .await;

    to_string(&return_data).map_err(|c| {
        throw_json_error(
            "Serde",
            &vec!["view_all_orders"],
            "diesel_postgres",
            "240",
            "error in converting fetched data to JSON",
        )
    })?;

}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Diesel Postgres Stage", |rocket| async {
        rocket
            .attach(OrdersDb::fairing())
            .attach(AdHoc::on_ignite("Diesel Migrations", run_migrations))
            .mount(
                "/",
                routes![makeorder]
            )
    })
}
