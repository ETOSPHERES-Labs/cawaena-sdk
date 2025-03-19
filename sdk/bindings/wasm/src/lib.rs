//! Contains the bindings for WASM, needs to be compiled to the `wasm32-unknown-unknown` target.

mod types;
mod utils;

use crate::types::*;
use crate::utils::set_panic_hook;

#[cfg(feature = "viviswap-kyc")]
use sdk::types::File;

use sdk::{
    core::{Config, Sdk},
    types::{
        currencies::CryptoAmount,
        newtypes::{AccessToken, EncryptionPin, PlainPassword},
    },
};
use std::sync::Arc;
use tokio::sync::RwLock;
use wasm_bindgen::prelude::*;

/// Main object that contains all the functionality for interfacing with the ETOPaySdk.
#[wasm_bindgen]
pub struct ETOPaySdk {
    inner: Arc<RwLock<Sdk>>,
}

#[wasm_bindgen]
impl ETOPaySdk {
    #[wasm_bindgen(constructor)]
    #[allow(clippy::new_without_default)] // a default implementation would be useless in this case
    /// Create a new instance of the `ETOPaySdk`
    /// @returns {ETOPaySdk} a new `ETOPaySdk` instance.
    pub fn new() -> Self {
        #[cfg(feature = "console_error_panic_hook")]
        set_panic_hook();

        let sdk = Sdk::default();

        Self {
            inner: Arc::new(RwLock::new(sdk)),
        }
    }

    /// Set the configuration as a JSON-encoded string.
    ///
    /// @param {String} config The input string representing the configuration.
    ///
    /// @example
    /// ```json
    /// {
    ///     "auth_provider": "<authentication provider name>",
    ///     "backend_url": "<valid URL to the backend API>",
    ///     "storage_path": "/path/to/valid/folder",
    ///     "log_level": "info",
    /// }
    /// ```
    ///
    /// @returns {Promise<void>}
    #[wasm_bindgen(skip_jsdoc, js_name = "setConfig")]
    pub async fn set_config(&self, config: String) -> Result<(), String> {
        let mut sdk = self.inner.write().await;
        Config::from_json(&config)
            .and_then(|r| sdk.set_config(r))
            .map_err(|err| format!("{:#?}", err))
    }

    /// Selects the network for the ETOPay SDK.
    ///
    /// @param {String} network_key.
    /// @returns {Promise<void>}
    #[wasm_bindgen(skip_jsdoc, js_name = "setNetwork")]
    pub async fn set_network(&self, network_key: String) -> Result<(), String> {
        let mut sdk = self.inner.write().await;
        sdk.set_network(network_key).await.map_err(|e| format!("{e:#?}"))
    }

    /// Fetch available networks.
    ///
    /// @returns {Option<Vec<Network>>} Sdk networks
    #[wasm_bindgen(skip_jsdoc, js_name = "getNetworks")]
    pub async fn get_networks(&self) -> Result<Vec<Network>, String> {
        let mut sdk = self.inner.write().await;
        let networks = sdk
            .get_networks()
            .await
            .map_err(|e| format!("{e:#?}"))?
            .iter()
            .map(|n| Network::from(n.clone()))
            .collect::<Vec<Network>>();

        Ok(networks)
    }

    /// Initializes the etopay logger
    /// @param {Level} level - The log level.
    /// @returns {void}
    #[wasm_bindgen(skip_jsdoc, js_name = "initLogger")]
    pub fn init_logger(&self, level: Level) {
        // ignore the error since the function can only be called once per page load (or an error will be thrown)
        console_log::init_with_level(level.into()).ok();
    }

    /// Creates a new user for the SDK.
    ///
    /// @param {string} username - The input string representing the username.
    /// @returns {Promise<void>}
    #[wasm_bindgen(skip_jsdoc, js_name = "createNewUser")]
    pub async fn create_new_user(&self, username: String) -> Result<(), String> {
        let mut sdk = self.inner.write().await;
        sdk.create_new_user(&username).await.map_err(|e| format!("{e:#?}"))
    }

    /// Initializes an existing user in the SDK
    ///
    /// @param {string} username - The input string representing the username.
    /// @returns {Promise<void>}
    #[wasm_bindgen(skip_jsdoc, js_name = "initializeUser")]
    pub async fn initialize_user(&self, username: String) -> Result<(), String> {
        let mut sdk = self.inner.write().await;
        sdk.init_user(&username).await.map_err(|e| format!("{e:#?}"))
    }

    /// Refreshes the access token for the user in the SDK.
    ///
    /// @param {string} access_token - The input string representing the access token.
    /// @returns {Promise<void>}
    #[wasm_bindgen(skip_jsdoc, js_name = "refreshAccessToken")]
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
        .map_err(|e| format!("{e:#?}"))
    }

    /// Fetches the kyc verification status for the user
    ///
    /// @param {string} username - The input string representing the username.
    ///
    /// @returns {Promise<bool>} The kyc verification status as a boolean value
    #[wasm_bindgen(skip_jsdoc, js_name = "isKycVerified")]
    pub async fn is_kyc_verified(&self, username: String) -> Result<bool, String> {
        let mut sdk = self.inner.write().await;
        sdk.is_kyc_status_verified(&username)
            .await
            .map_err(|e| format!("{e:#?}"))
    }

    /// Creates a new random wallet and returns the mnemonic.
    ///
    /// @param {string} pin - The input string representing the pin.
    /// @returns {Promise<String>}
    #[wasm_bindgen(skip_jsdoc, js_name = "createNewWallet")]
    pub async fn create_new_wallet(&self, pin: String) -> Result<String, String> {
        let mut sdk = self.inner.write().await;
        async move {
            let pin = EncryptionPin::try_from_string(pin)?;
            sdk.create_wallet_from_new_mnemonic(&pin).await
        }
        .await
        .map_err(|e| format!("{e:#?}"))
    }

    /// Initializes a wallet from an existing mnemonic.
    ///
    /// @param {string} pin - The input string representing the pin.
    /// @param {string} mnemonic - The input string representing the mnemonic
    /// @returns {Promise<void>}
    #[wasm_bindgen(skip_jsdoc, js_name = "createWalletFromMnemonic")]
    pub async fn create_wallet_from_mnemonic(&self, pin: String, mnemonic: String) -> Result<(), String> {
        let mut sdk = self.inner.write().await;
        async move {
            let pin = EncryptionPin::try_from_string(pin)?;
            sdk.create_wallet_from_existing_mnemonic(&pin, &mnemonic).await
        }
        .await
        .map_err(|e| format!("{e:#?}"))
    }

    /// Creates a wallet from a backup.
    ///
    /// @param {string} pin - The input string representing the pin.
    /// @param {Uint8Array} backup - The bytes of the backup file.
    /// @param {string} backup_password - Password used to create the backup.
    ///
    /// @returns {Promise<void>}
    #[wasm_bindgen(skip_jsdoc, js_name = "createWalletFromBackup")]
    pub async fn create_wallet_from_backup(
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
        .map_err(|e| format!("{e:#?}"))
    }

    /// Creates a wallet backup.
    ///
    /// @param {string} pin - The input string representing the pin.
    /// @param {string} backup_password - Password used to create the backup.
    ///
    /// @returns {Promise<Uint8Array>}
    #[wasm_bindgen(skip_jsdoc, js_name = "createWalletBackup")]
    pub async fn create_wallet_backup(&self, pin: String, backup_password: String) -> Result<Vec<u8>, String> {
        let mut sdk = self.inner.write().await;
        async move {
            let pin = EncryptionPin::try_from_string(pin)?;
            let backup_password = PlainPassword::try_from_string(backup_password)?;
            sdk.create_wallet_backup(&pin, &backup_password).await
        }
        .await
        .map_err(|e| format!("{e:#?}"))
    }

    /// Deletes and existing wallet.
    ///
    /// @param {string} pin - The input string representing the pin.
    ///
    /// @returns {Promise<void>}
    #[wasm_bindgen(skip_jsdoc, js_name = "deleteWallet")]
    pub async fn delete_wallet(&self, pin: String) -> Result<(), String> {
        let mut sdk = self.inner.write().await;
        async move {
            let pin = EncryptionPin::try_from_string(pin)?;
            sdk.delete_wallet(&pin).await
        }
        .await
        .map_err(|e| format!("{e:#?}"))
    }

    /// Verify if the provided mnemonic is the one stored in the wallet.
    ///
    /// @param {string} pin - The input string representing the pin.
    /// @param {string} mnemonic - The input string representing the mnemonic.
    ///
    /// @returns {Promise<boolean>} - whether the mnemonics are the same or not.
    #[wasm_bindgen(skip_jsdoc, js_name = "verifyMnemonic")]
    pub async fn verify_mnemonic(&self, pin: String, mnemonic: String) -> Result<bool, String> {
        let mut sdk = self.inner.write().await;
        async move {
            let pin = EncryptionPin::try_from_string(pin)?;
            sdk.verify_mnemonic(&pin, &mnemonic).await
        }
        .await
        .map_err(|e| format!("{e:#?}"))
    }

    /// Generate a new receiver address based on selected network in the config.
    ///
    /// @param {string} pin - The input string representing the pin.
    ///
    /// @returns {Promise<string>} The receiver wallet address as string
    #[wasm_bindgen(skip_jsdoc, js_name = "generateNewAddress")]
    pub async fn generate_new_address(&self, pin: String) -> Result<String, String> {
        let mut sdk = self.inner.write().await;
        async move {
            let pin = EncryptionPin::try_from_string(pin)?;
            sdk.generate_new_address(&pin).await
        }
        .await
        .map_err(|e| format!("{e:#?}"))
    }

    /// Fetches the current balance of the base crypto network on the wallet
    ///
    /// @param {string} pin - The input string representing the pin.
    ///
    /// @returns {Promise<number>} The current balance as a double precision floating point number
    #[wasm_bindgen(skip_jsdoc, js_name = "getWalletBalance")]
    pub async fn get_wallet_balance(&self, pin: String) -> Result<f64, String> {
        let mut sdk = self.inner.write().await;
        async move {
            let pin = EncryptionPin::try_from_string(pin)?;
            sdk.get_balance(&pin).await.and_then(f64::try_from)
        }
        .await
        .map_err(|e| format!("{e:#?}"))
    }

    /// Initialize the KYC process for Postident
    ///
    /// @remarks
    /// This method is only available if the SDK is compiled with support for postident.
    ///    
    /// @returns {Promise<string>} The ID of the new Postident KYC case.
    #[wasm_bindgen(skip_jsdoc, js_name = "startKycVerificationForPostident")]
    pub async fn start_kyc_verification_for_postident(&self) -> Result<NewCaseIdResponse, String> {
        sdk::require_feature!("postident", {
            let mut sdk = self.inner.write().await;
            sdk.start_kyc_verification_for_postident()
                .await
                .map(|v| NewCaseIdResponse {
                    case_id: v.case_id,
                    case_url: v.case_url,
                })
                .map_err(|e| format!("{e:#?}"))
        })
    }

    /// Fetches the KYC details for the postident provider
    ///
    /// @remarks
    /// This method is only available if the SDK is compiled with support for postident.
    ///
    /// @returns {Promise<CaseDetailsResponse>} The case details
    #[wasm_bindgen(skip_jsdoc, js_name = "getKycDetailsForPostident")]
    pub async fn get_kyc_details_for_postident(&self) -> Result<CaseDetailsResponse, String> {
        sdk::require_feature!("postident", {
            let sdk = self.inner.write().await;
            sdk.get_kyc_details_for_postident()
                .await
                .map(|v| CaseDetailsResponse {
                    archived: v.archived,
                    case_id: v.case_id,
                    status: v.status,
                })
                .map_err(|e| format!("{e:#?}"))
        })
    }

    /// Triggers the backend to update the KYC status in the postident KYC provider
    ///
    /// @remarks
    /// This method is only available if the SDK is compiled with support for postident.
    ///
    /// @param {string} case_id - The input string representing the case_id to be updated
    /// @returns {Promise<void>}
    #[wasm_bindgen(skip_jsdoc, js_name = "updateKycStatusForPostident")]
    pub async fn update_kyc_status_for_postident(&self, case_id: String) -> Result<(), String> {
        sdk::require_feature!("postident", {
            let sdk = self.inner.write().await;
            sdk.update_kyc_status_for_postident(&case_id)
                .await
                .map_err(|e| format!("{e:#?}"))
        })
    }

    /// Creates a purchase request for buying an artefact
    ///
    /// @param receiver - The receiver of the purchase request.
    /// @param amount - The amount of the purchase.
    /// @param product_hash - The hash of the underlying product/artefact
    /// @param app_data - The app data for the purchase. This is application specific string or stringified object data.
    /// @param purchase_type - The type of the purchase. Either a COMPLIMENT or a PURCHASE
    ///
    /// @returns {Promise<string>} The purchase id of the created purchase request as string
    #[wasm_bindgen(skip_jsdoc, js_name = "createPurchaseRequest")]
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
        .map_err(|e| format!("{e:#?}"))
    }

    /// Fetches the purchase details from the given purchase ID.
    ///
    /// @param {string} purchase_id - The purchase id to query to details.
    ///
    /// @returns {Promise<PurchseDetails>} The purchase details
    #[wasm_bindgen(skip_jsdoc, js_name = "getPurchaseDetails")]
    pub async fn get_purchase_details(&self, purchase_id: String) -> Result<PurchaseDetails, String> {
        let sdk = self.inner.write().await;
        sdk.get_purchase_details(&purchase_id)
            .await
            .and_then(|v| {
                let invalid_reasons = match v.clone().status {
                    sdk::types::ApiTxStatus::WaitingForVerification(r) => r,
                    sdk::types::ApiTxStatus::Invalid(r) => r,
                    _ => Vec::new(),
                };

                Ok(PurchaseDetails {
                    main_address: v.system_address,
                    amount: f64::try_from(v.amount)?,
                    status: v.status.into(),
                    invalid_reasons,
                })
            })
            .map_err(|e| format!("{e:#?}"))
    }

    /// Confirm the purchase for the given purchase ID.
    ///
    /// @param {string} pin - The pin for confirmation of purchase
    /// @param {string} purchase_id - The purchase id to confirm.
    /// @returns {Promise<void>}
    #[wasm_bindgen(skip_jsdoc, js_name = "confirmPurchaseRequest")]
    pub async fn confirm_purchase_request(&self, pin: String, purchase_id: String) -> Result<(), String> {
        let mut sdk = self.inner.write().await;
        async move {
            let pin = EncryptionPin::try_from_string(pin)?;
            sdk.confirm_purchase_request(&pin, &purchase_id).await
        }
        .await
        .map_err(|e| format!("{e:#?}"))
    }
    /// Set the password to use for wallet operations. If the password was already set, this changes it.
    ///
    /// @param {string} pin - The pin used to encrypt the password
    /// @param {string} new_password - The password to set for the wallet
    /// @returns {Promise<void>}
    #[wasm_bindgen(skip_jsdoc, js_name = "setWalletPassword")]
    pub async fn set_wallet_password(&self, pin: String, new_password: String) -> Result<(), String> {
        let mut sdk = self.inner.write().await;
        async move {
            let pin = EncryptionPin::try_from_string(pin)?;
            let new_password = PlainPassword::try_from_string(new_password)?;
            sdk.set_wallet_password(&pin, &new_password).await
        }
        .await
        .map_err(|e| format!("{e:#?}"))
    }

    /// Check if the password to use for wallet operations is set.
    /// Use {@link setWalletPassword} to set a new or change an existing password.
    ///
    /// @returns {Promise<bool>}
    #[wasm_bindgen(skip_jsdoc, js_name = "isWalletPasswordSet")]
    pub async fn is_wallet_password_set(&self) -> Result<bool, String> {
        let sdk = self.inner.write().await;
        sdk.is_wallet_password_set().await.map_err(|e| format!("{e:#?}"))
    }

    /// Verifies the pin for the wallet
    ///
    /// @param {string} pin - The pin to be verified
    /// @returns {Promise<void>}
    #[wasm_bindgen(skip_jsdoc, js_name = "verifyPin")]
    pub async fn verify_pin(&self, pin: String) -> Result<(), String> {
        let sdk = self.inner.write().await;
        async move {
            let pin = EncryptionPin::try_from_string(pin)?;
            sdk.verify_pin(&pin).await
        }
        .await
        .map_err(|e| format!("{e:#?}"))
    }

    /// Change the pin used to encrypt the wallet password
    ///
    /// @param {string} pin - The old pin
    /// @param {string} new_pin - The new pin to be set for the wallet
    /// @returns {Promise<void>}
    #[wasm_bindgen(skip_jsdoc, js_name = "resetPin")]
    pub async fn reset_pin(&self, pin: String, new_pin: String) -> Result<(), String> {
        let mut sdk = self.inner.write().await;
        async move {
            let pin = EncryptionPin::try_from_string(pin)?;
            let new_pin = EncryptionPin::try_from_string(new_pin)?;
            sdk.change_pin(&pin, &new_pin).await
        }
        .await
        .map_err(|e| format!("{e:#?}"))
    }

    /// Sends the given amount to the given address
    ///
    /// @param {string} pin - The pin for verification
    /// @param {string} address - The address of the receiver
    /// @param {number} amount - The amount to send in the selected currency
    /// @param {Uint8Array | undefined} data - The data associated with the transaction. Optional.
    /// @returns {Promise<string>} the transaction id.
    #[wasm_bindgen(skip_jsdoc, js_name = "sendAmount")]
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
        .map_err(|e| format!("{e:#?}"))
    }

    /// Gets the detailed lists of purchases (COMPLIMENTS and PURCHASES)
    ///
    /// @param {number} start - The start page
    /// @param {number} limit - The limit per page
    ///
    /// @returns {Promise<TxList>} The details of the created purchases
    #[wasm_bindgen(skip_jsdoc, js_name = "getTransactionList")]
    pub async fn get_transaction_list(&self, start: u32, limit: u32) -> Result<TxList, String> {
        let sdk = self.inner.write().await;
        sdk.get_tx_list(start, limit)
            .await
            .map(|t| TxList {
                txs: t.txs.into_iter().map(Into::into).collect(),
            })
            .map_err(|e| format!("{e:#?}"))
    }

    /// Gets the current exchange rate for the cryptocurrency to EURO
    ///
    /// @returns {Promise<number>} The exchange rate as a floating point number
    #[wasm_bindgen(skip_jsdoc, js_name = "getExchangeRate")]
    pub async fn get_exchange_rate(&self) -> Result<f64, String> {
        let sdk = self.inner.write().await;
        sdk.get_exchange_rate()
            .await
            .and_then(|amount| Ok(f64::try_from(amount)?))
            .map_err(|e| format!("{e:#?}"))
    }

    /// Deletes the user in etopay. Hazmat!
    ///
    /// @param {string} pin - The wallet pin for confirmation. Optional in case there is an active wallet.
    /// @returns {Promise<void>}
    #[wasm_bindgen(skip_jsdoc, js_name = "deleteUser")]
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
        .map_err(|e| format!("{e:#?}"))
    }

    /// Gets the detailed lists of wallet transactions
    ///
    /// @param {pin} pin - The wallet pin
    /// @param {string} start - The start page
    /// @param {string} limit - The limit per page
    ///
    /// @returns {Promise<WalletTxInfoList>} The list of wallet transactions
    #[wasm_bindgen(skip_jsdoc, js_name = "getWalletTransactionList")]
    pub async fn get_wallet_transaction_list(
        &self,
        pin: String,
        start: usize,
        limit: usize,
    ) -> Result<WalletTxInfoList, String> {
        let mut sdk = self.inner.write().await;
        async move {
            let pin = EncryptionPin::try_from_string(pin)?;
            sdk.get_wallet_tx_list(&pin, start, limit)
                .await
                .map(|l| WalletTxInfoList {
                    transactions: l.transactions.into_iter().map(Into::into).collect(),
                })
        }
        .await
        .map_err(|e| format!("{e:#?}"))
    }

    /// Gets the details of a specific wallet transaction
    ///
    /// @param {pin} pin - The wallet pin
    /// @param {string} tx_id - The ID of the transaction to get details for.
    ///
    /// @returns {Promise<WalletTxInfo} The details of the wallet transaction as a serialized JSON string.
    #[wasm_bindgen(skip_jsdoc, js_name = "getWalletTransaction")]
    pub async fn get_wallet_tx(&self, pin: String, tx_id: String) -> Result<WalletTxInfo, String> {
        let mut sdk = self.inner.write().await;
        async move {
            let pin = EncryptionPin::try_from_string(pin)?;
            sdk.get_wallet_tx(&pin, &tx_id).await.map(Into::into)
        }
        .await
        .map_err(|e| format!("{e:#?}"))
    }

    /// Updates the IBAN of the user
    ///
    /// @param {string} pin - The pin for verification
    /// @param {string} address - The IBAN number to be updated
    ///
    /// @returns {Promise<ViviswapAddressDetail>} The details of the added IBAN
    #[cfg_attr(not(feature = "viviswap-swap"), allow(unused_variables))]
    #[wasm_bindgen(skip_jsdoc, js_name = "updateIbanViviswap")]
    pub async fn update_iban_viviswap(&self, pin: String, address: String) -> Result<ViviswapAddressDetail, String> {
        sdk::require_feature!("viviswap-swap", {
            let mut sdk = self.inner.write().await;
            async move {
                let pin = EncryptionPin::try_from_string(pin)?;
                sdk.update_iban_for_viviswap(&pin, address).await
            }
            .await
            .map(Into::into)
            .map_err(|e| format!("{e:#?}"))
        })
    }

    /// Gets the IBAN of the user
    ///
    /// @returns {Promise<ViviswapAddressDetail} The details of the IBAN
    #[wasm_bindgen(skip_jsdoc, js_name = "getIbanViviswap")]
    pub async fn get_iban_viviswap(&self) -> Result<ViviswapAddressDetail, String> {
        sdk::require_feature!("viviswap-swap", {
            let mut sdk = self.inner.write().await;
            sdk.get_iban_for_viviswap()
                .await
                .map(Into::into)
                .map_err(|e| format!("{e:#?}"))
        })
    }

    /// Creates a payment contract for depositing money in wallet using viviswap [EURO --> Crypto]
    ///
    /// @param {pin} pin - The wallet pin
    /// @returns {Promise<ViviswapDeposit>} The details of the added payment contract
    #[wasm_bindgen(skip_jsdoc, js_name = "createDepositWithViviswap")]
    pub async fn create_deposit_with_viviswap(&self, pin: String) -> Result<ViviswapDeposit, String> {
        sdk::require_feature!("viviswap-swap", {
            let mut sdk = self.inner.write().await;
            async move {
                let pin = EncryptionPin::try_from_string(pin)?;
                sdk.create_deposit_with_viviswap(&pin).await.map(Into::into)
            }
            .await
            .map_err(|e| format!("{e:#?}"))
        })
    }

    /// Creates a payment detail for the wallet crypto address in viviswap
    ///
    /// @param {pin} pin - The wallet pin
    /// @returns {Promise<ViviswapAddressDetail>} The details of the added payment detail
    #[wasm_bindgen(skip_jsdoc, js_name = "createDetailForViviswap")]
    pub async fn create_detail_for_viviswap(&self, pin: String) -> Result<ViviswapAddressDetail, String> {
        sdk::require_feature!("viviswap-swap", {
            let mut sdk = self.inner.write().await;
            async move {
                let pin = EncryptionPin::try_from_string(pin)?;
                sdk.create_detail_for_viviswap(&pin).await.map(Into::into)
            }
            .await
            .map_err(|e| format!("{e:#?}"))
        })
    }

    /// Creates a payment contract for withdrawing money from wallet using viviswap [Crypto --> EUR] and if the pin is provided automatically triggers a withdrawal
    ///
    /// @param {number} amount - The amount to withdraw from the wallet
    /// @param {string | undefined} pin - The pin for verification. Optional.
    /// @param {Uint8Array | undefined} data - The associated data with the transaction. Optional.
    ///
    /// @returns {Promise<ViviswapWithdrawal>} The details of the created payment contract
    #[cfg_attr(not(feature = "viviswap-swap"), allow(unused_variables))]
    #[wasm_bindgen(skip_jsdoc, js_name = "createWithdrawalWithViviswap")]
    pub async fn create_withdrawal_with_viviswap(
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
            .map_err(|e| format!("{e:#?}"))
        })
    }

    /// Gets the detail of a particular swap(deposit or withdrawal) created at viviswap based on the given order id.
    ///
    /// @param {string} order_id - The amount to withdraw from the wallet
    ///
    /// @returns {Promise<Order>} The details of the created order
    #[cfg_attr(not(feature = "viviswap-swap"), allow(unused_variables))]
    #[wasm_bindgen(skip_jsdoc, js_name = "getSwapDetails")]
    pub async fn get_swap_details(&self, order_id: String) -> Result<Order, String> {
        sdk::require_feature!("viviswap-swap", {
            let sdk = self.inner.write().await;
            sdk.get_swap_details(order_id)
                .await
                .map(Into::into)
                .map_err(|e| format!("{e:#?}"))
        })
    }

    /// Gets the detailed lists of swaps (deposit and withdrawal) created at viviswap
    ///
    /// @param {number} start - The start page
    /// @param {number} limit - The limit per page
    ///
    /// @returns {Promise<OrderList>} The list of created orders
    #[cfg_attr(not(feature = "viviswap-swap"), allow(unused_variables))]
    #[wasm_bindgen(skip_jsdoc, js_name = "getSwapList")]
    pub async fn get_swap_list(&self, start: u32, limit: u32) -> Result<OrderList, String> {
        sdk::require_feature!("viviswap-swap", {
            let sdk = self.inner.write().await;
            sdk.get_swap_list(start, limit)
                .await
                .map(|l| OrderList {
                    orders: l.orders.into_iter().map(Into::into).collect(),
                })
                .map_err(|e| format!("{e:#?}"))
        })
    }

    /// Starts the KYC verification process for viviswap
    ///
    /// @param {string} mail - The email address of the user as a string.
    /// @param {boolean} terms_accepted - The terms of conditions accepted flag for the user as a boolean
    ///
    /// @returns {Promise<NewViviswapUser>} The new viviswap user
    #[wasm_bindgen(skip_jsdoc, js_name = "startKycVerificationForViviswap")]
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
                .map_err(|e| format!("{e:#?}"))
        })
    }

    /// Fetches the KYC details for a user by the viviswap onboarding process
    ///
    /// @returns {Promise<ViviswapKycStatus>} The KYC details
    #[wasm_bindgen(skip_jsdoc, js_name = "getKycDetailsForViviswap")]
    pub async fn get_kyc_details_for_viviswap(&self) -> Result<ViviswapKycStatus, String> {
        sdk::require_feature!("viviswap-kyc", {
            let mut sdk = self.inner.write().await;
            sdk.get_kyc_details_for_viviswap()
                .await
                .map(Into::into)
                .map_err(|e| format!("{e:#?}"))
        })
    }

    /// Updates the partial KYC details for the viviswap onboarding process
    ///
    /// @param {boolean} is_individual - Flag indicating if the user is an individual.
    /// @param {boolean} is_pep - Flag indicating if the user is a politically exposed person.
    /// @param {boolean} is_us_citizen - Flag indicating if the user is a US citizen.
    /// @param {boolean} is_regulatory_disclosure - Flag indicating if the user has made a regulatory disclosure.
    /// @param {string} country_of_residence - The country of residence of the user.
    /// @param {string} nationality - The nationality of the user.
    /// @param {string} full_name - The full name of the user.
    /// @param {string} date_of_birth - The date of birth of the user.
    ///
    /// @returns {Promise<ViviswapPartiallyKycDetails>} The KYC updated details
    #[wasm_bindgen(skip_jsdoc, js_name = "updateKycPartiallyStatusForViviswap")]
    #[allow(clippy::too_many_arguments)]
    #[cfg_attr(not(feature = "viviswap-kyc"), allow(unused_variables))]
    pub async fn update_kyc_partially_status_for_viviswap(
        &self,
        is_individual: bool,
        is_pep: bool,
        is_us_citizen: bool,
        is_regulatory_disclosure: bool,
        country_of_residence: String,
        nationality: String,
        full_name: String,
        date_of_birth: String,
    ) -> Result<ViviswapPartiallyKycDetails, String> {
        sdk::require_feature!("viviswap-kyc", {
            let mut sdk = self.inner.write().await;
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
            .map(Into::into)
            .map_err(|e| format!("{e:#?}"))
        })
    }

    /// Submits the partial KYC details for the viviswap onboarding process
    /// @returns {Promise<void>}
    #[wasm_bindgen(skip_jsdoc, js_name = "submitKycPartiallyStatusForViviswap")]
    pub async fn submit_kyc_partially_status_for_viviswap(&self) -> Result<(), String> {
        sdk::require_feature!("viviswap-kyc", {
            let mut sdk = self.inner.write().await;
            sdk.submit_kyc_partially_status_for_viviswap()
                .await
                .map_err(|e| format!("{e:#?}"))
        })
    }

    /// Set Viviswap KYC identity details
    ///
    /// @param {OfficialDocumentType} official_document_type - The type of the official document.
    /// @param {string} expiration_date - The expiration date of the official document.
    /// @param {string} document_number - The number of the official document.
    /// @param {Uint8Array} official_document_front_image_data -  The data of the image of the front of the official document.
    /// @param {string} official_document_front_image_filename - The filename (including extension) of the image of the front of the official document.
    /// @param {Uint8Array | undefined} official_document_back_image_data - The data of the image of the back of the official document. Leave as empty string to no provide a back image.
    /// @param {string | undefined} official_document_back_image_filename - The filename (including extension) of the image of the back of the official document. Leave as empty string to no provide a back image.
    /// @param {Uint8Array | undefined} personal_video_data - The data of the 30 second personal video recording.
    /// @param {string} personal_video_filename - The filename (including extenstion) of the 30 second personal video recording.
    /// @returns {Promise<void>}
    #[wasm_bindgen(skip_jsdoc, js_name = "setViviswapKycIdentityDetails")]
    #[allow(clippy::too_many_arguments)]
    #[cfg_attr(not(feature = "viviswap-kyc"), allow(unused_variables))]
    pub async fn set_viviswap_kyc_identity_details(
        &self,
        official_document_type: OfficialDocumentType,
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
            let sdk = self.inner.write().await;

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
                    r#type: official_document_type.into(),
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
            .map_err(|e| format!("{e:#?}"))
        })
    }

    /// Set Viviswap KYC residence details
    ///
    /// @param {string} country_code - User country code
    /// @param {string} region - User region
    /// @param {string} zip_code - User zip code
    /// @param {string} city - User city
    /// @param {string} address_line_1 - User address line 1
    /// @param {string} address_line_2 - User address line 2
    /// @param {boolean} is_public_entry - Inidcates that a valid public entry of this clients address can be found.
    /// @param {string | undefined} public_entry_reference - if `is_public_entry` is `true`, then this must contain the resource link.
    /// @param {boolean} has_no_official_document - indicates if the client does not have any document verifying their address.
    /// @param {Uint8Array | undefined} official_document_image_data - if `has_no_official_document` is `false`, then this must contain the
    ///                                                                 bytes of the document file that verifies that this person is currently living at the address.
    /// @param {string | undefined} official_document_image_filename - the filename (including extension) of the document.
    /// @returns {Promise<void>}
    #[wasm_bindgen(skip_jsdoc, js_name = "setViviswapKycResidenceDetails")]
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
        official_document_image_data: Option<Vec<u8>>,
        official_document_image_filename: Option<String>,
    ) -> Result<(), String> {
        sdk::require_feature!("viviswap-kyc", {
            let sdk = self.inner.write().await;

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
            .map_err(|e| format!("{e:#?}"))
        })
    }

    /// Get the open AMLA KYC questions
    ///
    /// @returns {Promise<OpenAmlaQuestions>} A list of the currently open AMLA questions.
    #[wasm_bindgen(skip_jsdoc, js_name = "getViviswapKycAmlaOpenQuestions")]
    pub async fn get_viviswap_kyc_amla_open_questions(&self) -> Result<OpenAmlaQuestions, String> {
        sdk::require_feature!("viviswap-kyc", {
            let sdk = self.inner.write().await;
            sdk.get_viviswap_kyc_amla_open_questions()
                .await
                .map(|q| OpenAmlaQuestions {
                    questions: q.into_iter().map(Into::into).collect(),
                })
                .map_err(|e| format!("{e:#?}"))
        })
    }

    /// Set the answer to an open AMLA KYC question
    ///
    /// @param {string} question_id - The ID of the question to set the answer to.
    /// @param {string} answers - a list of the selected available answers for the question.
    /// @param {string | undefined} freetext_answer - an optional free-text answer.
    /// @returns {Promise<void>}
    #[wasm_bindgen(skip_jsdoc, js_name = "setViviswapKycAmlaAnswer")]
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
                .map_err(|e| format!("{e:#?}"))
        })
    }

    /// Get the currently open/missing documents for KYC
    ///
    /// @returns {Promise<OpenDocuments>} A list of the currently open documents.
    #[wasm_bindgen(skip_jsdoc, js_name = "getViviswapKycOpenDocuments")]
    pub async fn get_viviswap_kyc_open_documents(&self) -> Result<OpenDocuments, String> {
        sdk::require_feature!("viviswap-kyc", {
            let sdk = self.inner.write().await;
            sdk.get_viviswap_kyc_open_documents()
                .await
                .map(|d| OpenDocuments {
                    documents: d.into_iter().map(Into::into).collect(),
                })
                .map_err(|e| format!("{e:#?}"))
        })
    }

    /// Set / upload an open KYC document
    ///
    /// @param {string} document_id - The ID of the document to upload.
    /// @param {string} expiration_date - the expiration date of this document.
    /// @param {string} document_number - the official document number.
    /// @param {Uint8Array} front_image_data - the image data of the front side of the document.
    /// @param {string} front_image_filename - the filename (including extension) of the front side of the document.
    /// @param {Uint8Array | undefined} back_image_data - the image data of the back side of the document. Leave empty for not specifying a back side image.
    /// @param {string} back_image_filename - the filename (including extension) of the back side of the document. Leave empty for not specifying a back side image.
    /// @returns {Promise<void>}
    #[wasm_bindgen(skip_jsdoc, js_name = "setViviswapKycDocument")]
    #[allow(clippy::too_many_arguments)]
    #[cfg_attr(not(feature = "viviswap-kyc"), allow(unused_variables))]
    pub async fn set_viviswap_kyc_document(
        &self,
        document_id: String,
        expiration_date: String,
        document_number: String,
        front_image_data: Option<Vec<u8>>,
        front_image_filename: Option<String>,
        back_image_data: Option<Vec<u8>>,
        back_image_filename: Option<String>,
    ) -> Result<(), String> {
        sdk::require_feature!("viviswap-kyc", {
            let sdk = self.inner.write().await;

            let front_image = if let (Some(filename), Some(data)) = (front_image_filename, front_image_data) {
                Some(File::from_bytes(&data, &filename))
            } else {
                None
            };

            let back_image = if let (Some(filename), Some(data)) = (back_image_filename, back_image_data) {
                Some(File::from_bytes(&data, &filename))
            } else {
                None
            };

            sdk.set_viviswap_kyc_document(document_id, expiration_date, document_number, front_image, back_image)
                .await
                .map_err(|e| format!("{e:#?}"))
        })
    }

    /// Get the recovery share.
    ///
    /// @returns {Promise<string?>} The recovery share as a string, or `undefined` if none exists.
    #[wasm_bindgen(skip_jsdoc, js_name = "getRecoveryShare")]
    pub async fn get_recovery_share(&self) -> Result<Option<String>, String> {
        use sdk::secrecy::ExposeSecret;

        let sdk = self.inner.write().await;
        sdk.get_recovery_share()
            .await
            .map(|s| s.map(|s| s.to_string().expose_secret().to_string()))
            .map_err(|err| format!("{:#?}", err))
    }

    /// Set the recovery share.
    ///
    /// @param {string} share The recovery share to set.
    /// @returns {Promise<void>}
    #[wasm_bindgen(skip_jsdoc, js_name = "setRecoveryShare")]
    pub async fn set_recovery_share(&self, share: String) -> Result<(), String> {
        let mut sdk = self.inner.write().await;

        let share: sdk::share::Share = share.parse().map_err(|e| format!("{e:#?}"))?;
        sdk.set_recovery_share(share).await.map_err(|err| format!("{:#?}", err))
    }

    /// Get the preferred network.
    ///
    /// @returns {Promise<String?>} The id of preferred network id, or `undefined` if none exists.
    #[wasm_bindgen(skip_jsdoc, js_name = "getPreferredNetwork")]
    pub async fn get_preferred_network(&self) -> Result<Option<String>, String> {
        let sdk = self.inner.write().await;
        sdk.get_preferred_network().await.map_err(|e| format!("{e:#?}"))
    }

    /// Set the preferred network.
    ///
    /// @param {String?} network - the id of the network, or null to reset it.
    ///
    /// @returns {Promise<()>}
    #[wasm_bindgen(skip_jsdoc, js_name = "setPreferredNetwork")]
    pub async fn set_preferred_network(&self, network: Option<String>) -> Result<(), String> {
        let mut sdk = self.inner.write().await;
        sdk.set_preferred_network(network).await.map_err(|e| format!("{e:#?}"))
    }

    /// Get sdk build information.
    ///
    /// @returns {Promise<string?>} The sdk build information as a string.
    #[wasm_bindgen(skip_jsdoc, js_name = "getBuildInfo")]
    pub fn get_build_info(&self) -> String {
        Sdk::get_build_info()
    }
}
