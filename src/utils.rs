use serde_json::from_str as from_json;
use serde_json::Value;
use crate::ds::{Data, LyraJson, TokenContracts};

pub fn filter_collections_for_lyra_mints(json_str: String) -> String {
    let mut parsed: LyraJson = serde_json::from_str(&json_str).unwrap();

    let mut lyra_collections = Vec::new();

    for collection in parsed.data.tokenContracts {
        let token = &collection.tokens[0];
        if token.tokenURI.contains("https://gateway.pinata.cloud/ipfs/") {
            let mut new_token = token.clone();
            new_token.tokenURI = token.tokenURI.replace("https://gateway.pinata.cloud/", "https://medici-test.mypinata.cloud/");
            let new_collection = TokenContracts {
                id: collection.id,
                supportsEIP721Metadata: collection.supportsEIP721Metadata,
                tokens: vec![new_token]
            };
            lyra_collections.push(new_collection);
        }
    }

    let filtered = LyraJson {
        data: Data {
            tokenContracts: lyra_collections
        }
    };

    serde_json::to_string_pretty(&filtered).unwrap()
}