use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum ApiProtocol {
    Evm { chain_id: u64 },
    EvmERC20 { chain_id: u64, contract_address: String },
    Stardust {},
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ApiNetwork {
    pub key: String,
    pub display_name: String,
    pub display_symbol: String,
    pub coin_type: u32,
    pub node_urls: Vec<String>,
    pub decimals: u32,
    pub can_do_purchases: bool,
    pub protocol: ApiProtocol,
    pub block_explorer_url: String,
}
