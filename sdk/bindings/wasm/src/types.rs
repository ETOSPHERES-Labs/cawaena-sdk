use crate::utils::{convert_enum, convert_simple_struct};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub enum Level {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

convert_enum!(log::Level, Level, Error, Warn, Info, Debug, Trace,);

#[wasm_bindgen]
pub enum Currency {
    Iota,
    Eth,
}

convert_enum!(sdk::types::currencies::Currency, Currency, Iota, Eth,);

#[wasm_bindgen(getter_with_clone, inspectable)]
pub struct NewCaseIdResponse {
    /// New Postident case id
    pub case_id: String,
    /// Username
    pub case_url: String,
}

#[wasm_bindgen(getter_with_clone)]
pub struct CaseDetailsResponse {
    pub case_id: String,
    pub archived: bool,
    pub status: String,
}

#[wasm_bindgen(getter_with_clone, inspectable)]
pub struct PurchaseDetails {
    /// The main address where the fees goes to.
    pub main_address: String,
    /// The amount to be paid.
    pub amount: f64,
    /// The status of transaction
    pub status: TxStatus,
    /// Transaction invalid reasons
    pub invalid_reasons: Vec<String>,
}

#[wasm_bindgen]
#[derive(Clone)]
pub enum TxStatus {
    Pending,
    WaitingForVerification,
    Valid,
    Invalid,
    ProcessingIncoming,
    ProcessingOutgoing,
    Completed,
    Failed,
}
impl From<sdk::types::ApiTxStatus> for TxStatus {
    fn from(value: sdk::types::ApiTxStatus) -> Self {
        match value {
            sdk::types::ApiTxStatus::Pending => TxStatus::Pending,
            sdk::types::ApiTxStatus::WaitingForVerification(_vec) => TxStatus::WaitingForVerification,
            sdk::types::ApiTxStatus::Valid => TxStatus::Valid,
            sdk::types::ApiTxStatus::Invalid(_vec) => TxStatus::Invalid,
            sdk::types::ApiTxStatus::ProcessingIncoming => TxStatus::ProcessingIncoming,
            sdk::types::ApiTxStatus::ProcessingOutgoing => TxStatus::ProcessingOutgoing,
            sdk::types::ApiTxStatus::Completed => TxStatus::Completed,
            sdk::types::ApiTxStatus::Failed => TxStatus::Failed,
        }
    }
}

#[wasm_bindgen(getter_with_clone, inspectable)]
#[derive(Serialize, Deserialize)]
pub struct Network {
    pub key: String,
    pub display_name: String,
}

impl From<sdk::types::networks::ApiNetwork> for Network {
    fn from(value: sdk::types::networks::ApiNetwork) -> Self {
        Network {
            key: value.key,
            display_name: value.display_name,
        }
    }
}

#[wasm_bindgen(getter_with_clone, inspectable)]
pub struct TxList {
    pub txs: Vec<TxInfo>,
}

#[wasm_bindgen(getter_with_clone, inspectable)]
#[derive(Clone)]
pub struct TxInfo {
    /// Tx creation date, if available
    pub date: Option<String>,
    /// sender of the transaction
    pub sender: String,
    /// receiver of the transaction
    pub receiver: String,
    /// etopay reference id for the transaction
    pub reference_id: String,
    /// Application specific metadata attached to the tx
    pub application_metadata: Option<ApplicationMetadata>,
    /// Amount of transfer
    pub amount: f64,
    /// Currency of transfer
    pub currency: String,
    /// Status of the transfer
    pub status: TxStatus,
    /// The transaction hash on the network
    pub transaction_hash: Option<String>,
    /// Exchange rate
    pub course: f64,
    /// Reasons in case of invalid tx
    pub invalid_reasons: Vec<String>,
}

#[wasm_bindgen(getter_with_clone, inspectable)]
#[derive(Clone)]
pub struct ApplicationMetadata {
    pub product_hash: String,
    pub reason: String,
    pub purchase_model: String,
    pub app_data: String,
}

impl From<sdk::types::ApiApplicationMetadata> for ApplicationMetadata {
    fn from(value: sdk::types::ApiApplicationMetadata) -> Self {
        Self {
            product_hash: value.product_hash,
            reason: value.reason,
            purchase_model: value.purchase_model,
            app_data: value.app_data,
        }
    }
}

impl From<sdk::types::transactions::TxInfo> for TxInfo {
    fn from(value: sdk::types::transactions::TxInfo) -> Self {
        let invalid_reasons = match value.clone().status {
            sdk::types::ApiTxStatus::WaitingForVerification(r) => r,
            sdk::types::ApiTxStatus::Invalid(r) => r,
            _ => Vec::new(),
        };

        Self {
            date: value.date,
            sender: value.sender,
            receiver: value.receiver,
            reference_id: value.reference_id,
            application_metadata: value.application_metadata.map(Into::into),
            amount: value.amount,
            currency: value.currency,
            status: value.status.into(),
            transaction_hash: value.transaction_hash,
            course: value.course,
            invalid_reasons,
        }
    }
}

#[wasm_bindgen(getter_with_clone, inspectable)]
#[derive(Clone)]
pub struct WalletTxInfo {
    /// Tx creation date, if available
    pub date: String,
    /// Contains block id
    pub block_id: Option<String>,
    /// transaction id for particular transaction
    pub transaction_id: String,
    /// Describes type of transaction
    pub incoming: bool,
    /// Amount of transfer
    pub amount: f64,
    /// either SMR or IOTA
    pub network: String,
    /// Status of the transfer
    pub status: String,
    /// Url of network explorer
    pub explorer_url: Option<String>,
}

#[wasm_bindgen(getter_with_clone, inspectable)]
pub struct WalletTxInfoList {
    pub transactions: Vec<WalletTxInfo>,
}

impl From<sdk::types::transactions::WalletTxInfo> for WalletTxInfo {
    fn from(value: sdk::types::transactions::WalletTxInfo) -> Self {
        Self {
            date: value.date,
            block_id: value.block_id,
            transaction_id: value.transaction_id,
            incoming: value.incoming,
            amount: value.amount,
            network: value.network,
            status: value.status,
            explorer_url: value.explorer_url,
        }
    }
}

#[wasm_bindgen(getter_with_clone, inspectable)]
pub struct ViviswapAddressDetail {
    /// the unique id of the address detail
    pub id: String,
    /// the address used in the detail
    pub address: String,
    /// the status from viviswap, whether the address is verified
    pub is_verified: bool,
}

impl From<sdk::types::viviswap::ViviswapAddressDetail> for ViviswapAddressDetail {
    fn from(value: sdk::types::viviswap::ViviswapAddressDetail) -> Self {
        Self {
            id: value.id,
            address: value.address,
            is_verified: value.is_verified,
        }
    }
}

#[wasm_bindgen(getter_with_clone, inspectable)]
pub struct ViviswapDeposit {
    /// The unique UUID of the contract
    pub contract_id: String,
    /// The deposit address (crypto) where the swap will put the funds from fiat
    pub deposit_address: String,
    /// The details of the deposit (for the user)
    pub details: ViviswapDepositDetails,
}

impl From<sdk::types::viviswap::ViviswapDeposit> for ViviswapDeposit {
    fn from(value: sdk::types::viviswap::ViviswapDeposit) -> Self {
        Self {
            contract_id: value.contract_id,
            deposit_address: value.deposit_address,
            details: ViviswapDepositDetails {
                reference: value.details.reference,
                beneficiary: value.details.beneficiary,
                name_of_bank: value.details.name_of_bank,
                address_of_bank: value.details.address_of_bank,
                iban: value.details.iban,
                bic: value.details.bic,
            },
        }
    }
}

#[wasm_bindgen(getter_with_clone, inspectable)]
#[derive(Clone)]
pub struct ViviswapDepositDetails {
    /// The reference to be entered by the user in his SEPA bank transfer
    pub reference: String,
    /// The name of the beneficiary receiving the SEPA transfer
    pub beneficiary: String,
    /// The name of the bank of the beneficiary
    pub name_of_bank: String,
    /// The address of the bank of the beneficiary
    pub address_of_bank: String,
    /// The IBAN of the beneficiary
    pub iban: String,
    /// The BIC/SWIFT code for the SEPA transfer
    pub bic: String,
}

/// Viviswap withdrawal details for crypto to FIAT swap
#[wasm_bindgen(getter_with_clone, inspectable)]
#[derive(Clone)]
pub struct ViviswapWithdrawalDetails {
    /// The reference used by viviswap for the SEPA transfer
    pub reference: String,
    /// The id of the unique wallet internal to viviswap
    pub wallet_id: String,
    /// The crypto address of viviswap where the crypto swap is to be sent
    pub crypto_address: String,
}

/// The viviswap withdrawal contract information
#[wasm_bindgen(getter_with_clone, inspectable)]
pub struct ViviswapWithdrawal {
    /// The unique UUID to track the withdrawal contract
    pub contract_id: String,
    /// The deposit address, in this case the IBAN of the user, where fiat will be deposited.
    pub deposit_address: String,
    /// The details of the withdrawal
    pub details: ViviswapWithdrawalDetails,
}

impl From<sdk::types::viviswap::ViviswapWithdrawal> for ViviswapWithdrawal {
    fn from(value: sdk::types::viviswap::ViviswapWithdrawal) -> Self {
        Self {
            contract_id: value.contract_id,
            deposit_address: value.deposit_address,
            details: ViviswapWithdrawalDetails {
                reference: value.details.reference,
                wallet_id: value.details.wallet_id,
                crypto_address: value.details.crypto_address,
            },
        }
    }
}

#[wasm_bindgen(getter_with_clone, inspectable)]
#[derive(Clone)]
pub struct Order {
    pub id: String,
    pub is_payed_out: bool,
    pub is_approved: bool,
    pub is_canceled: bool,
    pub fees_amount_eur: f32,
    pub crypto_fees: f32,
    pub contract_id: String,
    pub incoming_payment_method_id: String,
    pub incoming_payment_method_currency: String,
    pub incoming_amount: f32,
    pub incoming_course: f32,
    pub outgoing_payment_method_id: String,
    pub outgoing_payment_method_currency: String,
    pub outgoing_amount: f32,
    pub outgoing_course: f32,
    pub refund_amount: Option<f32>,
    pub refund_course: Option<f32>,
    pub refund_payment_method_id: Option<String>,
    pub status: i32,
    pub creation_date: String,
    pub incoming_payment_detail: Option<PaymentDetail>,
    pub outgoing_payment_detail: Option<PaymentDetail>,
    pub refund_payment_detail: Option<PaymentDetail>,
}

/// Orders list
#[wasm_bindgen(getter_with_clone, inspectable)]
pub struct OrderList {
    pub orders: Vec<Order>,
}

#[wasm_bindgen(getter_with_clone, inspectable)]
#[derive(Clone)]
pub struct PaymentDetail {
    pub id: String,
    pub address: String,
    pub is_verified: Option<bool>,
}

impl From<sdk::types::PaymentDetail> for PaymentDetail {
    fn from(value: sdk::types::PaymentDetail) -> Self {
        Self {
            address: value.address,
            id: value.id,
            is_verified: value.is_verified,
        }
    }
}

impl From<sdk::types::Order> for Order {
    fn from(value: sdk::types::Order) -> Self {
        Self {
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
            refund_payment_method_id: value.refund_payment_method_id,
            status: value.status,
            creation_date: value.creation_date,
            incoming_payment_detail: value.incoming_payment_detail.map(Into::into),
            outgoing_payment_detail: value.outgoing_payment_detail.map(Into::into),
            refund_payment_detail: value.refund_payment_detail.map(Into::into),
        }
    }
}

#[wasm_bindgen(getter_with_clone, inspectable)]
pub struct NewViviswapUser {
    /// Username of new viviswap user
    pub username: String,
}

impl From<sdk::types::viviswap::NewViviswapUser> for NewViviswapUser {
    fn from(value: sdk::types::viviswap::NewViviswapUser) -> Self {
        Self {
            username: value.username,
        }
    }
}

#[wasm_bindgen(getter_with_clone, inspectable)]
pub struct ViviswapKycStatus {
    /// full name of the user
    pub full_name: String,
    /// the current submission step in the KYC onboarding process for the user
    pub submission_step: ViviswapVerificationStep,
    /// the current verified step in the KYC onboarding process for the user
    pub verified_step: ViviswapVerificationStep,
    /// the user verification status
    pub verification_status: ViviswapVerificationStatus,
    /// The monthly swap limit in euros
    pub monthly_limit_eur: f32,
}

#[wasm_bindgen]
#[derive(Clone)]
pub enum ViviswapVerificationStep {
    /// no verification step (no next verification step available)
    Undefined,
    /// general verification step
    General,
    /// personal verification step
    Personal,
    /// residence verification step
    Residence,
    /// identity verification step
    Identity,
    /// amla general verification step
    Amla,
    /// document verification step
    Documents,
}

#[wasm_bindgen]
#[derive(Clone)]
pub enum ViviswapVerificationStatus {
    /// The user is fully verified
    Verified,
    /// The user is not verified
    Unverified,
    /// The user is partially verified
    PartiallyVerified,
}

convert_enum!(
    sdk::types::viviswap::ViviswapVerificationStep,
    ViviswapVerificationStep,
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
    ViviswapVerificationStatus,
    Verified,
    Unverified,
    PartiallyVerified,
);

impl From<sdk::types::viviswap::ViviswapKycStatus> for ViviswapKycStatus {
    fn from(value: sdk::types::viviswap::ViviswapKycStatus) -> Self {
        Self {
            full_name: value.full_name,
            submission_step: value.submission_step.into(),
            verified_step: value.verified_step.into(),
            verification_status: value.verification_status.into(),
            monthly_limit_eur: value.monthly_limit_eur,
        }
    }
}

#[wasm_bindgen(getter_with_clone, inspectable)]
pub struct ViviswapPartiallyKycDetails {
    /// Is the user an individual
    pub is_individual: Option<bool>,
    /// Is the user a politically exposed person
    pub is_pep: Option<bool>,
    /// Is the user a US citizen
    pub is_us_citizen: Option<bool>,
    /// Is the regulatory disclosure confirmed by user
    pub is_regulatory_disclosure: Option<bool>,
    /// The country of tax residence of the user
    pub country_of_residence: Option<String>,
    /// The user's nationality
    pub nationality: Option<String>,
    /// The full name of the user as per his legal documents
    pub full_name: Option<String>,
    /// The date of birth of the user as per his legal documents
    pub date_of_birth: Option<String>,
}

convert_simple_struct!(
    sdk::types::viviswap::ViviswapPartiallyKycDetails,
    ViviswapPartiallyKycDetails,
    is_individual,
    is_pep,
    is_us_citizen,
    is_regulatory_disclosure,
    country_of_residence,
    nationality,
    full_name,
    date_of_birth,
);

#[wasm_bindgen]
pub enum OfficialDocumentType {
    Passport,
    DriversLicense,
    Id,
}
convert_enum!(
    sdk::types::OfficialDocumentType,
    OfficialDocumentType,
    Passport,
    DriversLicense,
    Id,
);

#[wasm_bindgen(getter_with_clone, inspectable)]
pub struct OpenAmlaQuestions {
    pub questions: Vec<KycAmlaQuestion>,
}

#[wasm_bindgen(getter_with_clone, inspectable)]
#[derive(Clone)]
pub struct KycAmlaQuestion {
    /// The unique ID of this question.
    pub id: String,

    /// The question the user has to answer.
    pub question: String,

    /// A list of available answers that the user can choose from.
    pub possible_answers: Vec<String>,

    /// Indicator if this question allows free text answers.
    pub is_free_text: bool,

    /// The minumum number of answers (including the free-text answer) that are required.
    pub min_answers: i32,

    /// The maximum number of answers (including the free-text answer) that are allowed.
    pub max_answers: i32,
}

convert_simple_struct!(
    sdk::types::KycAmlaQuestion,
    KycAmlaQuestion,
    id,
    question,
    possible_answers,
    is_free_text,
    min_answers,
    max_answers,
);

#[wasm_bindgen(getter_with_clone, inspectable)]
#[derive(Clone)]
pub struct KycOpenDocument {
    pub id: String,
    pub is_back_image_required: bool,
    pub r#type: String,
    pub description: String,
}

convert_simple_struct!(
    sdk::types::KycOpenDocument,
    KycOpenDocument,
    id,
    is_back_image_required,
    r#type,
    description,
);

#[wasm_bindgen(getter_with_clone, inspectable)]
pub struct OpenDocuments {
    pub documents: Vec<KycOpenDocument>,
}
