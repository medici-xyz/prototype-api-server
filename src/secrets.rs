pub const url: &str = "http://localhost:8000/subgraphs/name/wighawag/eip721-subgraph";
pub const ORDER_TABLE_NAME: &str = "orders";

pub const query: &str = r#"{
	"query": "{tokenContracts(orderBy: numOwners, orderDirection: desc, first: 100) {id\n name\n numTokens\n numOwners\n supportsEIP721Metadata\n }\n }"
}"#;
