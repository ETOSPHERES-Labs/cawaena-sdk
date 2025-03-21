//! This module contains the Rust FFI bindings for the Swift SDK.
//! It provides functions for setting up and interacting with the SDK.
//!
//! The FFI functions in this module are used to set various configuration options for the SDK,
//! such as the path prefix, authentication provider, backend URL, SDK environment, and currency type.
//! They also provide functionalities for creating a new user, initializing a user, getting the user state,
//! refreshing the access token, checking the KYC verification status, initializing the wallet, and verifying a mnemonic.
//! The `destroy` function is used to drop the SDK instance when it is no longer needed.
//!
//! The FFI functions return a `Result` response, which contains the shared types used for arguments and responses
//! declared in bridge module `pub mod ffi` inside `lib.rs`.
//! The shared types are generally the same as Rust types in SDK with slight adaptations
//! to bypass the limitations in the `swift-bridge` crate for some unsupported types.
//! The conversion of types between Swift and Rust is done in the `type_conversion.rs` module.

use crate::ffi::{
    CaseDetailsResponse, File, IdentityOfficialDocumentData, IdentityPersonalDocumentData, NewCaseIdResponse,
    NewViviswapUser, PurchaseDetails, TxStatus, ViviswapAddressDetail, ViviswapDeposit, ViviswapKycStatus,
    ViviswapPartiallyKycDetails, ViviswapWithdrawal,
};
use sdk::core::{Config, Sdk};
use sdk::types::currencies::CryptoAmount;
use sdk::types::newtypes::{AccessToken, EncryptionPin, PlainPassword};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Struct representing the ETOPay SDK with an inner data structure wrapped in an atomic reference count and read-write lock.
/// Utilizes atomic reference counting (`Arc`) and a read-write lock (`RwLock`) to provide thread-safe access to the inner data structure,
/// allowing multiple threads to concurrently read from or write to the ETOPay SDK while ensuring data integrity and preventing data races.
pub struct ETOPaySdk {
    inner: Arc<RwLock<sdk::core::Sdk>>,
}

impl ETOPaySdk {
    /// Create a new instance of the ETOPay Sdk.
    ///
    /// # Returns
    ///
    /// New `ETOPaySdk` instance with atomic reference count and read write lock
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(sdk::core::Sdk::default())),
        }
    }
    /// Sets the configuration as a JSON-encoded string.
    ///
    /// # Arguments
    ///
    /// * `config` - The input string representing the configuration with the following structure:
    /// ```json
    /// {
    ///     "auth_provider": "<authentication provider name>",
    ///     "backend_url": "<valid URL to the backend API>",
    ///     "storage_path": "/path/to/valid/folder",
    ///     "log_level": "info"
    /// }
    /// ```
    ///
    /// # Returns
    ///
    /// * Ok - if the configuration is set successfully.
    /// * Err - if the configuration is invalid.
    pub async fn set_config(&self, config: String) -> Result<(), String> {
        let mut sdk = self.inner.write().await;
        Config::from_json(&config)
            .and_then(|r| sdk.set_config(r))
            .map_err(|err| format!("{:#?}", err))
    }

    /// Fetch available networks.
    ///
    /// # Returns
    ///
    /// * Ok - Serialized string of a Vec<Network>>
    /// * Err - if there is an error fetching the networks.
    pub async fn get_networks(&self) -> Result<Vec<Network>, String> {
        let mut sdk = self.inner.write().await;
        async move {
            sdk.get_networks()
                .await
                .map(|n| n.into_iter().map(|network| network.into()).collect())
        }
        .await
        .map_err(|err| format!("{:#?}", err))
    }

    /// Selects the network for the ETOPay SDK.
    ///
    /// # Arguments
    ///
    /// * `network_key` - The input string representing the network id.
    ///
    /// # Returns
    ///
    /// * Ok - if the network is set successfully.
    /// * Err - if something went wrong.`
    pub async fn set_network(&self, network_key: String) -> Result<(), String> {
        let mut sdk = self.inner.write().await;
        sdk.set_network(network_key).await.map_err(|e| format!("{e:#?}"))
    }

    /// Destructor for the SDK handle
    ///
    /// # Arguments
    ///
    /// * `None`
    ///
    /// * Returns
    ///
    /// * Ok - if the sdk handle is dropped successfully.
    /// * Err - if something went wrong.
    pub async fn destroy(&self) -> Result<(), String> {
        let mut sdk = self.inner.write().await;
        let sdk = std::mem::take(&mut *sdk);
        drop(sdk);
        Ok(())
    }

    /// Creates a new sdk user
    ///
    /// # Arguments
    ///
    /// * `username` - The username of the new user.
    ///
    /// # Returns
    ///
    /// * Ok - empty if the user is created successfully.
    /// * Err - if there is an issue validating the configuration, initializing the repository, or creating the user.
    pub async fn create_new_user(&self, username: String) -> Result<(), String> {
        let mut sdk = self.inner.write().await;
        sdk.create_new_user(&username)
            .await
            .map_err(|err| format!("{:#?}", err))
    }

    /// Initializes the user
    ///
    /// # Arguments
    ///
    /// * `username` - The username of the user to initialize.
    ///
    /// # Returns
    ///
    /// * Ok - empty if the user is initialized successfully.
    /// * Err - if there is an issue validating the configuration, initializing the repository, or checking the KYC status.
    pub async fn init_user(&self, username: String) -> Result<(), String> {
        let mut sdk = self.inner.write().await;
        sdk.init_user(&username).await.map_err(|err| format!("{:#?}", err))
    }

    /// Refreshes the access token
    ///
    /// # Arguments
    ///
    /// * `access_token` - The new access token to be set.
    ///
    /// # Returns
    ///
    /// * Ok - empty if the access token is refreshed successfully.
    /// * Err - if there is an issue validating the configuration.
    pub async fn refresh_access_token(&self, access_token: String) -> Result<(), String> {
        let mut sdk = self.inner.write().await;
        async move {
            let access_token = if access_token.is_empty() {
                None
            } else {
                Some(AccessToken::try_from(access_token)?)
            };
            sdk.refresh_access_token(access_token).await
        }
        .await
        .map_err(|err| format!("{:#?}", err))
    }

    /// Gets the kyc verification status
    ///
    /// # Arguments
    ///
    /// * `username`` - The username of the user to check KYC status for.
    ///
    /// # Returns
    ///
    /// * Ok - true if the KYC status is verified, or false if it is not verified.
    /// * Err - if there is an issue validating the configuration, initializing the repository, or checking the KYC status.
    pub async fn is_kyc_verified(&self, username: String) -> Result<bool, String> {
        let mut sdk = self.inner.write().await;
        sdk.is_kyc_status_verified(&username)
            .await
            .map_err(|err| format!("{:#?}", err))
    }

    /// Verifies the given mnemonic
    ///
    /// # Arguments
    ///
    /// * `pin` - The PIN for the wallet.
    /// * `mnemonic` - The mnemonic to verify.
    ///
    /// # Returns
    ///
    /// * Ok(boolean) - if the mnemonic is successfully verified or not.
    /// * Err - if there is an error initializing the wallet.
    pub async fn verify_mnemonic(&self, pin: String, mnemonic: String) -> Result<bool, String> {
        let mut sdk = self.inner.write().await;
        async move {
            let pin = EncryptionPin::try_from_string(pin)?;
            sdk.verify_mnemonic(&pin, &mnemonic).await
        }
        .await
        .map_err(|err| format!("{:#?}", err))
    }

    /// Creates the new wallet
    ///
    /// # Arguments
    ///
    /// * `pin` - The PIN for the wallet.
    ///
    /// # Returns
    ///
    /// * Ok - returns the mnemonic phrase of the newly created wallet if successful.
    /// * Err - if there is an error initializing the wallet, initializing the repository, initializing the user.
    pub async fn create_new_wallet(&self, pin: String) -> Result<String, String> {
        let mut sdk = self.inner.write().await;
        async move {
            let pin = EncryptionPin::try_from_string(pin)?;
            sdk.create_wallet_from_new_mnemonic(&pin).await
        }
        .await
        .map_err(|err| format!("{:#?}", err))
    }

    /// Creates wallet from mnemonic
    ///
    /// # Arguments
    ///
    /// * `pin` - The PIN for the wallet.
    /// * `mnemonic` - The mnemonic to migrate from.
    ///
    /// # Returns
    ///
    /// * Ok - empty if the wallet is successfully created.
    /// * Err - if there is an error initializing the wallet, initializing the repository, initializing the user.
    pub async fn create_wallet_from_mnemonic(&self, pin: String, mnemonic: String) -> Result<(), String> {
        let mut sdk = self.inner.write().await;
        async move {
            let pin = EncryptionPin::try_from_string(pin)?;
            sdk.create_wallet_from_existing_mnemonic(&pin, &mnemonic).await
        }
        .await
        .map_err(|err| format!("{:#?}", err))
    }

    /// Restores a wallet from backup
    ///
    /// # Arguments
    ///
    /// * `pin` - The PIN for the wallet.
    /// * `backup` - The bytes representing the backup file.
    /// * `backup_password` - The password the backup was created with.
    ///
    /// # Returns
    ///
    /// * Ok - empty if the wallet is successfully created.
    /// * Err - if there is an error initializing the wallet, initializing the repository, initializing the user.
    pub async fn restore_wallet_from_backup(
        &self,
        pin: String,
        backup: Vec<u8>,
        backup_password: String,
    ) -> Result<(), String> {
        let mut sdk = self.inner.write().await;
        async move {
            let pin = EncryptionPin::try_from_string(pin)?;
            let backup_password = PlainPassword::try_from_string(backup_password)?;
            sdk.create_wallet_from_backup(&pin, &backup, &backup_password).await
        }
        .await
        .map_err(|err| format!("{:#?}", err))
    }

    /// Creates a wallet backup
    ///
    /// # Arguments
    ///
    /// * `pin` - The PIN for the wallet.
    /// * `backup_password` - The password to use for the backup.
    ///
    /// # Returns
    ///
    /// * Ok - the bytes representing the backup file if successful.
    /// * Err - if there is an error initializing the wallet.
    pub async fn create_wallet_backup(&self, pin: String, backup_password: String) -> Result<Vec<u8>, String> {
        let mut sdk = self.inner.write().await;
        async move {
            let pin = EncryptionPin::try_from_string(pin)?;
            let password = PlainPassword::try_from_string(backup_password)?;
            sdk.create_wallet_backup(&pin, &password).await
        }
        .await
        .map_err(|err| format!("{:#?}", err))
    }

    /// Deletes an existing wallet
    ///
    /// # Arguments
    ///
    /// * `pin` - The PIN for the wallet.
    ///
    /// # Returns
    ///
    /// * Ok - empty if the wallet is successfully deleted.
    /// * Err - if there is an error initializing the wallet, closing or deleting the wallet.
    pub async fn delete_wallet(&self, pin: String) -> Result<(), String> {
        let mut sdk = self.inner.write().await;
        async move {
            let pin = EncryptionPin::try_from_string(pin)?;
            sdk.delete_wallet(&pin).await
        }
        .await
        .map_err(|err| format!("{:#?}", err))
    }

    /// Generates a new receiver address.
    ///
    /// # Arguments
    ///
    /// * `pin` - The PIN for the wallet.
    ///
    /// # Returns
    ///
    /// * Ok - the generated address as a String if successful.
    /// * Err - if there is an error initializing the wallet or initializing the user.
    pub async fn generate_new_address(&self, pin: String) -> Result<String, String> {
        let mut sdk = self.inner.write().await;
        async move {
            let pin = EncryptionPin::try_from_string(pin)?;
            sdk.generate_new_address(&pin).await
        }
        .await
        .map_err(|err| format!("{:#?}", err))
    }

    /// Fetches the current balance
    ///
    /// # Arguments
    ///
    /// * `pin` - The PIN for the wallet.
    ///
    /// # Returns
    ///
    /// * Ok - the balance as a f64 if successful.
    /// * Err - if there is an error initializing the wallet.
    pub async fn get_balance(&self, pin: String) -> Result<f64, String> {
        let mut sdk = self.inner.write().await;
        async move {
            let pin = EncryptionPin::try_from_string(pin)?;
            sdk.get_balance(&pin).await.and_then(f64::try_from)
        }
        .await
        .map_err(|err| format!("{:#?}", err))
    }

    /// Starts KYC verification with Postident
    ///
    /// Note: This method is only available if the SDK is compiled with support for postident.
    ///
    /// # Returns
    ///
    /// * Ok - the NewCaseIdResponse if successful.
    /// * Err - if there is an error initializing the repository, initializing the user or the user is already KYC verified.
    pub async fn init_kyc_verification_for_postident(&self) -> Result<NewCaseIdResponse, String> {
        sdk::require_feature!("postident", {
            let mut sdk = self.inner.write().await;
            sdk.start_kyc_verification_for_postident()
                .await
                .map(Into::into)
                .map_err(|err| format!("{:#?}", err))
        })
    }

    /// Fetches KYC details for Postident
    ///
    /// Note: This method is only available if the SDK is compiled with support for postident.
    ///
    /// # Returns
    ///
    /// * Ok - the CaseDetailsResponse if successful.
    /// * Err - if there is an error initializing the user.
    pub async fn get_kyc_details_for_postident(&self) -> Result<CaseDetailsResponse, String> {
        sdk::require_feature!("postident", {
            let sdk = self.inner.write().await;
            sdk.get_kyc_details_for_postident()
                .await
                .map(Into::into)
                .map_err(|err| format!("{:#?}", err))
        })
    }

    /// Updates the KYC Details for Postident
    ///
    /// Note: This method is only available if the SDK is compiled with support for postident.
    ///
    /// # Arguments
    ///
    /// * `case_id` - The ID of the case to update.
    ///
    /// # Returns
    ///
    /// * Ok - empty if successful.
    /// * Err - if something went wrong.
    pub async fn update_kyc_details_for_postident(&self, case_id: String) -> Result<(), String> {
        sdk::require_feature!("postident", {
            let sdk = self.inner.write().await;
            sdk.update_kyc_status_for_postident(&case_id)
                .await
                .map_err(|err| format!("{:#?}", err))
        })
    }

    /// Creates a new purchase request
    ///
    /// # Arguments
    ///
    /// * `receiver` - The receiver's username.
    /// * `amount` - The amount of the purchase.
    /// * `product_hash` - The hash of the product.
    /// * `app_data` - The application data.
    /// * `purchase_type` - The type of the purchase.
    ///
    /// # Returns
    ///
    /// * Ok - the purchase ID. This is an internal index used to reference the transaction in etopay
    /// * Err - if the user or wallet is not initialized, or if there is an error creating the transaction.
    pub async fn create_purchase_request(
        &self,
        receiver: String,
        amount: f64,
        product_hash: String,
        app_data: String,
        purchase_type: String,
    ) -> Result<String, String> {
        let sdk = self.inner.write().await;
        async move {
            let amount = CryptoAmount::try_from(amount)?;
            sdk.create_purchase_request(&receiver, amount, &product_hash, &app_data, &purchase_type)
                .await
        }
        .await
        .map_err(|err| format!("{:#?}", err))
    }

    /// Fetches the purchase details
    ///
    /// # Arguments
    ///
    /// * `purchase_id` - The ID of the purchase.
    ///
    /// # Returns
    ///
    /// * Ok - the purchase details if successful.
    /// * Err - if the user or wallet is not initialized, or if there is an error getting the transaction details.
    pub async fn get_purchase_details(&self, purchase_id: String) -> Result<PurchaseDetails, String> {
        let sdk = self.inner.write().await;
        sdk.get_purchase_details(&purchase_id)
            .await
            .and_then(TryInto::try_into)
            .map_err(|err| format!("{:#?}", err))
    }

    /// Confirms the purchase request
    ///
    /// # Arguments
    ///
    /// * `purchase_id` - The ID of the purchase.
    /// * `pin` - The PIN of the user.
    ///
    /// # Returns
    ///
    /// * Ok - if the purchase request is confirmed successfully.
    /// * Err -  if the user or wallet is not initialized, if there is an error verifying the PIN, if there is an error getting the transaction details,
    ///   or if there is an error committing the transaction.
    pub async fn confirm_purchase_request(&self, pin: String, purchase_id: String) -> Result<(), String> {
        let mut sdk = self.inner.write().await;
        async move {
            let pin = EncryptionPin::try_from_string(pin)?;
            sdk.confirm_purchase_request(&pin, &purchase_id).await
        }
        .await
        .map_err(|err| format!("{:#?}", err))
    }

    /// Starts KYC verification with viviswap
    ///
    /// # Arguments
    ///
    /// * `mail` - The email address of the user.
    /// * `terms_accepted` - A boolean indicating whether the terms have been accepted.
    ///
    /// # Returns
    ///
    /// * Ok - the NewViviswapUser if successful.
    /// * Err - if there is a repository initialization error, user already exists, viviswap API error, user status update error.
    #[cfg_attr(not(feature = "viviswap-kyc"), allow(unused_variables))]
    pub async fn start_kyc_verification_for_viviswap(
        &self,
        mail: String,
        terms_accepted: bool,
    ) -> Result<NewViviswapUser, String> {
        sdk::require_feature!("viviswap-kyc", {
            let mut sdk = self.inner.write().await;
            sdk.start_kyc_verification_for_viviswap(&mail, terms_accepted)
                .await
                .map(Into::into)
                .map_err(|err| format!("{:#?}", err))
        })
    }

    /// Fetches the KYC status in viviswap
    ///
    /// # Arguments
    ///
    /// * `None`
    ///
    /// # Returns
    ///
    /// * Ok - the ViviswapKycStatus if successful.
    /// * Err -  if there is a repository initialization error, viviswap API error.
    pub async fn get_kyc_details_for_viviswap(&self) -> Result<ViviswapKycStatus, String> {
        sdk::require_feature!("viviswap-kyc", {
            let mut sdk = self.inner.write().await;
            sdk.get_kyc_details_for_viviswap()
                .await
                .map(Into::into)
                .map_err(|err| format!("{:#?}", err))
        })
    }

    /// Updates the viviswap partial kyc
    ///
    /// # Arguments
    ///
    /// * `is_individual` - Whether the user is an individual. Optional.
    /// * `is_pep` - Whether the user is a politically exposed person. Optional.
    /// * `is_us_citizen` - Whether the user is a US citizen. Optional.
    /// * `is_regulatory_disclosure` - Whether the user has accepted the regulatory disclosure. Optional.
    /// * `country_of_residence` - The country of residence of the user. Optional.
    /// * `nationality` - The nationality of the user. Optional.
    /// * `full_name` - The full name of the user. Optional.
    /// * `date_of_birth` - The date of birth of the user. Optional.
    ///
    /// # Returns
    ///
    /// * Ok - containing the partially updated KYC details.
    /// * Err - vector of errors if any validation errors occur during the update process.
    #[allow(clippy::too_many_arguments)]
    #[cfg_attr(not(feature = "viviswap-kyc"), allow(unused_variables))]
    pub async fn update_kyc_partially_status_for_viviswap(
        &self,
        is_individual: Option<bool>,
        is_pep: Option<bool>,
        is_us_citizen: Option<bool>,
        is_regulatory_disclosure: Option<bool>,
        country_of_residence: Option<String>,
        nationality: Option<String>,
        full_name: Option<String>,
        date_of_birth: Option<String>,
    ) -> Result<ViviswapPartiallyKycDetails, String> {
        sdk::require_feature!("viviswap-kyc", {
            let mut sdk = self.inner.write().await;
            sdk.update_kyc_partially_status_for_viviswap(
                is_individual,
                is_pep,
                is_us_citizen,
                is_regulatory_disclosure,
                country_of_residence,
                nationality,
                full_name,
                date_of_birth,
            )
            .await
            .map(Into::into)
            .map_err(|err| format!("{:#?}", err))
        })
    }

    /// Submits partial KYC Status for viviswap
    ///
    /// # Arguments
    ///
    /// * `None`
    ///
    /// # Returns
    ///
    /// * Ok - empty if the submission is successful.
    /// * Err - if there is a repository initialization error, viviswap missing user error, viviswap invalid state error, viviswap missing field error,
    ///   viviswap API error.
    pub async fn submit_kyc_partially_status_for_viviswap(&self) -> Result<(), String> {
        sdk::require_feature!("viviswap-kyc", {
            let mut sdk = self.inner.write().await;
            sdk.submit_kyc_partially_status_for_viviswap()
                .await
                .map_err(|err| format!("{:#?}", err))
        })
    }

    /// Sets Viviswap KYC identity details
    ///
    /// # Arguments
    ///
    /// * `identity_official_document_data` - IdentityOfficialDocumentData
    /// * `identity_personal_document_data` - IdentityPersonalDocumentData
    ///
    /// # Returns
    ///
    /// * Ok - empty if setting the identity details is successful.
    /// * Err - if there is a repository initialization error, viviswap API error.
    #[cfg_attr(not(feature = "viviswap-kyc"), allow(unused_variables))]
    pub async fn set_viviswap_kyc_identity_details(
        &self,
        identity_official_document_data: IdentityOfficialDocumentData,
        identity_personal_document_data: IdentityPersonalDocumentData,
    ) -> Result<(), String> {
        sdk::require_feature!("viviswap-kyc", {
            let sdk = self.inner.write().await;
            sdk.set_viviswap_kyc_identity_details(
                identity_official_document_data.into(),
                identity_personal_document_data.into(),
            )
            .await
            .map_err(|err| format!("{:#?}", err))
        })
    }

    /// Sets Viviswap KYC residence details
    ///
    /// # Arguments
    ///
    /// * `country_code`, `region`, `zip_code`, `city`, `address_line_1`, `address_line_2` - basic address data.
    /// * `is_public_entry` - Inidcates that a valid public entry of this clients address can be found.
    /// * `public_entry_reference` - if `is_public_entry` is `true`, then this must contain the resource link.
    /// * `has_no_official_document` - indicates if the client does not have any document verifying their address.
    /// * `official_document` - Option<File> type that requires the image data and file name.
    ///
    /// # Returns
    ///
    /// * Ok - empty if setting the residence details is successful.
    /// * Err - if there is a repository initialization error, input values are not valid, viviswap API error.
    #[allow(clippy::too_many_arguments)]
    #[cfg_attr(not(feature = "viviswap-kyc"), allow(unused_variables))]
    pub async fn set_viviswap_kyc_residence_details(
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
    ) -> Result<(), String> {
        sdk::require_feature!("viviswap-kyc", {
            let sdk = self.inner.write().await;
            let official_document = official_document.map(|doc| doc.into());
            sdk.set_viviswap_kyc_residence_details(
                country_code,
                region,
                zip_code,
                city,
                address_line_1,
                address_line_2,
                is_public_entry,
                public_entry_reference,
                has_no_official_document,
                official_document,
            )
            .await
            .map_err(|err| format!("{:#?}", err))
        })
    }

    /// Gets the open AMLA KYC questions
    ///
    /// # Arguments
    ///
    /// * `None`
    ///
    /// # Returns
    ///
    /// * Ok - a list of the currently open AMLA questions.
    /// * Err - if the user is not initialized or viviswap API error.
    pub async fn get_viviswap_kyc_amla_open_questions(&self) -> Result<Vec<KycAmlaQuestion>, String> {
        sdk::require_feature!("viviswap-kyc", {
            let sdk = self.inner.write().await;
            match sdk.get_viviswap_kyc_amla_open_questions().await {
                Ok(kyc_amla_questions) => {
                    let questions = kyc_amla_questions.into_iter().map(|q| q.into()).collect();
                    Ok(questions)
                }
                Err(err) => Err(format!("{:#?}", err)),
            }
        })
    }

    /// Gets the currently open/missing documents for KYC
    ///
    /// # Arguments
    ///
    /// * `None`
    ///
    /// # Returns
    ///
    /// * Ok - a list of the currently open documents.
    /// * Err - if the user is not initialized or viviswap API error.
    pub async fn get_viviswap_kyc_open_documents(&self) -> Result<Vec<KycOpenDocument>, String> {
        sdk::require_feature!("viviswap-kyc", {
            let sdk = self.inner.write().await;
            match sdk.get_viviswap_kyc_open_documents().await {
                Ok(kyc_open_documents) => {
                    let documents = kyc_open_documents.into_iter().map(|d| d.into()).collect();
                    Ok(documents)
                }
                Err(err) => Err(format!("{:#?}", err)),
            }
        })
    }

    /// Sets / uploads an open KYC document
    ///
    /// # Arguments
    ///
    /// - `document_id` - The ID of the document to upload.
    /// - `expiration_date` - the expiration date of this document.
    /// - `document_number` - the official document number.
    /// * `front_image` - Option<File> type which needs front image and file name.
    /// * `back_image` - Option<File> type which needs back image and file name.
    ///
    /// # Returns
    ///
    /// * Ok - empty if setting the viviswap kyc document is successful.
    /// * Err - if the user is not initialized or viviswap API error.
    #[cfg_attr(not(feature = "viviswap-kyc"), allow(unused_variables))]
    pub async fn set_viviswap_kyc_document(
        &self,
        document_id: String,
        expiration_date: String,
        document_number: String,
        front_image: Option<File>,
        back_image: Option<File>,
    ) -> Result<(), String> {
        sdk::require_feature!("viviswap-kyc", {
            let sdk = self.inner.write().await;

            let front_image = front_image.map(|doc| doc.into());
            let back_image = back_image.map(|doc| doc.into());

            sdk.set_viviswap_kyc_document(document_id, expiration_date, document_number, front_image, back_image)
                .await
                .map_err(|err| format!("{:#?}", err))
        })
    }

    /// Sets the answer to an open AMLA KYC question
    ///
    /// # Arguments
    ///
    /// - `question_id` - The ID of the question to set the answer to.
    /// - `answers` - a list of the selected available answers for the question.
    /// - `freetext_answer` - an optional free-text answer.
    ///
    /// # Returns
    ///
    /// * Ok - empty if setting the viviswap kyc amla answer is successful.
    /// * Err - if the user is not initialized or viviswap API error.
    #[cfg_attr(not(feature = "viviswap-kyc"), allow(unused_variables))]
    pub async fn set_viviswap_kyc_amla_answer(
        &self,
        question_id: String,
        answers: Vec<String>,
        freetext_answer: Option<String>,
    ) -> Result<(), String> {
        sdk::require_feature!("viviswap-kyc", {
            let sdk = self.inner.write().await;
            sdk.set_viviswap_kyc_amla_answer(question_id, answers, freetext_answer)
                .await
                .map_err(|err| format!("{:#?}", err))
        })
    }

    /// Verifies an existing pin
    ///
    /// # Arguments
    ///
    /// * `pin` - The pin to verify.
    ///
    /// # Returns
    ///
    /// * Ok - empty if the pin is verified successfully.
    /// * Err - if there is an error for initializing the repository, initializing the user, initializing the wallet, password is missing,
    ///   pin or password is incorrect.
    pub async fn verify_pin(&self, pin: String) -> Result<(), String> {
        let sdk = self.inner.write().await;
        async move {
            let pin = EncryptionPin::try_from_string(pin)?;
            sdk.verify_pin(&pin).await
        }
        .await
        .map_err(|err| format!("{:#?}", err))
    }

    /// Resets the wallet pin
    ///
    /// # Arguments
    ///
    /// * `pin` - The current pin for the wallet.
    /// * `new_pin` - The new pin to set for the wallet.
    ///
    /// # Returns
    ///
    /// * Ok - empty if the pin is reset successfully.
    /// * Err - if there is an error for initializing the repository, initializing the user, password is missing,
    ///   pin or password is incorrect.
    pub async fn reset_pin(&self, pin: String, new_pin: String) -> Result<(), String> {
        let mut sdk = self.inner.write().await;
        async move {
            let pin = EncryptionPin::try_from_string(pin)?;
            let new_pin = EncryptionPin::try_from_string(new_pin)?;
            sdk.change_pin(&pin, &new_pin).await
        }
        .await
        .map_err(|err| format!("{:#?}", err))
    }

    /// Set the password to use for wallet operations. If the password was already set, this changes it.
    ///
    /// # Arguments
    ///
    /// * `pin` - The pin to verify.
    /// * `new_password` - The new password to set for the wallet.
    ///
    /// # Returns
    ///
    /// * Ok - empty if the password is changed successfully.
    /// * Err - if there is an error for initializing the repository, initializing the user, initializing the wallet.
    pub async fn set_wallet_password(&self, pin: String, new_password: String) -> Result<(), String> {
        let mut sdk = self.inner.write().await;
        async move {
            let pin = EncryptionPin::try_from_string(pin)?;
            let new_password = PlainPassword::try_from_string(new_password)?;
            sdk.set_wallet_password(&pin, &new_password).await
        }
        .await
        .map_err(|err| format!("{:#?}", err))
    }

    /// Check if the password to use for wallet operations is set.
    /// Use [`set_wallet_password`] to set a new or change an existing password.
    ///
    /// # Returns
    ///
    /// Whether the password is already set or not.
    pub async fn is_wallet_password_set(&self) -> Result<bool, String> {
        let sdk = self.inner.read().await;
        sdk.is_wallet_password_set().await.map_err(|err| format!("{:#?}", err))
    }

    /// Sends amount from wallet
    ///
    /// # Arguments
    ///
    /// * `pin` - The PIN of the user.
    /// * `address` - The receiver's address.
    /// * `amount` - The amount to send.
    /// * `data` - The associated data with the transaction.
    ///
    /// # Returns
    ///
    /// * Ok - transaction id if the amount is sent successfully.
    /// * Err - if the user or wallet is not initialized, there is an error verifying the PIN, or there is an error sending the amount.
    pub async fn send_amount(
        &self,
        pin: String,
        address: String,
        amount: f64,
        data: Option<Vec<u8>>,
    ) -> Result<String, String> {
        let mut sdk = self.inner.write().await;
        async move {
            let amount = CryptoAmount::try_from(amount)?;
            let pin = EncryptionPin::try_from_string(pin)?;
            sdk.send_amount(&pin, &address, amount, data).await
        }
        .await
        .map_err(|err| format!("{:#?}", err))
    }

    /// Updates IBAN in SDK
    ///
    /// # Arguments
    ///
    /// * `pin` - The user's PIN.
    /// * `address` - The new IBAN address.
    ///
    /// # Returns
    ///
    /// * Ok - the updated Viviswap address detail.
    /// * Err - if repository initialization fails, viviswap user is missing, error updating the user status.
    #[cfg_attr(not(feature = "viviswap-swap"), allow(unused_variables))]
    pub async fn update_iban_viviswap(&self, pin: String, address: String) -> Result<ViviswapAddressDetail, String> {
        sdk::require_feature!("viviswap-swap", {
            let mut sdk = self.inner.write().await;

            async move {
                let pin = EncryptionPin::try_from_string(pin)?;
                sdk.update_iban_for_viviswap(&pin, address).await
            }
            .await
            .map(Into::into)
            .map_err(|err| format!("{:#?}", err))
        })
    }

    /// Gets IBAN from SDK
    ///
    /// # Arguments
    ///
    /// * `None`
    ///
    /// # Returns
    ///
    /// * Ok - the current IBAN of the viviswap user.
    /// * Err - if the viviswap state is invalid, repository initialization fails, error in the viviswap API, error updating the user status.
    pub async fn get_iban_viviswap(&self) -> Result<ViviswapAddressDetail, String> {
        sdk::require_feature!("viviswap-swap", {
            let mut sdk = self.inner.write().await;
            sdk.get_iban_for_viviswap()
                .await
                .map(Into::into)
                .map_err(|err| format!("{:#?}", err))
        })
    }

    /// Creates deposit with viviswap
    ///
    /// # Arguments
    ///
    /// * `pin` - The current pin for the wallet.
    ///
    /// # Returns
    ///
    /// * Ok - the created Viviswap deposit.
    /// * Err - if the viviswap state is invalid, repository initialization fails, error in the viviswap API, viviswap user is missing.
    pub async fn deposit_with_viviswap(&self, pin: String) -> Result<ViviswapDeposit, String> {
        sdk::require_feature!("viviswap-swap", {
            let mut sdk = self.inner.write().await;
            async move {
                let pin = EncryptionPin::try_from_string(pin)?;
                sdk.create_deposit_with_viviswap(&pin).await
            }
            .await
            .map(Into::into)
            .map_err(|err| format!("{:#?}", err))
        })
    }

    /// Creates detail with viviswap
    ///
    /// # Arguments
    ///
    /// * `pin` - The current pin for the wallet.
    ///
    /// # Returns
    ///
    /// * Ok - the created Viviswap address detail.
    /// * Err - if fails initializing the configuration, viviswap user is missing.
    pub async fn create_detail_viviswap(&self, pin: String) -> Result<ViviswapAddressDetail, String> {
        sdk::require_feature!("viviswap-swap", {
            let mut sdk = self.inner.write().await;
            async move {
                let pin = EncryptionPin::try_from_string(pin)?;
                sdk.create_detail_for_viviswap(&pin).await
            }
            .await
            .map(Into::into)
            .map_err(|err| format!("{:#?}", err))
        })
    }

    /// Creates withdraw with viviswap
    ///
    /// # Arguments
    ///
    /// * `amount` - The amount of the withdrawal.
    /// * `pin` - The optional PIN for verification.
    /// * `data` - The associated data for the transaction.
    ///
    /// # Returns
    ///
    /// * Ok - the created Viviswap withdrawal.
    /// * Err - if viviswap state is invalid, viviswap user is missing, viviswap API error.
    #[cfg_attr(not(feature = "viviswap-swap"), allow(unused_variables))]
    pub async fn withdraw_with_viviswap(
        &self,
        amount: f64,
        pin: Option<String>,
        data: Option<Vec<u8>>,
    ) -> Result<ViviswapWithdrawal, String> {
        sdk::require_feature!("viviswap-swap", {
            let mut sdk = self.inner.write().await;
            async move {
                let amount = CryptoAmount::try_from(amount)?;
                let pin = match pin {
                    Some(pin) => Some(EncryptionPin::try_from_string(pin)?),
                    None => None,
                };
                sdk.create_withdrawal_with_viviswap(amount, pin.as_ref(), data).await
            }
            .await
            .map(Into::into)
            .map_err(|err| format!("{:#?}", err))
        })
    }

    /// Retrieves details for a specific swap(order in viviswap)
    ///
    /// # Arguments
    ///
    /// * `order_id` - The ID of the swap order.
    ///
    /// # Returns
    ///
    /// * Ok - the swap order details.
    /// * Err - if something went wrong.
    #[cfg_attr(not(feature = "viviswap-swap"), allow(unused_variables))]
    pub async fn get_swap_details(&self, order_id: String) -> Result<Order, String> {
        sdk::require_feature!("viviswap-swap", {
            let sdk = self.inner.write().await;
            sdk.get_swap_details(order_id)
                .await
                .map(Into::into)
                .map_err(|err| format!("{:#?}", err))
        })
    }

    /// Retrieves the list of all the swaps(viviswap orders) performed by a user
    ///
    /// # Arguments
    ///
    /// * `start` - The start page.
    /// * `limit` - The limit per page.
    ///
    /// # Returns
    ///
    /// * Ok - vector of Swaps if successful.
    /// * Err - if repository initialization error, viviswap API error.
    #[cfg_attr(not(feature = "viviswap-swap"), allow(unused_variables))]
    pub async fn get_swap_list(&self, start: u32, limit: u32) -> Result<Vec<Order>, String> {
        sdk::require_feature!("viviswap-swap", {
            let sdk = self.inner.write().await;
            match sdk.get_swap_list(start, limit).await {
                Ok(order_list) => {
                    let orders = order_list.orders.into_iter().map(|o| o.into()).collect();
                    Ok(orders)
                }
                Err(err) => Err(format!("{:#?}", err)),
            }
        })
    }

    /// Gets exchange rate from SDK
    ///
    /// # Arguments
    ///
    /// * `None`
    ///
    /// # Returns
    ///
    /// * Ok - the exchange rate as f32 if successful.
    /// * Err - if viviswap API error.
    pub async fn get_exchange_rate(&self) -> Result<f64, String> {
        let sdk = self.inner.write().await;
        sdk.get_exchange_rate()
            .await
            .and_then(|v| Ok(f64::try_from(v)?))
            .map_err(|err| format!("{:#?}", err))
    }

    /// Creates withdraw with viviswap
    ///
    /// # Arguments
    ///
    /// * `start` - The start page.
    /// * `limit` - The limit per page.
    ///
    /// # Returns
    ///
    /// * Ok - TxList if successful.
    /// * Err - if there is a problem getting the list of transactions.
    pub async fn get_transaction_list(&self, start: u32, limit: u32) -> Result<Vec<TxInfo>, String> {
        let sdk = self.inner.write().await;
        match sdk.get_tx_list(start, limit).await {
            Ok(tx_list) => {
                let txs = tx_list.txs.into_iter().map(|tx| tx.into()).collect();
                Ok(txs)
            }
            Err(err) => Err(format!("{:#?}", err)),
        }
    }

    /// Deletes the user
    ///
    /// # Arguments
    ///
    /// * `pin` - The wallet pin for confirmation. Optional in case there is an active wallet.
    ///
    /// # Returns
    ///
    /// * Ok - empty if the user is deleted successfully.
    /// * Err - if there is an issue verifying the PIN, initializing the repository, deleting the user, or deleting the wallet.
    pub async fn delete_user(&self, pin: Option<String>) -> Result<(), String> {
        let mut sdk = self.inner.write().await;
        async move {
            let encryption_pin = match pin {
                Some(p) => Some(EncryptionPin::try_from_string(p)?),
                None => None,
            };
            sdk.delete_user(encryption_pin.as_ref()).await
        }
        .await
        .map_err(|err| format!("{:#?}", err))
    }

    /// Returns wallet transaction list
    ///
    /// # Arguments
    ///
    /// * `pin` - The current pin for the wallet.
    /// * `start` - The start page.
    /// * `limit` - The limit per page.
    ///
    /// # Returns
    ///
    /// * Ok - wallet transaction list if successful.
    /// * Err - if there is a problem getting the wallet list of transactions.
    pub async fn get_wallet_transaction_list(
        &self,
        pin: String,
        start: usize,
        limit: usize,
    ) -> Result<Vec<WalletTxInfo>, String> {
        let mut sdk = self.inner.write().await;
        async move {
            let pin = EncryptionPin::try_from_string(pin)?;
            sdk.get_wallet_tx_list(&pin, start, limit)
                .await
                .map(|l| l.transactions.into_iter().map(|tx| tx.into()).collect())
        }
        .await
        .map_err(|err| format!("{:#?}", err))
    }

    /// Fetches a wallet transaction.
    ///
    /// # Arguments
    ///
    /// * `pin` - The current pin for the wallet.
    /// * `transaction_id` - The ID of the transaction to get details for.
    ///
    /// # Returns
    ///
    /// * Ok - the details of the wallet transaction.
    /// * Err - if there is a problem getting the wallet transaction details.
    pub async fn get_wallet_transaction(&self, pin: String, transaction_id: String) -> Result<WalletTxInfo, String> {
        let mut sdk = self.inner.write().await;
        async move {
            let pin = EncryptionPin::try_from_string(pin)?;
            sdk.get_wallet_tx(&pin, &transaction_id).await
        }
        .await
        .map(Into::into)
        .map_err(|err| format!("{:#?}", err))
    }

    /// Get the recovery share.
    ///
    /// # Returns
    ///
    /// * The recovery share as a string, or an empty string if no recovery share exists.
    /// * Err - if the user is not initialized.
    pub async fn get_recovery_share(&self) -> Result<String, String> {
        use sdk::secrecy::ExposeSecret;

        let sdk = self.inner.write().await;
        sdk.get_recovery_share()
            .await
            .map(|s| s.map(|s| s.to_string().expose_secret().to_string()).unwrap_or_default())
            .map_err(|err| format!("{:#?}", err))
    }

    /// Set the recovery share.
    ///
    /// # Arguments
    ///
    /// * `share` - The recovery share to set.
    ///
    /// # Returns
    ///
    /// * Ok - if conversion went well.
    /// * Err - if the share has the wrong format or the user is not initialized.
    pub async fn set_recovery_share(&self, share: String) -> Result<(), String> {
        let mut sdk = self.inner.write().await;

        let share: sdk::share::Share = share.parse().map_err(|e| format!("{e:#?}"))?;
        sdk.set_recovery_share(share).await.map_err(|err| format!("{:#?}", err))
    }

    /// Get the user's preferred network.
    ///
    /// # Returns
    ///
    /// * Ok - the preferred network, or `None` if it has not been set.
    /// * Err - if there was an error contacting the backend.
    pub async fn get_preferred_network(&self) -> Result<String, String> {
        let sdk = self.inner.write().await;
        let result = sdk.get_preferred_network().await;
        match result {
            Ok(network) => Ok(network.unwrap_or_default()),
            Err(err) => Err(format!("{:#?}", err)),
        }
    }

    /// Set the user's preferred network.
    ///
    /// # Arguments
    ///
    /// * `network_key` - The preferred network, or `None` if it should be unset.
    ///
    /// # Returns
    ///
    /// * Ok - if setting the preferred network was successful.
    /// * Err - if there was an error contacting the backend.
    pub async fn set_preferred_network(&self, network_key: Option<String>) -> Result<(), String> {
        let mut sdk = self.inner.write().await;
        sdk.set_preferred_network(network_key)
            .await
            .map_err(|err| format!("{:#?}", err))
    }

    /// Get sdk build information.
    ///
    /// # Returns
    ///
    /// The sdk build information as a string.
    pub fn get_build_info(&self) -> String {
        Sdk::get_build_info()
    }
}

pub struct KycOpenDocument {
    pub id: String,
    pub is_back_image_required: bool,
    pub document_type: String,
    pub description: String,
}

impl KycOpenDocument {
    pub fn id(&self) -> String {
        self.id.clone()
    }
    pub fn is_back_image_required(&self) -> bool {
        self.is_back_image_required
    }
    pub fn document_type(&self) -> String {
        self.document_type.clone()
    }
    pub fn description(&self) -> String {
        self.description.clone()
    }
}

pub struct KycAmlaQuestion {
    pub id: String,
    pub question: String,
    pub possible_answers: Vec<String>,
    pub is_free_text: bool,
    pub min_answers: i32,
    pub max_answers: i32,
}

impl KycAmlaQuestion {
    pub fn id(&self) -> String {
        self.id.clone()
    }
    pub fn is_free_text(&self) -> bool {
        self.is_free_text
    }
    pub fn question(&self) -> String {
        self.question.clone()
    }
    pub fn min_answers(&self) -> i32 {
        self.min_answers
    }
    pub fn max_answers(&self) -> i32 {
        self.max_answers
    }
    pub fn possible_answers(&self) -> Vec<String> {
        self.possible_answers.clone()
    }
}

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
    pub refund_payment_method_id: String,
    pub status: i32,
    pub creation_date: String,
    pub incoming_payment_detail: String,
    pub outgoing_payment_detail: String,
    pub refund_payment_detail: String,
}

impl Order {
    pub fn id(&self) -> String {
        self.id.clone()
    }

    pub fn is_payed_out(&self) -> bool {
        self.is_payed_out
    }

    pub fn is_approved(&self) -> bool {
        self.is_approved
    }

    pub fn is_canceled(&self) -> bool {
        self.is_canceled
    }

    pub fn fees_amount_eur(&self) -> f32 {
        self.fees_amount_eur
    }

    pub fn crypto_fees(&self) -> f32 {
        self.crypto_fees
    }

    pub fn contract_id(&self) -> String {
        self.contract_id.clone()
    }

    pub fn incoming_payment_method_id(&self) -> String {
        self.incoming_payment_method_id.clone()
    }

    pub fn incoming_payment_method_currency(&self) -> String {
        self.incoming_payment_method_currency.clone()
    }

    pub fn incoming_amount(&self) -> f32 {
        self.incoming_amount
    }

    pub fn incoming_course(&self) -> f32 {
        self.incoming_course
    }

    pub fn outgoing_payment_method_id(&self) -> String {
        self.outgoing_payment_method_id.clone()
    }

    pub fn outgoing_payment_method_currency(&self) -> String {
        self.outgoing_payment_method_currency.clone()
    }

    pub fn outgoing_amount(&self) -> f32 {
        self.outgoing_amount
    }

    pub fn outgoing_course(&self) -> f32 {
        self.outgoing_course
    }

    pub fn refund_amount(&self) -> Option<f32> {
        self.refund_amount
    }

    pub fn refund_course(&self) -> Option<f32> {
        self.refund_course
    }

    pub fn refund_payment_method_id(&self) -> String {
        self.refund_payment_method_id.clone()
    }

    pub fn status(&self) -> i32 {
        self.status
    }

    pub fn creation_date(&self) -> String {
        self.creation_date.clone()
    }

    pub fn incoming_payment_detail(&self) -> String {
        self.incoming_payment_detail.clone()
    }

    pub fn outgoing_payment_detail(&self) -> String {
        self.outgoing_payment_detail.clone()
    }

    pub fn refund_payment_detail(&self) -> String {
        self.refund_payment_detail.clone()
    }
}

pub struct TxInfo {
    pub date: String,
    pub sender: String,
    pub receiver: String,
    pub reference_id: String,
    pub application_metadata: String,
    pub amount: f64,
    pub currency: String,
    pub status: TxStatus,
    pub transaction_hash: String,
    pub course: f64,
    pub invalid_reasons: Vec<String>,
}

impl TxInfo {
    pub fn date(&self) -> String {
        self.date.clone()
    }

    pub fn sender(&self) -> String {
        self.sender.clone()
    }

    pub fn receiver(&self) -> String {
        self.receiver.clone()
    }

    pub fn reference_id(&self) -> String {
        self.reference_id.clone()
    }

    pub fn application_metadata(&self) -> String {
        self.application_metadata.clone()
    }

    pub fn amount(&self) -> f64 {
        self.amount
    }

    pub fn currency(&self) -> String {
        self.currency.clone()
    }

    pub fn status(&self) -> TxStatus {
        self.status
    }

    pub fn transaction_hash(&self) -> String {
        self.transaction_hash.clone()
    }

    pub fn course(&self) -> f64 {
        self.course
    }

    pub fn invalid_reasons(&self) -> Vec<String> {
        self.invalid_reasons.clone()
    }
}

pub struct WalletTxInfo {
    pub date: String,
    pub block_id: String,
    pub transaction_id: String,
    pub receiver: String,
    pub incoming: bool,
    pub amount: f64,
    pub network: String,
    pub status: String,
    pub explorer_url: String,
}

impl WalletTxInfo {
    pub fn date(&self) -> String {
        self.date.clone()
    }

    pub fn block_id(&self) -> String {
        self.block_id.clone()
    }

    pub fn transaction_id(&self) -> String {
        self.transaction_id.clone()
    }

    pub fn receiver(&self) -> String {
        self.receiver.clone()
    }

    pub fn incoming(&self) -> bool {
        self.incoming
    }

    pub fn amount(&self) -> f64 {
        self.amount
    }

    pub fn network(&self) -> String {
        self.network.clone()
    }

    pub fn status(&self) -> String {
        self.status.clone()
    }

    pub fn explorer_url(&self) -> String {
        self.explorer_url.clone()
    }
}

pub struct Network {
    pub key: String,
    pub display_name: String,
}

impl Network {
    pub fn key(&self) -> String {
        self.key.clone()
    }

    pub fn display_name(&self) -> String {
        self.display_name.clone()
    }
}
