//! This module contains the conversion of shared types between Swift and Rust
//! which are declared in the bridge module `pub mod ffi` inside `lib.rs`.
use crate::{
    convert_enum, convert_simple_struct,
    ffi::{self},
};

impl From<sdk::types::ApiTxStatus> for ffi::TxStatus {
    fn from(value: sdk::types::ApiTxStatus) -> Self {
        match value {
            sdk::types::ApiTxStatus::Pending => ffi::TxStatus::Pending,
            sdk::types::ApiTxStatus::WaitingForVerification(_vec) => ffi::TxStatus::WaitingForVerification,
            sdk::types::ApiTxStatus::Valid => ffi::TxStatus::Valid,
            sdk::types::ApiTxStatus::Invalid(_vec) => ffi::TxStatus::Invalid,
            sdk::types::ApiTxStatus::ProcessingIncoming => ffi::TxStatus::ProcessingIncoming,
            sdk::types::ApiTxStatus::ProcessingOutgoing => ffi::TxStatus::ProcessingOutgoing,
            sdk::types::ApiTxStatus::Completed => ffi::TxStatus::Completed,
            sdk::types::ApiTxStatus::Failed => ffi::TxStatus::Failed,
        }
    }
}

convert_simple_struct!(sdk::types::NewCaseIdResponse, ffi::NewCaseIdResponse, case_id, case_url,);

convert_simple_struct!(
    sdk::types::CaseDetailsResponse,
    ffi::CaseDetailsResponse,
    case_id,
    archived,
    status,
);

impl TryFrom<sdk::types::transactions::PurchaseDetails> for ffi::PurchaseDetails {
    type Error = sdk::Error;
    fn try_from(value: sdk::types::transactions::PurchaseDetails) -> Result<Self, Self::Error> {
        let invalid_reasons = match value.clone().status {
            sdk::types::ApiTxStatus::WaitingForVerification(r) => r,
            sdk::types::ApiTxStatus::Invalid(r) => r,
            _ => Vec::new(),
        };

        Ok(ffi::PurchaseDetails {
            main_address: value.system_address,
            amount: f64::try_from(value.amount)?,
            status: value.status.into(),
            invalid_reasons,
        })
    }
}

convert_simple_struct!(sdk::types::viviswap::NewViviswapUser, ffi::NewViviswapUser, username,);

convert_enum!(
    sdk::types::viviswap::ViviswapVerificationStep,
    ffi::ViviswapVerificationStep,
    Undefined,
    General,
    Personal,
    Residence,
    Identity,
    Amla,
    Documents,
);

convert_enum!(
    sdk::types::viviswap::ViviswapVerificationStatus,
    ffi::ViviswapVerificationStatus,
    Verified,
    Unverified,
    PartiallyVerified,
);

impl From<sdk::types::viviswap::ViviswapKycStatus> for ffi::ViviswapKycStatus {
    fn from(value: sdk::types::viviswap::ViviswapKycStatus) -> Self {
        ffi::ViviswapKycStatus {
            full_name: value.full_name,
            submission_step: value.submission_step.into(),
            verified_step: value.verified_step.into(),
            verification_status: value.verification_status.into(),
            monthly_limit_eur: value.monthly_limit_eur,
        }
    }
}

impl From<sdk::types::viviswap::ViviswapPartiallyKycDetails> for ffi::ViviswapPartiallyKycDetails {
    fn from(value: sdk::types::viviswap::ViviswapPartiallyKycDetails) -> Self {
        ffi::ViviswapPartiallyKycDetails {
            is_individual: value.is_individual,
            is_pep: value.is_pep,
            is_us_citizen: value.is_us_citizen,
            is_regulatory_disclosure: value.is_regulatory_disclosure,
            country_of_residence: value.country_of_residence.unwrap_or("".to_string()),
            nationality: value.nationality.unwrap_or("".to_string()),
            full_name: value.full_name.unwrap_or("".to_string()),
            date_of_birth: value.date_of_birth.unwrap_or("".to_string()),
        }
    }
}

convert_simple_struct!(
    sdk::types::KycAmlaQuestion,
    crate::ffi_functions::KycAmlaQuestion,
    id,
    question,
    possible_answers,
    is_free_text,
    min_answers,
    max_answers,
);

impl From<sdk::types::KycOpenDocument> for crate::ffi_functions::KycOpenDocument {
    fn from(value: sdk::types::KycOpenDocument) -> Self {
        crate::ffi_functions::KycOpenDocument {
            id: value.id,
            is_back_image_required: value.is_back_image_required,
            document_type: value.r#type,
            description: value.description,
        }
    }
}

convert_simple_struct!(
    sdk::types::viviswap::ViviswapAddressDetail,
    ffi::ViviswapAddressDetail,
    id,
    address,
    is_verified,
);

convert_simple_struct!(
    sdk::types::viviswap::ViviswapDepositDetails,
    ffi::ViviswapDepositDetails,
    reference,
    beneficiary,
    name_of_bank,
    address_of_bank,
    iban,
    bic,
);

impl From<sdk::types::viviswap::ViviswapDeposit> for ffi::ViviswapDeposit {
    fn from(value: sdk::types::viviswap::ViviswapDeposit) -> Self {
        ffi::ViviswapDeposit {
            contract_id: value.contract_id,
            deposit_address: value.deposit_address,
            details: value.details.into(),
        }
    }
}

convert_simple_struct!(
    sdk::types::viviswap::ViviswapWithdrawalDetails,
    ffi::ViviswapWithdrawalDetails,
    reference,
    wallet_id,
    crypto_address,
);

impl From<sdk::types::viviswap::ViviswapWithdrawal> for ffi::ViviswapWithdrawal {
    fn from(value: sdk::types::viviswap::ViviswapWithdrawal) -> Self {
        ffi::ViviswapWithdrawal {
            contract_id: value.contract_id,
            deposit_address: value.deposit_address,
            details: value.details.into(),
        }
    }
}

convert_simple_struct!(sdk::types::PaymentDetail, ffi::PaymentDetail, id, address, is_verified,);

impl From<sdk::types::Order> for crate::ffi_functions::Order {
    fn from(value: sdk::types::Order) -> Self {
        crate::ffi_functions::Order {
            id: value.id,
            is_payed_out: value.is_payed_out,
            is_approved: value.is_approved,
            is_canceled: value.is_canceled,
            fees_amount_eur: value.fees_amount_eur,
            crypto_fees: value.crypto_fees,
            contract_id: value.contract_id,
            incoming_payment_method_id: value.incoming_payment_method_id,
            incoming_payment_method_currency: value.incoming_payment_method_currency,
            incoming_amount: value.incoming_amount,
            incoming_course: value.incoming_course,
            outgoing_payment_method_id: value.outgoing_payment_method_id,
            outgoing_payment_method_currency: value.outgoing_payment_method_currency,
            outgoing_amount: value.outgoing_amount,
            outgoing_course: value.outgoing_course,
            refund_amount: value.refund_amount,
            refund_course: value.refund_course,
            refund_payment_method_id: value.refund_payment_method_id.unwrap_or("".to_string()),
            status: value.status,
            creation_date: value.creation_date,
            incoming_payment_detail: serde_json::to_string(&value.incoming_payment_detail).unwrap_or("".to_string()),
            outgoing_payment_detail: serde_json::to_string(&value.outgoing_payment_detail).unwrap_or("".to_string()),
            refund_payment_detail: serde_json::to_string(&value.refund_payment_detail).unwrap_or("".to_string()),
        }
    }
}

impl From<sdk::types::transactions::TxInfo> for crate::ffi_functions::TxInfo {
    fn from(value: sdk::types::transactions::TxInfo) -> Self {
        let invalid_reasons = match value.clone().status {
            sdk::types::ApiTxStatus::WaitingForVerification(r) => r,
            sdk::types::ApiTxStatus::Invalid(r) => r,
            _ => Vec::new(),
        };

        crate::ffi_functions::TxInfo {
            date: value.date.unwrap_or("".to_string()),
            sender: value.sender,
            receiver: value.receiver,
            reference_id: value.reference_id,
            application_metadata: serde_json::to_string(&value.application_metadata).unwrap_or("".to_string()),
            amount: value.amount,
            currency: value.currency,
            status: value.status.into(),
            transaction_hash: value.transaction_hash.unwrap_or("".to_string()),
            course: value.course,
            invalid_reasons,
        }
    }
}

impl From<sdk::types::transactions::WalletTxInfo> for crate::ffi_functions::WalletTxInfo {
    fn from(value: sdk::types::transactions::WalletTxInfo) -> Self {
        crate::ffi_functions::WalletTxInfo {
            date: value.date,
            block_id: value.block_id.unwrap_or("".to_string()),
            transaction_id: value.transaction_id,
            incoming: value.incoming,
            amount: value.amount,
            network: value.network,
            status: value.status,
            explorer_url: value.explorer_url.unwrap_or("".to_string()),
        }
    }
}

impl From<sdk::types::networks::ApiNetwork> for crate::ffi_functions::Network {
    fn from(value: sdk::types::networks::ApiNetwork) -> Self {
        crate::ffi_functions::Network {
            key: value.key,
            display_name: value.display_name,
        }
    }
}

convert_enum!(ffi::Currency, sdk::types::currencies::Currency, Iota, Eth,);

convert_enum!(
    ffi::OfficialDocumentType,
    sdk::types::OfficialDocumentType,
    Passport,
    DriversLicense,
    Id,
);

impl From<ffi::File> for sdk::types::File {
    fn from(value: ffi::File) -> Self {
        sdk::types::File::from_bytes(&value.data, &value.filename)
    }
}

impl From<ffi::IdentityOfficialDocumentData> for sdk::types::IdentityOfficialDocumentData {
    fn from(value: ffi::IdentityOfficialDocumentData) -> Self {
        sdk::types::IdentityOfficialDocumentData {
            r#type: value.doc_type.into(),
            expiration_date: value.expiration_date,
            document_number: value.document_number,
            front_image: value.front_image.into(),
            back_image: value.back_image.map(Into::into),
        }
    }
}

impl From<ffi::IdentityPersonalDocumentData> for sdk::types::IdentityPersonalDocumentData {
    fn from(value: ffi::IdentityPersonalDocumentData) -> Self {
        sdk::types::IdentityPersonalDocumentData {
            video: value.video.into(),
        }
    }
}
