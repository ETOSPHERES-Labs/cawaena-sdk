pub use api_types::api::networks::{ApiNetwork, ApiProtocol};
// use serde::{Deserialize, Serialize};

// /// Represents the type of the network.
// #[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
// pub enum Protocol {
//     /// Represents an EVM-based network (e.g., Ethereum).
//     /// Contains the URL for the node and the chain ID.
//     Evm {
//         /// chain_id
//         chain_id: u64,
//     },
//     /// Represents an ERC-20 token.
//     /// Contains the contract address and the chain ID.
//     EvmERC20 {
//         /// contract address
//         contract_address: String,
//         /// chain id
//         chain_id: u64,
//     },
//     /// Represents a Stardust network.
//     /// Contains the URL for the node.
//     Stardust {},
// }

// /// Represents a network supported by the wallet.
// #[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
// pub struct Network {
//     /// Internal network identifier
//     pub id: String,
//     /// A user-friendly name for the network.
//     pub name: String,
//     /// The currency used in the network (e.g., "ETH", "IOTA").
//     pub currency: String,
//     /// URL for the network's block explorer.
//     pub block_explorer_url: String,
//     /// Indicates if the network is currently enabled or disabled.
//     pub enabled: bool,
//     /// Optional network identifier
//     pub network_identifier: Option<String>,
//     /// The type of the network (e.g., EVM, Stardust).
//     pub network_type: NetworkType,
// }

// impl From<ApiProtocol> for NetworkType {
//     fn from(value: ApiNetworkType) -> Self {
//         match value {
//             ApiNetworkType::Evm { node_urls, chain_id } => NetworkType::Evm { node_urls, chain_id },
//             ApiNetworkType::Stardust { node_urls } => NetworkType::Stardust { node_urls },
//         }
//     }
// }

// impl From<ApiNetwork> for Network {
//     fn from(value: ApiNetwork) -> Self {
//         Self {
//             id: value.id,
//             name: value.name,
//             currency: value.currency,
//             block_explorer_url: value.block_explorer_url,
//             enabled: value.enabled,
//             network_identifier: value.network_identifier,
//             network_type: value.network_type.into(),
//         }
//     }
// }
