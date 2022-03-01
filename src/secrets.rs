pub const url: &str = "http://localhost:8000/subgraphs/name/wighawag/eip721-subgraph";

// pub const query: &str = r#"
// {
//     "query": "{
//         tokenContract(orderBy: numOwners, orderDirection: desc, first: 100) {
//             id
//             name
//             numTokens
//             numOwners
//             supportsEIP721Metadata
//         }
//     }"
// }
// "#;

// pub const query: &str = "{\"query\": \"{\ntokenContracts(orderBy: numOwners, orderDirection: desc, first: 100) {\n    id\n    name\n    numTokens\n    numOwners\n    supportsEIP721Metadata\n  }\n}\"}";

pub const query: &str = r#"{
	"query": "{tokenContracts(orderBy: numOwners, orderDirection: desc, first: 100) {id\n name\n numTokens\n numOwners\n supportsEIP721Metadata\n }\n }"
}"#;
