//! This module contains the generated FFI glue between Swift and Rust code by using the `swift_bridge::bridge` procedural macro to declare a bridge module.
//! We have declared some shared types which are used by both Rust and Swift and are converted accordingly in the `type_conversion.rs` module.
//! The FFI functions exported by Rust are declared in the `ffi_functions.rs` module.
//! At build time the `swift-bridge-build` crate is used to generate the corresponding Swift and C FFI glue code.

// The swift bridge macros are likely generating code that's outside our control, which triggers some warnings.
// There is no clear way to resolving these warnings.
// Therefore, we are ignoring them.
#![allow(clippy::not_unsafe_ptr_arg_deref)]

mod utils;
use ffi_functions::*;
mod ffi_functions;
mod type_conversion;

#[swift_bridge::bridge]
pub mod ffi {
    #[swift_bridge(swift_repr = "struct")]
    pub struct NewCaseIdResponse {
        pub case_id: String,
        pub case_url: String,
    }

    #[swift_bridge(swift_repr = "struct")]
    pub struct CaseDetailsResponse {
        pub case_id: String,
        pub archived: bool,
        pub status: String,
    }

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

    #[swift_bridge(swift_repr = "struct")]
    pub struct PurchaseDetails {
        pub main_address: String,
        pub amount: f64,
        pub status: TxStatus,
        pub invalid_reasons: Vec<String>,
    }

    #[swift_bridge(swift_repr = "struct")]
    pub struct NewViviswapUser {
        pub username: String,
    }

    pub enum ViviswapVerificationStep {
        Undefined,
        General,
        Personal,
        Residence,
        Identity,
        Amla,
        Documents,
    }

    pub enum ViviswapVerificationStatus {
        Verified,
        Unverified,
        PartiallyVerified,
    }

    #[swift_bridge(swift_repr = "struct")]
    pub struct ViviswapKycStatus {
        pub full_name: String,
        pub submission_step: ViviswapVerificationStep,
        pub verified_step: ViviswapVerificationStep,
        pub verification_status: ViviswapVerificationStatus,
        pub monthly_limit_eur: f32,
    }

    #[swift_bridge(swift_repr = "struct")]
    pub struct ViviswapPartiallyKycDetails {
        pub is_individual: Option<bool>,
        pub is_pep: Option<bool>,
        pub is_us_citizen: Option<bool>,
        pub is_regulatory_disclosure: Option<bool>,
        pub country_of_residence: String,
        pub nationality: String,
        pub full_name: String,
        pub date_of_birth: String,
    }

    #[swift_bridge(swift_repr = "struct")]
    pub struct ViviswapAddressDetail {
        pub id: String,
        pub address: String,
        pub is_verified: bool,
    }

    #[swift_bridge(swift_repr = "struct")]
    pub struct ViviswapDepositDetails {
        pub reference: String,
        pub beneficiary: String,
        pub name_of_bank: String,
        pub address_of_bank: String,
        pub iban: String,
        pub bic: String,
    }

    #[swift_bridge(swift_repr = "struct")]
    pub struct ViviswapDeposit {
        pub contract_id: String,
        pub deposit_address: String,
        pub details: ViviswapDepositDetails,
    }

    #[swift_bridge(swift_repr = "struct")]
    pub struct ViviswapWithdrawalDetails {
        pub reference: String,
        pub wallet_id: String,
        pub crypto_address: String,
    }

    #[swift_bridge(swift_repr = "struct")]
    pub struct ViviswapWithdrawal {
        pub contract_id: String,
        pub deposit_address: String,
        pub details: ViviswapWithdrawalDetails,
    }

    #[swift_bridge(swift_repr = "struct")]
    pub struct PaymentDetail {
        pub id: String,
        pub address: String,
        pub is_verified: Option<bool>,
    }

    pub enum Currency {
        Iota,
        Eth,
    }

    pub enum OfficialDocumentType {
        Id,
        Passport,
        DriversLicense,
    }

    #[swift_bridge(swift_repr = "struct")]
    pub struct File {
        data: Vec<u8>,
        filename: String,
    }

    #[swift_bridge(swift_repr = "struct")]
    pub struct IdentityOfficialDocumentData {
        pub doc_type: OfficialDocumentType,
        pub expiration_date: String,
        pub document_number: String,
        pub front_image: File,
        pub back_image: Option<File>,
    }

    #[swift_bridge(swift_repr = "struct")]
    pub struct IdentityPersonalDocumentData {
        pub video: File,
    }

    extern "Rust" {
        type KycOpenDocument;

        fn id(&self) -> String;
        fn is_back_image_required(&self) -> bool;
        fn document_type(&self) -> String;
        fn description(&self) -> String;
    }

    extern "Rust" {
        type KycAmlaQuestion;

        fn id(&self) -> String;
        fn is_free_text(&self) -> bool;
        fn question(&self) -> String;
        fn min_answers(&self) -> i32;
        fn max_answers(&self) -> i32;
        fn possible_answers(&self) -> Vec<String>;
    }

    extern "Rust" {
        type Order;

        fn id(&self) -> String;
        fn is_payed_out(&self) -> bool;
        fn is_approved(&self) -> bool;
        fn is_canceled(&self) -> bool;
        fn fees_amount_eur(&self) -> f32;
        fn crypto_fees(&self) -> f32;
        fn contract_id(&self) -> String;
        fn incoming_payment_method_id(&self) -> String;
        fn incoming_payment_method_currency(&self) -> String;
        fn incoming_amount(&self) -> f32;
        fn incoming_course(&self) -> f32;
        fn outgoing_payment_method_id(&self) -> String;
        fn outgoing_payment_method_currency(&self) -> String;
        fn outgoing_amount(&self) -> f32;
        fn outgoing_course(&self) -> f32;
        fn refund_amount(&self) -> Option<f32>;
        fn refund_course(&self) -> Option<f32>;
        fn refund_payment_method_id(&self) -> String;
        fn status(&self) -> i32;
        fn creation_date(&self) -> String;
        fn incoming_payment_detail(&self) -> String;
        fn outgoing_payment_detail(&self) -> String;
        fn refund_payment_detail(&self) -> String;
    }

    extern "Rust" {
        type TxInfo;

        fn date(&self) -> String;
        fn sender(&self) -> String;
        fn receiver(&self) -> String;
        fn reference_id(&self) -> String;
        fn application_metadata(&self) -> String;
        fn amount(&self) -> f64;
        fn currency(&self) -> String;
        fn status(&self) -> TxStatus;
        fn transaction_hash(&self) -> String;
        fn course(&self) -> f64;
        fn invalid_reasons(&self) -> Vec<String>;
    }

    extern "Rust" {
        type WalletTxInfo;

        fn date(&self) -> String;
        fn block_id(&self) -> String;
        fn transaction_id(&self) -> String;
        fn receiver(&self) -> String;
        fn incoming(&self) -> bool;
        fn amount(&self) -> f64;
        fn network(&self) -> String;
        fn status(&self) -> String;
        fn explorer_url(&self) -> String;
    }

    extern "Rust" {
        type Network;

        fn key(&self) -> String;
        fn display_name(&self) -> String;
    }

    // Export Rust functions with the above shared types for Swift.
    extern "Rust" {
        type ETOPaySdk;

        #[swift_bridge(init)]
        fn new() -> ETOPaySdk;

        #[swift_bridge(swift_name = "setConfig")]
        async fn set_config(&self, config: String) -> Result<(), String>;
        #[swift_bridge(swift_name = "getNetworks")]
        async fn get_networks(&self) -> Result<Vec<Network>, String>;
        #[swift_bridge(swift_name = "setNetwork")]
        async fn set_network(&self, network_key: String) -> Result<(), String>;
        async fn destroy(&self) -> Result<(), String>;
        #[swift_bridge(swift_name = "createNewUser")]
        async fn create_new_user(&self, username: String) -> Result<(), String>;
        #[swift_bridge(swift_name = "initUser")]
        async fn init_user(&self, username: String) -> Result<(), String>;
        #[swift_bridge(swift_name = "refreshAccessToken")]
        async fn refresh_access_token(&self, access_token: String) -> Result<(), String>;
        #[swift_bridge(swift_name = "isKycVerified")]
        async fn is_kyc_verified(&self, username: String) -> Result<bool, String>;
        #[swift_bridge(swift_name = "verifyMnemonic")]
        async fn verify_mnemonic(&self, pin: String, mnemonic: String) -> Result<bool, String>;
        #[swift_bridge(swift_name = "createNewWallet")]
        async fn create_new_wallet(&self, pin: String) -> Result<String, String>;
        #[swift_bridge(swift_name = "createWalletFromMnemonic")]
        async fn create_wallet_from_mnemonic(&self, pin: String, mnemonic: String) -> Result<(), String>;
        #[swift_bridge(swift_name = "restoreWalletFromBackup")]
        async fn restore_wallet_from_backup(
            &self,
            pin: String,
            backup: Vec<u8>,
            backup_password: String,
        ) -> Result<(), String>;
        #[swift_bridge(swift_name = "createWalletBackup")]
        async fn create_wallet_backup(&self, pin: String, password: String) -> Result<Vec<u8>, String>;
        #[swift_bridge(swift_name = "deleteWallet")]
        async fn delete_wallet(&self, pin: String) -> Result<(), String>;
        #[swift_bridge(swift_name = "generateNewAddress")]
        async fn generate_new_address(&self, pin: String) -> Result<String, String>;
        #[swift_bridge(swift_name = "getWalletBalance")]
        async fn get_balance(&self, pin: String) -> Result<f64, String>;

        // functions for postident, actual implementation is hidden behind feature flag
        #[swift_bridge(swift_name = "initKycVerificationForPostident")]
        async fn init_kyc_verification_for_postident(&self) -> Result<NewCaseIdResponse, String>;
        #[swift_bridge(swift_name = "getKycDetailsForPostident")]
        async fn get_kyc_details_for_postident(&self) -> Result<CaseDetailsResponse, String>;
        #[swift_bridge(swift_name = "updateKycDetailsForPostident")]
        async fn update_kyc_details_for_postident(&self, case_id: String) -> Result<(), String>;

        #[swift_bridge(swift_name = "createPurchaseRequest")]
        async fn create_purchase_request(
            &self,
            receiver: String,
            amount: f64,
            product_hash: String,
            app_data: String,
            purchase_type: String,
        ) -> Result<String, String>;
        #[swift_bridge(swift_name = "getPurchaseDetails")]
        async fn get_purchase_details(&self, purchase_id: String) -> Result<PurchaseDetails, String>;
        #[swift_bridge(swift_name = "confirmPurchaseRequest")]
        async fn confirm_purchase_request(&self, pin: String, purchase_id: String) -> Result<(), String>;
        #[swift_bridge(swift_name = "startKycVerificationForViviswap")]
        async fn start_kyc_verification_for_viviswap(
            &self,
            mail: String,
            terms_accepted: bool,
        ) -> Result<NewViviswapUser, String>;
        #[swift_bridge(swift_name = "getKycDetailsForViviswap")]
        async fn get_kyc_details_for_viviswap(&self) -> Result<ViviswapKycStatus, String>;
        #[swift_bridge(swift_name = "updateKycPartiallyStatusForViviswap")]
        async fn update_kyc_partially_status_for_viviswap(
            &self,
            is_individual: Option<bool>,
            is_pep: Option<bool>,
            is_us_citizen: Option<bool>,
            is_regulatory_disclosure: Option<bool>,
            country_of_residence: Option<String>,
            nationality: Option<String>,
            full_name: Option<String>,
            date_of_birth: Option<String>,
        ) -> Result<ViviswapPartiallyKycDetails, String>;
        #[swift_bridge(swift_name = "submitKycPartiallyStatusForViviswap")]
        async fn submit_kyc_partially_status_for_viviswap(&self) -> Result<(), String>;
        #[swift_bridge(swift_name = "setViviswapKycIdentityDetails")]
        async fn set_viviswap_kyc_identity_details(
            &self,
            identity_official_document_data: IdentityOfficialDocumentData,
            identity_personal_document_data: IdentityPersonalDocumentData,
        ) -> Result<(), String>;
        #[swift_bridge(swift_name = "setViviswapKycResidenceDetails")]
        async fn set_viviswap_kyc_residence_details(
            &self,
            country_code: String,
            region: String,
            zip_code: String,
            city: String,
            address_line_1: String,
            address_line_2: String,
            is_public_entry: bool,
            public_entry_reference: Option<String>,
            has_no_official_document: bool,
            official_document: Option<File>,
        ) -> Result<(), String>;
        #[swift_bridge(swift_name = "getViviswapKycAmlaOpenQuestions")]
        async fn get_viviswap_kyc_amla_open_questions(&self) -> Result<Vec<KycAmlaQuestion>, String>;
        #[swift_bridge(swift_name = "getViviswapKycOpenDocuments")]
        async fn get_viviswap_kyc_open_documents(&self) -> Result<Vec<KycOpenDocument>, String>;
        #[swift_bridge(swift_name = "setViviswapKycDocument")]
        async fn set_viviswap_kyc_document(
            &self,
            document_id: String,
            expiration_date: String,
            document_number: String,
            front_image: Option<File>,
            back_image: Option<File>,
        ) -> Result<(), String>;
        #[swift_bridge(swift_name = "setViviswapKycAmlaAnswer")]
        async fn set_viviswap_kyc_amla_answer(
            &self,
            question_id: String,
            answers: Vec<String>,
            freetext_answer: Option<String>,
        ) -> Result<(), String>;
        #[swift_bridge(swift_name = "verifyPin")]
        async fn verify_pin(&self, pin: String) -> Result<(), String>;
        #[swift_bridge(swift_name = "resetPin")]
        async fn reset_pin(&self, pin: String, new_pin: String) -> Result<(), String>;
        #[swift_bridge(swift_name = "setWalletPassword")]
        async fn set_wallet_password(&self, pin: String, new_password: String) -> Result<(), String>;
        #[swift_bridge(swift_name = "isWalletPasswordSet")]
        pub async fn is_wallet_password_set(&self) -> Result<bool, String>;
        #[swift_bridge(swift_name = "sendAmount")]
        async fn send_amount(
            &self,
            pin: String,
            address: String,
            amount: f64,
            data: Option<Vec<u8>>,
        ) -> Result<String, String>;
        #[swift_bridge(swift_name = "updateIbanViviswap")]
        async fn update_iban_viviswap(&self, pin: String, address: String) -> Result<ViviswapAddressDetail, String>;
        #[swift_bridge(swift_name = "getIbanViviswap")]
        async fn get_iban_viviswap(&self) -> Result<ViviswapAddressDetail, String>;
        #[swift_bridge(swift_name = "depositWithViviswap")]
        async fn deposit_with_viviswap(&self, pin: String) -> Result<ViviswapDeposit, String>;
        #[swift_bridge(swift_name = "createDetailViviswap")]
        async fn create_detail_viviswap(&self, pin: String) -> Result<ViviswapAddressDetail, String>;
        #[swift_bridge(swift_name = "withdrawWithViviswap")]
        async fn withdraw_with_viviswap(
            &self,
            amount: f64,
            pin: Option<String>,
            data: Option<Vec<u8>>,
        ) -> Result<ViviswapWithdrawal, String>;
        #[swift_bridge(swift_name = "getSwapDetails")]
        async fn get_swap_details(&self, order_id: String) -> Result<Order, String>;
        #[swift_bridge(swift_name = "getExchangeRate")]
        async fn get_exchange_rate(&self) -> Result<f64, String>;
        #[swift_bridge(swift_name = "deleteUser")]
        async fn delete_user(&self, pin: Option<String>) -> Result<(), String>;
        #[swift_bridge(swift_name = "getSwapList")]
        async fn get_swap_list(&self, start: u32, limit: u32) -> Result<Vec<Order>, String>;
        #[swift_bridge(swift_name = "getTransactionList")]
        async fn get_transaction_list(&self, start: u32, limit: u32) -> Result<Vec<TxInfo>, String>;
        #[swift_bridge(swift_name = "getWalletTransactionList")]
        async fn get_wallet_transaction_list(
            &self,
            pin: String,
            start: usize,
            limit: usize,
        ) -> Result<Vec<WalletTxInfo>, String>;
        #[swift_bridge(swift_name = "getWalletTransaction")]
        async fn get_wallet_transaction(&self, pin: String, transaction_id: String) -> Result<WalletTxInfo, String>;

        #[swift_bridge(swift_name = "getRecoveryShare")]
        async fn get_recovery_share(&self) -> Result<String, String>;
        #[swift_bridge(swift_name = "setRecoveryShare")]
        async fn set_recovery_share(&self, share: String) -> Result<(), String>;
        #[swift_bridge(swift_name = "getPreferredNetwork")]
        async fn get_preferred_network(&self) -> Result<String, String>;
        #[swift_bridge(swift_name = "setPreferredNetwork")]
        async fn set_preferred_network(&self, network_key: Option<String>) -> Result<(), String>;
        #[swift_bridge(swift_name = "getBuildInfo")]
        fn get_build_info(&self) -> String;
    }
}
