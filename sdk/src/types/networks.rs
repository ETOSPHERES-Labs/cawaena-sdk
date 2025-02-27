use api_types::api::networks::{ApiNetwork, ApiNetworkType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub enum NetworkType {
    Evm { node_url: String, chain_id: u64 },
    Stardust { node_url: String },
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Network {
    pub id: String,
    pub name: String,
    pub currency: String,
    pub block_explorer_url: String,
    pub enabled: bool,
    pub network_identifier: Option<String>,
    pub network_type: NetworkType,
}

impl From<ApiNetworkType> for NetworkType {
    fn from(value: ApiNetworkType) -> Self {
        match value {
            ApiNetworkType::Evm { node_url, chain_id } => NetworkType::Evm { node_url, chain_id },
            ApiNetworkType::Stardust { node_url } => NetworkType::Stardust { node_url },
        }
    }
}

impl From<ApiNetwork> for Network {
    fn from(value: ApiNetwork) -> Self {
        Self {
            id: value.id,
            name: value.name,
            currency: value.currency,
            block_explorer_url: value.block_explorer_url,
            enabled: value.enabled,
            network_identifier: value.network_identifier,
            network_type: value.network_type.into(),
        }
    }
}
