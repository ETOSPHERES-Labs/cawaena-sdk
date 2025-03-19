use super::networks::ApiNetwork;
use serde::{Deserialize, Serialize};

/// Get/set system/user address query parameters
#[derive(Debug, Deserialize, Serialize, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema, utoipa::IntoParams))]
pub struct AddressQueryParameters {
    pub network_key: String,
}

/// Get User address request
#[derive(Debug, Default, Deserialize, Serialize, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct SetUserAddressRequest {
    /// User address
    pub address: String,
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
    pub network_key: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct GetPreferredNetworkResponse {
    /// The id of the preferred network, or None if it is not set.
    pub network_key: Option<String>,
}

// Get networks from the backend
#[derive(Debug, Deserialize, Serialize, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ApiGetNetworksResponse {
    pub networks: Vec<ApiNetwork>,
}
