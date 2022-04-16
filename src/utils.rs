use serde_json::from_str as from_json;
use serde_json::Value;
use crate::ds::{Data, LyraJson};

pub fn filter_collections_for_lyra_mints(json_str: String) -> String {
    let mut parsed: LyraJson = serde_json::from_str(&json_str).unwrap();

    let mut lyra_collections = Vec::new();

    for collection in parsed.data.tokenContracts {
        let token = &collection.tokens[0];
        if token.tokenURI.contains("https://gateway.pinata.cloud/ipfs/") {
            lyra_collections.push(collection);
        }
    }

    let filtered = LyraJson {
        data: Data {
            tokenContracts: lyra_collections
        }
    };

    serde_json::to_string_pretty(&filtered).unwrap()
}