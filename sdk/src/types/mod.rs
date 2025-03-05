//! Types module for SDK
//!
//!

/// Business logic for config sdk module
pub mod currencies;
/// Errors related to sdk types
pub mod error;
/// Network definition
pub mod networks;
/// Newtypes used for sensitive data
pub mod newtypes;
/// business logic for transaction sdk module
pub mod transactions;
/// Business logic for user sdk module
pub mod users;
/// business logic for viviswap sdk module
pub mod viviswap;

/// Export some `api_types` for the bindings to reference
pub use api_types::api::{
    postident::{CaseDetailsResponse, NewCaseIdResponse},
    transactions::{ApiApplicationMetadata, ApiTxStatus},
    viviswap::{
        detail::PaymentDetail,
        kyc::{
            File, IdentityOfficialDocumentData, IdentityPersonalDocumentData, KycAmlaQuestion, KycOpenDocument,
            OfficialDocumentType,
        },
        order::{Order, OrderList},
    },
};
