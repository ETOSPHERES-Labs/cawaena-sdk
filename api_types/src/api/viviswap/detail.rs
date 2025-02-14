use serde::{Deserialize, Serialize};

// data objects

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct PaymentDetail {
    pub id: String,
    pub address: String,
    pub is_verified: Option<bool>,
}

/// The key used to identify the type of PaymentDetail we are trying to operate on
#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(rename_all = "UPPERCASE")]
pub enum SwapPaymentDetailKey {
    Sepa,
    Iota,
    Eth,
}

// request/response objects

// get details

#[derive(Debug, Deserialize, Serialize, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema, utoipa::IntoParams))]
pub struct GetPaymentDetailsRequestQueries {
    pub payment_method_key: SwapPaymentDetailKey,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct GetPaymentDetailsResponse {
    pub payment_detail: Vec<PaymentDetail>,
}

// set/create detail

#[derive(Debug, Deserialize, Serialize, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct SetDetailRequestBody {
    pub address: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema, utoipa::IntoParams))]
pub struct SetDetailRequestQueries {
    pub payment_method_key: SwapPaymentDetailKey,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct SetPaymentDetailResponse {
    pub payment_detail: Option<PaymentDetail>,
}

// delete detail

#[derive(Debug, Deserialize, Serialize, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema, utoipa::IntoParams))]
pub struct DeleteDetailRequestQueries {
    pub payment_method_key: SwapPaymentDetailKey,
    pub payment_detail_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct DeletePaymentDetailResponse {
    pub payment_detail: PaymentDetail,
}
