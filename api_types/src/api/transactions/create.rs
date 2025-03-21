use super::ApiApplicationMetadata;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateTransactionRequest {
    pub amount: Decimal,
    pub network_key: String,
    pub receiver: String,
    pub application_metadata: ApiApplicationMetadata,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateTransactionResponse {
    pub index: String,
}
