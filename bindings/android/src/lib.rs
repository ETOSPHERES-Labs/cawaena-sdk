//! This file contains the Rust implementation of the Android SDK bindings for the ETOPay library.
//!
//! It provides functions that interface with the Java code and handle the communication with the ETOPay SDK.
//! The functions in this file are called from the Java code using JNI (Java Native Interface).
//! The Rust code uses the `jni` crate to interact with the Java code and the `sdk` crate to access the ETOPay SDK functionality.
//! The Rust code also uses other crates such as `once_cell`, `tokio`, and `serde_json` for various purposes.
//! The functions in this file handle tasks such as setting up the SDK, initializing users, performing transactions, and managing wallets.
//!
//! Error handling is done by throwing exceptions back to the Java code.
//! The Rust code uses asynchronous programming with the help of the `tokio` runtime.
//! The main data structure used in this file is `SdkWrapper`, which is an `Arc<RwLock<Sdk>>` type.
//! This allows for concurrent access to the SDK from multiple threads.
//! The Rust code also defines various helper macros for string conversion and error handling.
//!
//! Overall, this file serves as the bridge between the Java code and the ETOPay SDK, enabling seamless integration of the SDK into Android applications.

mod type_conversions;

#[cfg(feature = "viviswap-kyc")]
use sdk::types::File;

use once_cell::sync::OnceCell;
use sdk::core::Sdk;
use std::sync::Arc;
use tokio::{runtime::Runtime, sync::RwLock};

#[doc = r"Sdk handle with atomic reference count and read write lock"]
type SdkWrapper = Arc<RwLock<Sdk>>;

/// Returns or creates a reference to the tokio runtime.
/// The runtime is lazily initialized using the `OnceCell` pattern.
///
/// # Returns
///
/// The runtime thread wrapped with atomic reference counter as a static reference
#[allow(clippy::unwrap_used)]
fn runtime() -> &'static Arc<Runtime> {
    static INSTANCE: OnceCell<Arc<Runtime>> = OnceCell::new();
    INSTANCE.get_or_init(|| {
        Arc::new(
            tokio::runtime::Builder::new_multi_thread()
                .thread_name("com.standalone.sdk.thread")
                .worker_threads(1)
                .enable_all()
                .build()
                .unwrap(),
        )
    })
}

/// Returns or creates a reference to the sdk object
/// The runtime is lazily initialized using the `OnceCell` pattern.
///
/// # Returns
///
/// The sdk handle with atomic reference counter as a static reference
fn get_or_init_sdk() -> &'static SdkWrapper {
    static CELL: OnceCell<SdkWrapper> = OnceCell::new();
    let sdk = CELL.get_or_init(|| {
        let sdk_user = Sdk::default();
        Arc::new(RwLock::new(sdk_user))
    });
    sdk
}

/// Main object that contains all the functionality for interfacing with the ETOPaySdk.
#[jnigen_macro::generate("com.etospheres.etopay.ETOPaySdk")]
mod ffi {
    use super::*;
    use sdk::{
        core::Config,
        share::Share,
        types::{
            currencies::CryptoAmount,
            newtypes::{AccessToken, EncryptionPin, PlainPassword},
        },
        WalletError,
    };
    use type_conversions::PurchaseDetailsEntity;

    /// Set the configuration as a JSON-encoded string.
    ///
    /// @param config The input string representing the configuration with the following structure:
    /// <pre>
    /// {@code
    /// {
    ///     "auth_provider": "<authentication provider name>",
    ///     "backend_url": "<valid URL to the backend API>",
    ///     "storage_path": "/path/to/valid/folder",
    ///     "log_level": "info",
    /// }
    /// }
    /// </pre>
    ///
    #[public_name = "setConfig"]
    pub fn setConfig(config: String) -> Result<(), String> {
        let result = runtime().block_on(async move {
            let mut sdk = get_or_init_sdk().write().await;
            sdk.set_config(Config::from_json(&config)?)
        });
        result.map_err(|e| format!("{e:#?}"))
    }

    /// Fetch available currencies and corresponding node urls.
    ///
    /// @return Serialized string of a hashmap with currencies as key and node urls as value
    pub fn getNetworks() -> Result<String, String> {
        let result = runtime().block_on(async move {
            let mut sdk = get_or_init_sdk().write().await;
            sdk.get_networks().await
        });

        match result {
            Ok(value) => serde_json::to_string(&value).map_err(|e| format!("{e:#?}")),
            Err(e) => Err(format!("{e:#?}")),
        }
    }

    /// Selects the network for the ETOPay SDK.
    ///
    /// @param network_id The input string representing the network id.
    pub fn setNetwork(network_id: String) -> Result<(), String> {
        let result = runtime().block_on(async move {
            let mut sdk = get_or_init_sdk().write().await;
            sdk.set_network(network_id).await
        });
        result.map_err(|e| format!("{e:#?}"))
    }

    /// Destructor for the SDK handle
    #[public_name = "close"]
    pub fn destroy() {
        runtime().block_on(async move {
            let mut sdk = get_or_init_sdk().write().await;
            let sdk = std::mem::take(&mut *sdk);
            drop(sdk);
        });
    }

    /// Creates a new user for the SDK.
    ///
    /// @param username The input string representing the username.
    pub fn createNewUser(username: String) -> Result<(), String> {
        let result = runtime().block_on(async move {
            let mut sdk = get_or_init_sdk().write().await;
            sdk.create_new_user(&username).await
        });
        result.map_err(|e| format!("{e:#?}"))
    }

    /// Initializes an existing user in the SDK
    ///
    /// @param username The input string representing the username.
    pub fn initializeUser(username: String) -> Result<(), String> {
        let result = runtime().block_on(async move {
            let mut sdk = get_or_init_sdk().write().await;
            sdk.init_user(&username).await
        });

        result.map_err(|e| format!("{e:#?}"))
    }

    /// Refreshes the access token for the user in the SDK.
    ///
    /// @param access_token The input string representing the access token.
    pub fn refreshAccessToken(access_token: String) -> Result<(), String> {
        let result = runtime().block_on(async move {
            let mut sdk = get_or_init_sdk().write().await;

            let access_token = if access_token.is_empty() {
                None
            } else {
                Some(AccessToken::try_from(access_token)?)
            };
            sdk.refresh_access_token(access_token).await
        });
        result.map_err(|e| format!("{e:#?}"))
    }

    /// Fetches the kyc verification status for the user
    ///
    /// @param username The input string representing the username.
    ///
    /// @return The kyc verification status as a boolean value
    pub fn isKycVerified(username: String) -> Result<bool, String> {
        let result = runtime().block_on(async move {
            let mut sdk = get_or_init_sdk().write().await;
            sdk.is_kyc_status_verified(&username).await
        });

        result.map_err(|e| format!("{e:#?}"))
    }

    /// Verifies the mnemonic for the wallet
    ///
    /// @param pin The input string representing the pin.
    /// @param mnemonic The input string representing the mnemonic
    ///
    /// @return a boolean indicating if the mnemonic is correct or not
    pub fn verifyMnemonic(pin: String, mnemonic: String) -> Result<bool, String> {
        let result = runtime().block_on(async move {
            let mut sdk = get_or_init_sdk().write().await;
            let pin = EncryptionPin::try_from_string(pin)?;
            sdk.verify_mnemonic(&pin, &mnemonic).await
        });
        result.map_err(|e| format!("{e:#?}"))
    }
    /// Creates a new wallet and sets the pin and password
    ///
    /// @param pin The input string representing the pin.
    ///
    /// @return The mnemonic of the created wallet as a string
    pub fn createNewWallet(pin: String) -> Result<String, String> {
        let result = runtime().block_on(async move {
            let mut sdk = get_or_init_sdk().write().await;
            let pin = EncryptionPin::try_from_string(pin)?;
            sdk.create_wallet_from_new_mnemonic(&pin).await
        });
        result.map_err(|e| format!("{e:#?}"))
    }
    /// Creates/migrates a wallet from an existing mnemonic
    ///
    /// @param pin The input string representing the pin.
    /// @param mnemonic The input string representing the mnemonic
    pub fn createWalletFromMnemonic(pin: String, mnemonic: String) -> Result<(), String> {
        let result = runtime().block_on(async move {
            let mut sdk = get_or_init_sdk().write().await;
            let pin = EncryptionPin::try_from_string(pin)?;
            sdk.create_wallet_from_existing_mnemonic(&pin, &mnemonic).await
        });
        result.map_err(|e| format!("{e:#?}"))
    }

    /// Creates a wallet from a previously created backup
    ///
    /// @param pin The input string representing the pin.
    /// @param backup The bytes of the backup file.
    /// @param backup_password The input string representing the password of the backup
    pub fn createWalletFromBackup(pin: String, backup: Vec<u8>, backup_password: String) -> Result<(), String> {
        let result = runtime().block_on(async move {
            let mut sdk = get_or_init_sdk().write().await;
            let pin = EncryptionPin::try_from_string(pin)?;
            let backup_password = PlainPassword::try_from_string(backup_password)?;
            sdk.create_wallet_from_backup(&pin, &backup, &backup_password).await
        });

        result.map_err(|e| format!("{e:#?}"))
    }
    /// Creates a wallet backup file for the existing wallet and encrypts it with the password
    ///
    /// @param pin The input string representing the pin.
    /// @param backup_password The input string representing the password to be used to encrypt the backup
    ///
    /// @return The path to the generated backup file as string
    #[public_name = "createWalletBackup"]
    pub fn createWalletBackup(pin: String, backup_password: String) -> Result<Vec<u8>, String> {
        let result = runtime().block_on(async move {
            let mut sdk = get_or_init_sdk().write().await;
            let pin = EncryptionPin::try_from_string(pin)?;
            let backup_password = PlainPassword::try_from_string(backup_password)?;
            sdk.create_wallet_backup(&pin, &backup_password).await
        });

        result.map_err(|e| format!("{e:#?}"))
    }

    /// Deletes the local wallet and associated files
    ///
    /// @param pin The input string representing the pin.
    pub fn deleteWallet(pin: String) -> Result<(), String> {
        let result = runtime().block_on(async move {
            let mut sdk = get_or_init_sdk().write().await;
            let pin = EncryptionPin::try_from_string(pin)?;
            sdk.delete_wallet(&pin).await
        });
        result.map_err(|e| format!("{e:#?}"))
    }

    /// Generate a new receiver address based on selected currency in the config.
    ///
    /// @param pin The input string representing the pin.
    /// @return The receiver wallet address as String.
    #[public_name = "generateNewAddress"]
    pub fn generateNewAddress(pin: String) -> Result<String, String> {
        let result = runtime().block_on(async move {
            let mut sdk = get_or_init_sdk().write().await;
            let pin = EncryptionPin::try_from_string(pin)?;
            sdk.generate_new_address(&pin).await
        });
        result.map_err(|e| format!("{e:#?}"))
    }

    /// Fetches the current balance of the base crypto currency on the wallet
    ///
    /// @param pin The input string representing the pin.
    /// @return The current balance as a double precision floating point number
    pub fn getWalletBalance(pin: String) -> Result<f64, String> {
        let result = runtime().block_on(async move {
            let mut sdk = get_or_init_sdk().write().await;
            let pin = EncryptionPin::try_from_string(pin)?;
            sdk.get_balance(&pin).await.and_then(f64::try_from)
        });
        result.map_err(|e| format!("{e:#?}"))
    }

    /// Initialize the KYC process for Postident by generating a case id.
    ///
    /// <p>
    /// Note: This method is only available if the SDK is compiled with support for postident.
    /// </p>
    ///
    /// @return The ID of the new Postident KYC case.
    #[public_name = "startKycVerificationForPostident"]
    pub fn initKycVerificationForPostident() -> Result<String, String> {
        sdk::require_feature!("postident", {
            let result = runtime().block_on(async move {
                let mut sdk = get_or_init_sdk().write().await;
                sdk.start_kyc_verification_for_postident().await
            });

            match result {
                Ok(value) => serde_json::to_string(&value).map_err(|e| format!("{e:#?}")),
                Err(e) => Err(format!("{e:#?}")),
            }
        })
    }

    /// Fetches the KYC details for the postident provider
    ///
    /// <p>
    /// Note: This method is only available if the SDK is compiled with support for postident.
    /// </p>
    ///
    /// @return The case details as a serialized JSON string
    pub fn getKycDetailsForPostident() -> Result<String, String> {
        sdk::require_feature!("postident", {
            let result = runtime().block_on(async move {
                let sdk = get_or_init_sdk().write().await;
                sdk.get_kyc_details_for_postident().await
            });
            match result {
                Ok(value) => serde_json::to_string(&value).map_err(|e| format!("{e:#?}")),
                Err(e) => Err(format!("{e:#?}")),
            }
        })
    }

    /// Triggers the backend to update the KYC status in the postident KYC provider
    ///
    /// <p>
    /// Note: This method is only available if the SDK is compiled with support for postident.
    /// </p>
    ///
    /// @param case_id The input string representing the case_id to be updated
    // cSpell: disable
    #[public_name = "updateKycStatusForPostident"]
    pub fn updateKycDetailsForPostident(case_id: String) -> Result<(), String> {
        // cSpell: enable

        sdk::require_feature!("postident", {
            let result = runtime().block_on(async move {
                let sdk = get_or_init_sdk().write().await;
                sdk.update_kyc_status_for_postident(&case_id).await
            });

            result.map_err(|e| format!("{e:#?}"))
        })
    }

    /// Creates a purchase request for buying an artefact
    ///
    /// @param receiver The receiver of the purchase request.
    /// @param amount The amount of the purchase.
    /// @param product_hash The hash of the underlying product/artefact
    /// @param app_data The app data for the purchase. This is application specific string or stringified object data.
    /// @param purchase_type The type of the purchase. Either a COMPLIMENT or a PURCHASE
    ///
    /// @return The purchase id of the created purchase request as string
    #[public_name = "purchaseRequestCreate"]
    pub fn createPurchaseRequest(
        receiver: String,
        amount: f64,
        product_hash: String,
        app_data: String,
        purchase_type: String,
    ) -> Result<String, String> {
        let result = runtime().block_on(async move {
            let sdk = get_or_init_sdk().write().await;
            let amount = CryptoAmount::try_from(amount)?;
            sdk.create_purchase_request(&receiver, amount, &product_hash, &app_data, &purchase_type)
                .await
        });
        result.map_err(|e| format!("{e:#?}"))
    }

    /// Fetches the purchase details from the given purchase ID.
    ///
    /// @param purchase_id The purchase id to query to details.
    ///
    /// @return The purchase details as a serialized JSON string
    #[public_name = "purchaseDetails"]
    pub fn getPurchaseDetails(purchase_id: String) -> Result<String, String> {
        let result = runtime().block_on(async move {
            let sdk = get_or_init_sdk().write().await;
            sdk.get_purchase_details(&purchase_id).await.and_then(TryInto::try_into)
        });

        match result {
            Ok(value) => {
                let entity: PurchaseDetailsEntity = value;
                serde_json::to_string(&entity).map_err(|e| format!("{e:#?}"))
            }
            Err(e) => Err(format!("{e:#?}")),
        }
    }

    /// Confirm the purchase for the given purchase ID.
    ///
    /// @param pin The pin for confirmation of purchase
    /// @param purchase_id The purchase id to confirm.
    #[public_name = "purchaseRequestConfirm"]
    pub fn confirmPurchaseRequest(pin: String, purchase_id: String) -> Result<(), String> {
        let result = runtime().block_on(async move {
            let mut sdk = get_or_init_sdk().write().await;
            let pin = EncryptionPin::try_from_string(pin)?;
            sdk.confirm_purchase_request(&pin, &purchase_id).await
        });
        result.map_err(|e| format!("{e:#?}"))
    }

    /// Starts the KYC verification process for viviswap
    ///
    /// @param mail The email address of the user as a string.
    /// @param terms_accepted The terms of conditions accepted flag for the user as a boolean
    ///
    /// @return The new viviswap user as a serialized JSON string
    #[public_name = "startViviswapKyc"]
    pub fn startKycVerificationForViviswap(mail: String, terms_accepted: bool) -> Result<String, String> {
        sdk::require_feature!("viviswap-kyc", {
            let result = runtime().block_on(async move {
                let mut sdk = get_or_init_sdk().write().await;
                sdk.start_kyc_verification_for_viviswap(&mail, terms_accepted).await
            });
            match result {
                Ok(value) => serde_json::to_string(&value).map_err(|e| format!("{e:#?}")),
                Err(e) => Err(format!("{e:#?}")),
            }
        })
    }

    /// Fetches the KYC details for a user by the viviswap onboarding process
    ///
    /// @return The KYC details as a serialized JSON string
    #[public_name = "getViviswapKyc"]
    pub fn getKycDetailsForViviswap() -> Result<String, String> {
        sdk::require_feature!("viviswap-kyc", {
            let result = runtime().block_on(async move {
                let mut sdk = get_or_init_sdk().write().await;
                sdk.get_kyc_details_for_viviswap().await
            });
            match result {
                Ok(value) => serde_json::to_string(&value).map_err(|e| format!("{e:#?}")),
                Err(e) => Err(format!("{e:#?}")),
            }
        })
    }

    /// Updates the partial KYC details for the viviswap onboarding process
    ///
    /// @param is_individual Flag indicating if the user is an individual.
    /// @param is_pep Flag indicating if the user is a politically exposed person.
    /// @param is_us_citizen Flag indicating if the user is a US citizen.
    /// @param is_regulatory_disclosure Flag indicating if the user has made a regulatory disclosure.
    /// @param country_of_residence The country of residence of the user.
    /// @param nationality The nationality of the user.
    /// @param full_name The full name of the user.
    /// @param date_of_birth The date of birth of the user.
    ///
    /// @return The KYC updated details reflected as a serialized JSON string
    #[public_name = "updateViviswapKycPartial"]
    pub fn updateKycPartiallyStatusForViviswap(
        is_individual: bool,
        is_pep: bool,
        is_us_citizen: bool,
        is_regulatory_disclosure: bool,
        country_of_residence: String,
        nationality: String,
        full_name: String,
        date_of_birth: String,
    ) -> Result<String, String> {
        sdk::require_feature!("viviswap-kyc", {
            let result = runtime().block_on(async move {
                let mut sdk = get_or_init_sdk().write().await;
                sdk.update_kyc_partially_status_for_viviswap(
                    Some(is_individual),
                    Some(is_pep),
                    Some(is_us_citizen),
                    Some(is_regulatory_disclosure),
                    Some(country_of_residence),
                    Some(nationality),
                    Some(full_name),
                    Some(date_of_birth),
                )
                .await
            });

            match result {
                Ok(value) => serde_json::to_string(&value).map_err(|e| format!("{e:#?}")),
                Err(e) => Err(format!("{e:#?}")),
            }
        })
    }

    /// Submits the partial KYC details for the viviswap onboarding process
    #[public_name = "submitViviswapKycPartial"]
    pub fn submitKycPartiallyStatusForViviswap() -> Result<(), String> {
        sdk::require_feature!("viviswap-kyc", {
            let result = runtime().block_on(async move {
                let mut sdk = get_or_init_sdk().write().await;
                sdk.submit_kyc_partially_status_for_viviswap().await
            });
            result.map_err(|e| format!("{e:#?}"))
        })
    }

    /// Set Viviswap KYC identity details
    ///
    /// @param official_document_type The type of the official document.
    /// @param expiration_date The expiration date of the official document.
    /// @param document_number The number of the official document.
    /// @param official_document_front_image_data  The byte data of the image of the front of the official document.
    /// @param official_document_front_image_filename The filename (including extension) of the image of the front of the official document.
    /// @param official_document_back_image_data The byte data of the image of the back of the official document, or NULL to not provide a back image.
    /// @param official_document_back_image_filename The filename (including extension) of the image of the back of the official document, or NULL to no provide a back image.
    /// @param personal_video_data The byte data of the 30 second personal video recording.
    /// @param personal_video_filename The filename (including extenstion) of the 30 second personal video recording.
    pub fn setViviswapKycIdentityDetails(
        official_document_type: String,
        expiration_date: String,
        document_number: String,
        official_document_front_image_data: Vec<u8>,
        official_document_front_image_filename: String,
        official_document_back_image_data: Option<Vec<u8>>,
        official_document_back_image_filename: Option<String>,
        personal_video_data: Vec<u8>,
        personal_video_filename: String,
    ) -> Result<(), String> {
        sdk::require_feature!("viviswap-kyc", {
            let result = runtime().block_on(async move {
                let sdk = get_or_init_sdk().write().await;

                let front_image = File::from_bytes(
                    &official_document_front_image_data,
                    &official_document_front_image_filename,
                );

                let back_image = if let (Some(filename), Some(data)) =
                    (official_document_back_image_filename, official_document_back_image_data)
                {
                    Some(File::from_bytes(&data, &filename))
                } else {
                    None
                };

                let official_document_video = File::from_bytes(&personal_video_data, &personal_video_filename);

                sdk.set_viviswap_kyc_identity_details(
                    sdk::types::IdentityOfficialDocumentData {
                        r#type: official_document_type.parse().map_err(sdk::Error::Parse)?,
                        expiration_date,
                        document_number,
                        front_image,
                        back_image,
                    },
                    sdk::types::IdentityPersonalDocumentData {
                        video: official_document_video,
                    },
                )
                .await
            });
            result.map_err(|e| format!("{e:#?}"))
        })
    }

    /// Set Viviswap KYC residence details
    ///
    /// @param country_code`, `region`, `zip_code`, `city`, `address_line_1`, `address_line_2 basic address data.
    /// @param is_public_entry Inidcates that a valid public entry of this clients address can be found.
    /// @param public_entry_reference if `is_public_entry` is `true`, then this must contain the resource link.
    /// @param has_no_official_document indicates if the client does not have any document verifying their address.
    /// @param official_document_image_data if `has_no_official_document` is `false`, then this must contain the bytes of the
    ///        document file that verifies that this person is currently living at the address submitted. Otherwise leave as NULL.
    /// @param official_document_image_filename the filename (including extension) of the document,
    ///        or NULL if no document needs to be provided.
    pub fn setViviswapKycResidenceDetails(
        country_code: String,
        region: String,
        zip_code: String,
        city: String,
        address_line_1: String,
        address_line_2: String,
        is_public_entry: bool,
        public_entry_reference: String,
        has_no_official_document: bool,
        official_document_image_data: Option<Vec<u8>>,
        official_document_image_filename: Option<String>,
    ) -> Result<(), String> {
        sdk::require_feature!("viviswap-kyc", {
            let result = runtime().block_on(async move {
                let sdk = get_or_init_sdk().write().await;

                let public_entry_reference = if public_entry_reference.is_empty() {
                    None
                } else {
                    Some(public_entry_reference)
                };

                let official_document = if let (Some(filename), Some(data)) =
                    (official_document_image_filename, official_document_image_data)
                {
                    Some(File::from_bytes(&data, &filename))
                } else {
                    None
                };

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
            });
            result.map_err(|e| format!("{e:#?}"))
        })
    }

    /// Get the open AMLA KYC questions
    ///
    /// @return A list of the currently open AMLA questions.
    pub fn getViviswapKycAmlaOpenQuestions() -> Result<String, String> {
        sdk::require_feature!("viviswap-kyc", {
            let result = runtime().block_on(async move {
                let sdk = get_or_init_sdk().write().await;
                sdk.get_viviswap_kyc_amla_open_questions().await
            });

            match result {
                Ok(value) => serde_json::to_string(&value).map_err(|e| format!("{e:#?}")),
                Err(e) => Err(format!("{e:#?}")),
            }
        })
    }

    /// Set the answer to an open AMLA KYC question
    ///
    /// @param question_id The ID of the question to set the answer to.
    /// @param answers a list of the selected available answers for the question.
    /// @param freetext_answer an optional free-text answer. Pass NULL to not pass any value.
    pub fn setViviswapKycAmlaAnswer(
        question_id: String,
        answers: Vec<String>,
        freetext_answer: Option<String>,
    ) -> Result<(), String> {
        sdk::require_feature!("viviswap-kyc", {
            let result = runtime().block_on(async move {
                let sdk = get_or_init_sdk().write().await;

                sdk.set_viviswap_kyc_amla_answer(question_id, answers, freetext_answer)
                    .await
            });

            result.map_err(|e| format!("{e:#?}"))
        })
    }

    /// Get the currently open/missing documents for KYC
    ///
    /// @return A list of the currently open documents.
    pub fn getViviswapKycOpenDocuments() -> Result<String, String> {
        sdk::require_feature!("viviswap-kyc", {
            let result = runtime().block_on(async move {
                let sdk = get_or_init_sdk().write().await;
                sdk.get_viviswap_kyc_open_documents().await
            });

            match result {
                Ok(value) => serde_json::to_string(&value).map_err(|e| format!("{e:#?}")),
                Err(e) => Err(format!("{e:#?}")),
            }
        })
    }

    /// Set / upload an open KYC document
    ///
    /// @param document_id The ID of the document to upload.
    /// @param expiration_date the expiration date of this document.
    /// @param document_number the official document number.
    /// @param front_image_data the bytes of the image of the front side of the document.
    /// @param front_image_filename the filename (including extension) of the front side of the document.
    /// @param back_image_data the bytes of the mage of the back side of the documentk, or NULL for not specifying a back side image.
    /// @param back_image_filename the filename (including extension) of the back side of the documentk, or NULL for not specifying a back side image.
    pub fn setViviswapKycDocument(
        document_id: String,
        expiration_date: String,
        document_number: String,
        front_image_data: Vec<u8>,
        front_image_filename: String,
        back_image_data: Option<Vec<u8>>,
        back_image_filename: Option<String>,
    ) -> Result<(), String> {
        sdk::require_feature!("viviswap-kyc", {
            let result = runtime().block_on(async move {
                let sdk = get_or_init_sdk().write().await;

                let front_image = if front_image_data.is_empty() || front_image_filename.is_empty() {
                    None
                } else {
                    Some(File::from_bytes(&front_image_data, &front_image_filename))
                };

                let back_image = if let (Some(filename), Some(data)) = (back_image_filename, back_image_data) {
                    Some(File::from_bytes(&data, &filename))
                } else {
                    None
                };

                sdk.set_viviswap_kyc_document(document_id, expiration_date, document_number, front_image, back_image)
                    .await
            });

            result.map_err(|e| format!("{e:#?}"))
        })
    }

    /// Verifies the pin for the wallet
    ///
    /// @param pin The pin to be verified
    #[public_name = "pinVerify"]
    pub fn verifyPin(pin: String) -> Result<(), String> {
        let result = runtime().block_on(async move {
            let sdk = get_or_init_sdk().write().await;
            let pin = EncryptionPin::try_from_string(pin)?;
            sdk.verify_pin(&pin).await
        });
        result.map_err(|e| format!("{e:#?}"))
    }

    /// Resets the pin for the wallet using the wallet password
    ///
    /// @param new_pin The new pin to be set for the wallet
    #[public_name = "pinReset"]
    pub fn resetPin(pin: String, new_pin: String) -> Result<(), String> {
        let result = runtime().block_on(async move {
            let mut sdk = get_or_init_sdk().write().await;
            let pin = EncryptionPin::try_from_string(pin)?;
            let new_pin = EncryptionPin::try_from_string(new_pin)?;
            sdk.change_pin(&pin, &new_pin).await
        });
        result.map_err(|e| format!("{e:#?}"))
    }

    /// Set the password to use for wallet operations. If the password was already set, this changes it.
    ///
    /// @param pin The pin for verification
    /// @param new_password The new password to be set
    #[public_name = "setWalletPassword"]
    pub fn setWalletPassword(pin: String, new_password: String) -> Result<(), String> {
        let result = runtime().block_on(async move {
            let mut sdk = get_or_init_sdk().write().await;
            let pin = EncryptionPin::try_from_string(pin)?;
            let new_password = PlainPassword::try_from_string(new_password)?;
            sdk.set_wallet_password(&pin, &new_password).await
        });
        result.map_err(|e| format!("{e:#?}"))
    }

    /// Check if the password to use for wallet operations is set.
    /// Use {@link #setWalletPassword} to set a new or change an existing password.
    ///
    /// @return whether the password is already set or not.
    #[public_name = "isWalletPasswordSet"]
    pub fn is_wallet_password_set() -> Result<bool, String> {
        let result = runtime().block_on(async move {
            let sdk = get_or_init_sdk().read().await;
            sdk.is_wallet_password_set().await
        });
        result.map_err(|e| format!("{e:#?}"))
    }

    /// Sends the given amount to the given address
    ///
    /// @param pin The pin for verification
    /// @param address The address of the receiver
    /// @param amount The amount to send in the selected currency
    /// @param data The data associated with the transaction. Pass NULL to not specify any data.
    /// @return The transaction id.
    #[public_name = "sendAmount"]
    pub fn sendAmount(pin: String, address: String, amount: f64, data: Option<Vec<u8>>) -> Result<String, String> {
        let result = runtime().block_on(async move {
            let mut sdk = get_or_init_sdk().write().await;
            let amount = CryptoAmount::try_from(amount)?;
            let pin = EncryptionPin::try_from_string(pin)?;
            sdk.send_amount(&pin, &address, amount, data).await
        });
        result.map_err(|e| format!("{e:#?}"))
    }

    /// Updates the IBAN of the user
    ///
    /// @param pin The pin for verification
    /// @param address The IBAN number to be updated
    ///
    /// @return The details of the added IBAN as a serialized JSON string.
    #[public_name = "updateIbanViviswap"]
    pub fn updateIban(pin: String, address: String) -> Result<String, String> {
        sdk::require_feature!("viviswap-swap", {
            let result = runtime().block_on(async move {
                let mut sdk = get_or_init_sdk().write().await;
                let pin = EncryptionPin::try_from_string(pin)?;
                sdk.update_iban_for_viviswap(&pin, address).await
            });
            match result {
                Ok(value) => serde_json::to_string(&value).map_err(|e| format!("{e:#?}")),
                Err(e) => Err(format!("{e:#?}")),
            }
        })
    }

    /// Gets the IBAN of the user
    ///
    /// @return The details of the IBAN as a serialized JSON string.
    #[public_name = "getIbanViviswap"]
    pub fn getIban() -> Result<String, String> {
        sdk::require_feature!("viviswap-swap", {
            let result = runtime().block_on(async move {
                let mut sdk = get_or_init_sdk().write().await;
                sdk.get_iban_for_viviswap().await
            });
            match result {
                Ok(value) => serde_json::to_string(&value).map_err(|e| format!("{e:#?}")),
                Err(e) => Err(format!("{e:#?}")),
            }
        })
    }

    /// Creates a payment contract for depositing money in wallet using viviswap [EURO --> Crypto]
    ///
    /// @param pin The input string representing the pin.
    ///
    /// @return The details of the added payment contract as a serialized JSON string.
    #[public_name = "depositWithViviswap"]
    pub fn depositViviswap(pin: String) -> Result<String, String> {
        sdk::require_feature!("viviswap-swap", {
            let result = runtime().block_on(async move {
                let mut sdk = get_or_init_sdk().write().await;
                let pin = EncryptionPin::try_from_string(pin)?;
                sdk.create_deposit_with_viviswap(&pin).await
            });
            match result {
                Ok(value) => serde_json::to_string(&value).map_err(|e| format!("{e:#?}")),
                Err(e) => Err(format!("{e:#?}")),
            }
        })
    }

    /// Creates a payment detail for the wallet crypto address in viviswap
    ///
    /// @param pin The input string representing the pin.
    ///
    /// @return The details of the added payment detail as a serialized JSON string.
    #[public_name = "createViviswapDetail"]
    pub fn createDetailViviswap(pin: String) -> Result<String, String> {
        sdk::require_feature!("viviswap-swap", {
            let result = runtime().block_on(async move {
                let mut sdk = get_or_init_sdk().write().await;
                let pin = EncryptionPin::try_from_string(pin)?;
                sdk.create_detail_for_viviswap(&pin).await
            });
            match result {
                Ok(value) => serde_json::to_string(&value).map_err(|e| format!("{e:#?}")),
                Err(e) => Err(format!("{e:#?}")),
            }
        })
    }

    /// Creates a payment contract for withdrawing money from wallet using viviswap [Crypto --> EUR] and if the pin is provided automatically triggers a withdrawal
    ///
    /// @param amount The amount to withdraw from the wallet
    /// @param pin The pin for verification. Pass NULL to not specify a pin.
    /// @param data The data associated with the transaction. Pass NULL to not specify any data.
    ///
    /// @return The details of the created payment contract as a serialized JSON string.
    #[public_name = "withdrawWithViviswap"]
    pub fn withdrawViviswap(amount: f64, pin: Option<String>, data: Option<Vec<u8>>) -> Result<String, String> {
        sdk::require_feature!("viviswap-swap", {
            let result = runtime().block_on(async move {
                let mut sdk = get_or_init_sdk().write().await;

                let amount = CryptoAmount::try_from(amount)?;

                let pin = match pin {
                    Some(pin) => Some(EncryptionPin::try_from_string(pin)?),
                    None => None,
                };

                sdk.create_withdrawal_with_viviswap(amount, pin.as_ref(), data).await
            });
            match result {
                Ok(value) => serde_json::to_string(&value).map_err(|e| format!("{e:#?}")),
                Err(e) => Err(format!("{e:#?}")),
            }
        })
    }

    /// Gets the detail of a particular swap(deposit or withdrawal) created at viviswap based on the given order id.
    ///
    /// @param order_id The amount to withdraw from the wallet
    ///
    /// @return The details of the created order as a serialized JSON string.
    pub fn getSwapDetails(order_id: String) -> Result<String, String> {
        sdk::require_feature!("viviswap-swap", {
            let result = runtime().block_on(async move {
                let sdk = get_or_init_sdk().write().await;
                sdk.get_swap_details(order_id).await
            });
            match result {
                Ok(value) => serde_json::to_string(&value).map_err(|e| format!("{e:#?}")),
                Err(e) => Err(format!("{e:#?}")),
            }
        })
    }

    /// Gets the detailed lists of swaps (deposit and withdrawal) created at viviswap
    ///
    /// @param start The start page
    /// @param limit The limit per page
    ///
    /// @return The details of the created orders as a serialized JSON string.
    pub fn getSwapList(start: i64, limit: i64) -> Result<String, String> {
        sdk::require_feature!("viviswap-swap", {
            let result = runtime().block_on(async move {
                let sdk = get_or_init_sdk().write().await;
                sdk.get_swap_list(start as u32, limit as u32).await
            });
            match result {
                Ok(value) => serde_json::to_string(&value).map_err(|e| format!("{e:#?}")),
                Err(e) => Err(format!("{e:#?}")),
            }
        })
    }

    /// Gets the detailed lists of purchases (COMPLIMENTS and PURCHASES)
    ///
    /// @param start The start page
    /// @param limit The limit per page
    ///
    /// @return The details of the created purchases as a serialized JSON string.
    #[public_name = "txList"]
    pub fn getTxList(start: i64, limit: i64) -> Result<String, String> {
        let result = runtime().block_on(async move {
            let sdk = get_or_init_sdk().write().await;
            sdk.get_tx_list(start as u32, limit as u32).await
        });
        match result {
            Ok(value) => serde_json::to_string(&value).map_err(|e| format!("{e:#?}")),
            Err(e) => Err(format!("{e:#?}")),
        }
    }

    /// Gets the current exchange rate for the cryptocurrency to EURO
    ///
    /// @return The exchange rate as a floating point number
    #[public_name = "getExchangeRate"]
    pub fn exchangeRateViviswap() -> Result<f64, String> {
        let result = runtime().block_on(async move {
            let sdk = get_or_init_sdk().write().await;
            sdk.get_exchange_rate()
                .await
                .and_then(|amount| Ok(f64::try_from(amount)?))
        });
        result.map_err(|e| format!("{e:#?}"))
    }

    /// Deletes the user in ETOPay. Hazmat!
    ///
    /// @param pin The wallet pin for confirmation. Optional in case there is an active wallet.
    pub fn deleteUser(pin: Option<String>) -> Result<(), String> {
        let result = runtime().block_on(async move {
            let mut sdk = get_or_init_sdk().write().await;
            let encryption_pin = match pin {
                Some(p) => Some(EncryptionPin::try_from_string(p)?),
                None => None,
            };
            sdk.delete_user(encryption_pin.as_ref()).await
        });
        result.map_err(|e| format!("{e:#?}"))
    }

    /// Gets the detailed lists of wallet transactions
    ///
    /// @param pin The input string representing the pin.
    /// @param start The start page
    /// @param limit The limit per page
    ///
    /// @return The details of the wallet transactions as a serialized JSON string.
    #[public_name = "getWalletTransactionList"]
    pub fn getWalletTxList(pin: String, start: i64, limit: i64) -> Result<String, String> {
        let result = runtime().block_on(async move {
            let mut sdk = get_or_init_sdk().write().await;
            let pin = EncryptionPin::try_from_string(pin)?;
            sdk.get_wallet_tx_list(&pin, start as usize, limit as usize).await
        });
        match result {
            Ok(value) => serde_json::to_string(&value).map_err(|e| format!("{e:#?}")),
            Err(e) => Err(format!("{e:#?}")),
        }
    }

    /// Gets the details of a specific wallet transaction
    ///
    /// @param pin The input string representing the pin.
    /// @param tx_id The ID of the transaction to get details for.
    ///
    /// @return The details of the wallet transaction as a serialized JSON string.
    #[public_name = "getWalletTransaction"]
    pub fn getWalletTx(pin: String, tx_id: String) -> Result<String, String> {
        let result = runtime().block_on(async move {
            let mut sdk = get_or_init_sdk().write().await;
            let pin = EncryptionPin::try_from_string(pin)?;
            sdk.get_wallet_tx(&pin, &tx_id).await
        });

        match result {
            Ok(value) => serde_json::to_string(&value).map_err(|e| format!("{e:#?}")),
            Err(e) => Err(format!("{e:#?}")),
        }
    }

    /// Get/download the recovery share.
    ///
    ///
    /// @return The recovery share as a string, or `null` if none exists.
    pub fn getRecoveryShare() -> Result<Option<String>, String> {
        use sdk::secrecy::ExposeSecret;
        let result = runtime().block_on(async move {
            let sdk = get_or_init_sdk().write().await;
            sdk.get_recovery_share().await
        });

        result
            .map(|s| s.map(|s| s.to_string().expose_secret().to_string()))
            .map_err(|e| format!("{e:#?}"))
    }

    /// Set/upload the recovery share.
    ///
    /// @param share The recovery share to upload.
    pub fn setRecoveryShare(share: String) -> Result<(), String> {
        let result = runtime().block_on(async move {
            let mut sdk = get_or_init_sdk().write().await;
            let share: Share = share.parse().map_err(|e| sdk::Error::Wallet(WalletError::Share(e)))?;
            sdk.set_recovery_share(share).await
        });

        result.map_err(|e| format!("{e:#?}"))
    }

    /// Get the user's preferred network.
    ///
    /// @return The preferred network, or `null` if it has not been set.
    pub fn getPreferredNetwork() -> Result<Option<String>, String> {
        let result = runtime().block_on(async move {
            let sdk = get_or_init_sdk().write().await;
            sdk.get_preferred_network().await
        });

        result.map(|s| s.map(|c| c.to_string())).map_err(|e| format!("{e:#?}"))
    }

    /// Set the user's preferred network.
    ///
    /// @param network_id The preferred network, or `null` if it should be unset.
    pub fn setPreferredNetwork(network_id: Option<String>) -> Result<(), String> {
        let result = runtime().block_on(async move {
            let mut sdk = get_or_init_sdk().write().await;
            sdk.set_preferred_network(network_id).await
        });

        result.map_err(|e| format!("{e:#?}"))
    }

    /// Get sdk build information.
    ///
    /// @return The sdk build information as a string.
    pub fn getBuildInfo() -> String {
        Sdk::get_build_info()
    }
}
