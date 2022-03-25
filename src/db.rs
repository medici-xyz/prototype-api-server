use itertools::Itertools;
use postgres::{Client as PostgresClient, error::Error as PostgresError};
use rocket::response::Responder;
use serde_json::to_string as to_json;
use uuid::Uuid;

use crate::{data_structures::MakeOrderStorageStruct, secrets::{ORDER_TABLE_NAME}};

pub fn add_order_table_if_not_exists(
    client: &mut PostgresClient
) -> Result<(), PostgresError> {

    let query_string = format!(
        "CREATE TABLE IF NOT EXISTS {} (
            uuid TEXT PRIMARY KEY,
            signer TEXT NOT NULL,
            collection TEXT NOT NULL,
            price TEXT NOT NULL,
            token_id TEXT NOT NULL,
            end_time TEXT NOT NULL,
            is_order_ask TEXT NOT NULL,
            signed_msg TEXT NOT NULL
            makerorder JSON NOT NULL,
        )",
        ORDER_TABLE_NAME
    );

    client.execute(query_string.as_str(), &[])?;

    Ok(())
}

pub fn add_collection_to_table(
    client: &mut PostgresClient,
    fields: Vec<&str>,
    make_order: MakeOrderStorageStruct,
) -> Result<(), PostgresError> {
    
    add_order_table_if_not_exists(client)?;

    let fields_str = fields.iter().join(",");

    let query_string = format!(
        "INSERT INTO {} ({})
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)",
        ORDER_TABLE_NAME, fields_str
    );

    dbg!(&query_string);

    let uuid = Uuid::new_v4().to_simple().to_string();

    client.execute(
        query_string.as_str(),
        &[
            &uuid,
            &make_order.order_data.signer,
            &make_order.order_data.collection,
            &make_order.order_data.price.to_string(),
            &make_order.order_data.token_id.to_string(),
            &make_order.order_data.end_time.to_string(),
            &make_order.order_data.is_order_ask,
            &make_order.signed_msg.to_string(),
            &to_json(&make_order.order_data).unwrap()
        ],
    )?;
    Ok(())
}

pub fn fetch_orders_from_table(
    client: &mut PostgresClient,
    collection: String,
    token: String
) -> Result<String, PostgresError> {
    
    let query_string = format!(
        "
        SELECT
            signer,
            colletion,
            price,
            token_id,
            end_time,
            is_order_ask,
            signed_msg,
            makeorder_struct
        FROM
            {}
        WHERE
            collection=\"{}\" AND
            token_id=\"{}\";
        ",
        ORDER_TABLE_NAME,
        collection,
        token
    );
    client.execute(&query_string, &[])?;

    Ok("".to_string())
}

pub fn fetch_all_orders_from_table(
    client: &mut PostgresClient,
) -> Result<String, PostgresError> {
    
    let query_string = format!(
        "
        SELECT row_to_json(t)
        FROM (
            SELECT
                signer,
                colletion,
                price,
                token_id,
                end_time,
                is_order_ask,
                signed_msg,
                makeorder_struct
            FROM
                {}
        ) t
        ",
        ORDER_TABLE_NAME,
    );
    let result = client.query(&query_string, &[])?;
    dbg!(result);

    Ok("".to_string())
}