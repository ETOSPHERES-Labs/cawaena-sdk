use serde::{Deserialize, Serialize};

/// Supported crypto currencies
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum ApiCryptoCurrency {
    /// Iota Crypto Currency
    Iota,
    /// Ethereum Crypto Currency
    Eth,
}

/// Struct for storing the commit hash from different services
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct HelloResponse {
    pub message: String,
    pub account_hash: String,
    pub dlt_hash: String,
    pub kyc_hash: String,
    pub postident_hash: String,
    pub transaction_hash: String,
    pub viviswap_hash: String,
    pub user_data_hash: String,
    pub webhook_hash: String,
    pub requests_aggregator_hash: String,
}
