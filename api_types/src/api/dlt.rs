use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::networks::ApiNetwork;

/// Get/set system/user address query parameters
#[derive(Debug, Deserialize, Serialize, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema, utoipa::IntoParams))]
pub struct AddressQueryParameters {
    pub network_id: String,
}

/// Get User address request
#[derive(Debug, Default, Deserialize, Serialize, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct SetUserAddressRequest {
    /// User address
    pub address: String,
    /// Network
    pub network_id: String,
}

/// Get user address response
#[derive(Default, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct GetUserAddressResponse {
    /// User address
    pub address: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct SetPreferredNetworkRequest {
    /// The id of the preferred network, or None if it should be cleared.
    pub network_id: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct GetPreferredNetworkResponse {
    /// The id of the preferred network, or None if it is not set.
    pub network_id: Option<String>,
}

// Struct to get node urls from backend
#[derive(Debug, Deserialize, Serialize, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ApiGetNodeUrlsResponse {
    pub node_urls: HashMap<String, Vec<String>>,
}

// Struct to get networks from backend +
#[derive(Debug, Deserialize, Serialize, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ApiGetNetworksResponse {
    pub networks: Vec<ApiNetwork>,
}
