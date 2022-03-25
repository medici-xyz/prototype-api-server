use ethereum_types::{Signature, U256};
use serde::{Deserialize, Serialize};

type EthAddress = String;

#[derive(Debug, Serialize, Deserialize)]
pub struct MakeOrderStorageStruct {
    pub signed_msg: Signature,
    pub order_data: MakeOrderContractDataStruct,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MakeOrderContractDataStruct {
    pub is_order_ask: bool,
    pub signer: EthAddress,
    pub collection: EthAddress,
    pub price: U256,
    pub token_id: U256,
    pub amount: U256,
    pub strategy: EthAddress,
    pub currency: EthAddress,
    pub nonce: U256,
    pub start_time: U256,
    pub end_time: U256,
    pub min_percentage_to_ask: U256,
    pub params: String,
}
