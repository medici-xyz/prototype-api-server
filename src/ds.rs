use ethereum_types::{Address, Public, Signature, U256};
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

#[derive(Debug, Serialize, Deserialize)]
pub struct UserAuthenticationSetup {
    pub user_pubkey: Public,
    pub user_address: Address,
    pub signature: Signature,
    pub signed_msg: String,
    pub email: Option<String>,
    pub twitter: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LyraJson {
    pub data: Data
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    pub tokenContracts: Vec<TokenContracts>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenContracts {
    pub id: String,
    pub supportsEIP721Metadata: bool,
    pub tokens: Vec<Tokens>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tokens {
    pub tokenID: String,
    pub owner: Owner,
    pub tokenURI: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Owner {
    pub id: String,
}