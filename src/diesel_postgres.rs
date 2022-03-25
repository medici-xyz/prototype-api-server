use diesel::prelude::*;
use rocket::fairing::AdHoc;
use rocket::serde::json::Json;
use rocket::{Build, Rocket};
use rocket_sync_db_pools::database;
use serde_json::{to_string, to_value, Value as JsonValue};

use crate::ds::MakeOrderStorageStruct;
use crate::models::Orders;
use crate::schema::orders::dsl::*;

#[database("orders_db")]
pub struct OrdersDb(diesel::PgConnection);

#[get("/")]
pub async fn main_route(conn: OrdersDb) {}

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

#[post("/makeorder", format = "json", data = "<order>")]
async fn makeorder(conn: OrdersDb, order: Json<MakeOrderStorageStruct>) {
    use crate::schema::orders;

    let order_struct = order.into_inner();
    let new_uuid = ::uuid::Uuid::new_v4().to_simple().to_string();

    conn.run(move |c| {
        let new_order = Orders {
            uuid: new_uuid,
            signer: order_struct.order_data.signer.clone(),
            collection: order_struct.order_data.collection.clone(),
            price: order_struct.order_data.price.to_string(),
            token_id: order_struct.order_data.token_id.to_string(),
            amount: order_struct.order_data.amount.to_string(),
            end_time: order_struct.order_data.end_time.to_string(),
            is_order_ask: order_struct.order_data.is_order_ask,
            signed_msg: order_struct.signed_msg.to_string(),
            active: true,
            makerorder: to_value(&order_struct.order_data).unwrap(),
        };
        diesel::insert_into(orders::table)
            .values(new_order)
            .execute(c)
            .expect("inserting into table failed");
    })
    .await;
}
#[get("/vieworders/<collection_id>/<token>")]
async fn view_orders(conn: OrdersDb, collection_id: String, token: String) -> String {
    let return_data = conn
        .run(|c| {
            let all_orders = orders
                .filter(collection.eq(collection_id))
                .filter(token_id.eq(token))
                .filter(active.eq(true))
                .load::<Orders>(c)
                .expect("fetching orders failed");
            let mut relevant_fields = Vec::new();

            for order in all_orders {
                relevant_fields.push((order.signed_msg, order.makerorder))
            }

            relevant_fields
        })
        .await;

    to_string(&return_data).expect("error in converting fetched data to JSON")
}

#[get("/viewallorders")]
async fn view_all_orders(conn: OrdersDb) -> String {
    let return_data = conn
        .run(|c| {
            let all_orders = orders.load::<Orders>(c).expect("fetching orders failed");
            let mut relevant_fields = Vec::new();

            for order in all_orders {
                relevant_fields.push((order.signed_msg, order.makerorder))
            }

            relevant_fields
        })
        .await;

    to_string(&return_data).expect("error in converting fetched data to JSON")
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Diesel Postgres Stage", |rocket| async {
        rocket
            .attach(OrdersDb::fairing())
            .attach(AdHoc::on_ignite("Diesel Migrations", run_migrations))
            .mount("/", routes![makeorder, view_all_orders, view_orders])
    })
}
