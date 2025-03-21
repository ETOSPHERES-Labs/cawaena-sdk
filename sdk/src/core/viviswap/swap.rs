use crate::backend::viviswap::{
    delete_viviswap_detail, get_viviswap_details, get_viviswap_order, get_viviswap_orders, get_viviswap_payment_method,
    set_viviswap_contract, set_viviswap_detail,
};
use crate::core::viviswap::ViviswapError;
use crate::core::Sdk;
use crate::error::Result;
use crate::types::currencies::{CryptoAmount, Currency};
use crate::types::newtypes::EncryptionPin;
use crate::types::viviswap::{
    ViviswapAddressDetail, ViviswapDeposit, ViviswapDepositDetails, ViviswapDetailUpdateStrategy, ViviswapWithdrawal,
    ViviswapWithdrawalDetails,
};
use api_types::api::viviswap::contract::ViviswapApiContractDetails;
use api_types::api::viviswap::detail::SwapPaymentDetailKey;
use api_types::api::viviswap::order::{Order, OrderList};
use log::{debug, info};
use rust_decimal_macros::dec;

impl Sdk {
    /// Get current iban of viviswap user
    ///
    /// # Arguments
    ///
    /// None
    ///
    /// # Returns
    ///
    /// - `Result<ViviswapAddressDetail>` - The current IBAN of the viviswap user.
    ///
    /// # Errors
    ///
    /// - [`crate::Error::ViviswapInvalidState`] - If the viviswap state is invalid.
    /// - [`crate::Error::UserRepoNotInitialized`] - If the repository initialization fails.
    /// - [`crate::Error::ViviswapApi`] - If there is an error in the viviswap API.
    /// - [`crate::Error::UserStatusUpdateError`] - If there is an error updating the user status.
    // MARK1:get_iban_for_viviswap
    pub async fn get_iban_for_viviswap(&mut self) -> Result<ViviswapAddressDetail> {
        info!("Getting IBAN for viviswap");
        // load user entity
        let mut user = self.get_user().await?;

        // check if user has already a viviswap state available
        let Some(mut viviswap_state) = user.viviswap_state else {
            return Err(crate::Error::Viviswap(ViviswapError::UserStateExisting));
        };

        let Some(repo) = &mut self.repo else {
            return Err(crate::Error::UserRepoNotInitialized);
        };

        // get iban from store
        if let Some(iban) = viviswap_state.current_iban {
            // try to get it from backend maybe
            Ok(iban)
        } else {
            let access_token = self
                .access_token
                .as_ref()
                .ok_or(crate::error::Error::MissingAccessToken)?;
            let config = self.config.as_ref().ok_or(crate::Error::MissingConfig)?;
            let details = get_viviswap_details(config, access_token, SwapPaymentDetailKey::Sepa).await?;
            // update it internally
            // get the first detail
            if let Some(detail) = details.payment_detail.first() {
                let new_detail = ViviswapAddressDetail {
                    id: detail.id.clone(),
                    address: detail.address.clone(),
                    is_verified: detail.is_verified.unwrap_or(false),
                };
                viviswap_state.current_iban = Option::Some(new_detail.clone());
                user.viviswap_state = Some(viviswap_state.clone());
                repo.update(&user)?;
                Ok(new_detail)
            } else {
                Err(crate::Error::Viviswap(ViviswapError::Api(
                    "Unable to find IBAN at viviswap".to_string(),
                )))
            }
        }
    }

    /// Ensure details
    ///
    /// # Arguments
    ///
    /// - `address` - The address to ensure.
    /// - `payment_method_key` - The payment method key.
    /// - `some_update_strategy` - The update strategy for the detail.
    ///
    /// # Returns
    ///
    /// - `Result<ViviswapAddressDetail>` - The ensured Viviswap address detail.
    ///
    /// # Errors
    ///
    /// - [`crate::Error::ViviswapApi`] - If there is an error loading payment details from the Viviswap API.
    // MARK2:ensure_detail
    async fn ensure_detail(
        &self,
        address: String,
        payment_method_key: SwapPaymentDetailKey,
        some_update_strategy: ViviswapDetailUpdateStrategy,
    ) -> Result<ViviswapAddressDetail> {
        debug!("Ensuring payment detail for viviswap");
        // load user entity
        let _user = self.get_user().await?;
        let access_token = self
            .access_token
            .as_ref()
            .ok_or(crate::error::Error::MissingAccessToken)?;
        let config = self.config.as_ref().ok_or(crate::Error::MissingConfig)?;

        // get all viviswap details for specific payment-method
        let details = get_viviswap_details(config, access_token, payment_method_key).await?;
        debug!("{details:#?}");
        // search for address in existing details
        for detail in &details.payment_detail {
            debug!("{detail:#?}");
            let add = detail.address.clone();
            debug!("{add:#?}");
            debug!("{address:#?}");
            if detail.address == address {
                return Ok(ViviswapAddressDetail {
                    id: detail.id.to_string(),
                    address: detail.address.to_string(),
                    is_verified: detail.is_verified.unwrap_or(false),
                });
            }
        }

        // 1. creates a new detail
        let new_detail = set_viviswap_detail(config, access_token, payment_method_key, &address).await?;

        // set_viviswap_detail should fill this out, but if not, we need to return an error
        let Some(detail) = new_detail.payment_detail else {
            return Err(crate::Error::Viviswap(ViviswapError::Api(
                "no payment details returned".into(),
            )));
        };

        // 2. handle the update strategy
        if let (ViviswapDetailUpdateStrategy::Replace, Some(old_detail)) =
            (some_update_strategy, details.payment_detail.first())
        {
            delete_viviswap_detail(config, access_token, payment_method_key, old_detail.id.as_str()).await?
        }

        // 3. returns the new detail
        Ok(ViviswapAddressDetail {
            id: detail.id,
            address: detail.address,
            is_verified: detail.is_verified.unwrap_or(false),
        })
    }

    /// Update IBAN of viviswap user.
    ///
    /// # Arguments
    ///
    /// - `pin` - The user's PIN.
    /// - `address` - The new IBAN address.
    ///
    /// # Returns
    ///
    /// - `Result<ViviswapAddressDetail>` - The updated Viviswap address detail.
    ///
    /// # Errors
    ///
    /// - [`crate::Error::UserRepoNotInitialized`] - If the repository initialization fails.
    /// - [`crate::Error::ViviswapMissingUserError`] - If the viviswap user is missing.
    /// - [`crate::Error::UserStatusUpdateError`] - If there is an error updating the user status.
    // MARK3:update_iban_for_viviswap
    pub async fn update_iban_for_viviswap(
        &mut self,
        pin: &EncryptionPin,
        address: String,
    ) -> Result<ViviswapAddressDetail> {
        info!("Updating user IBAN");
        // verify pin
        self.verify_pin(pin).await?;

        // ensure that the repository exist (cannot borrow as mutable here since we also borrow self as mutable in between)
        if self.repo.is_none() {
            return Err(crate::Error::UserRepoNotInitialized);
        }

        // load user entity
        let mut user = self.get_user().await?;

        // check if user has already a viviswap state available
        let Some(mut viviswap_state) = user.viviswap_state else {
            return Err(crate::Error::Viviswap(ViviswapError::MissingUser));
        };

        // 1. check if iban is already saved to service
        if let Some(current_iban) = viviswap_state.current_iban {
            if current_iban.address == address {
                return Ok(current_iban.clone());
            }
        }

        // 2. check if iban does already exist
        let new_detail_response = self
            .ensure_detail(
                address,
                SwapPaymentDetailKey::Sepa,
                ViviswapDetailUpdateStrategy::Replace,
            )
            .await?;
        let new_detail = new_detail_response;

        // 3. update storage

        // load repository
        if let Some(repo) = &mut self.repo {
            viviswap_state.current_iban = Option::Some(new_detail.clone());
            user.viviswap_state = Some(viviswap_state.clone());
            repo.update(&user)?;
        } else {
            return Err(crate::Error::UserRepoNotInitialized);
        };

        Ok(new_detail)
    }

    /// create deposit for viviswap user
    ///
    /// # Returns
    ///
    /// - `Result<ViviswapDeposit>` - The created Viviswap deposit.
    ///
    /// # Errors
    ///
    /// - [`crate::Error::UserRepoNotInitialized`] - If the repository initialization fails.
    /// - [`crate::Error::ViviswapMissingUserError`] - If the viviswap user is missing.
    /// - [`crate::Error::ViviswapInvalidState`] - If the viviswap state is invalid.
    /// - [`crate::Error::ViviswapApi`] - If there is an error with the Viviswap API.
    // MARK4:create_deposit_with_viviswap
    pub async fn create_deposit_with_viviswap(&mut self, pin: &EncryptionPin) -> Result<ViviswapDeposit> {
        info!("Creating deposit for viviswap");
        // load user entity
        let user = self.get_user().await?;
        let address = self.generate_new_address(pin).await?;

        // check if user has already a viviswap state available
        let Some(viviswap_state) = user.viviswap_state else {
            return Err(crate::Error::Viviswap(ViviswapError::MissingUser));
        };

        // check if iban exists, otherwise error
        let Some(iban_detail) = viviswap_state.current_iban else {
            return Err(crate::Error::Viviswap(ViviswapError::InvalidState));
        };

        let iban_method_id = self.get_payment_method_id_viviswap(SwapPaymentDetailKey::Sepa).await?;
        let network = self.network.clone().ok_or(crate::Error::MissingNetwork)?;
        let currency = Currency::try_from(network.display_symbol)?;

        let payment_method_key = currency.to_vivi_payment_method_key();

        let coin_method_id = self.get_payment_method_id_viviswap(payment_method_key).await?;

        let coin_detail = self
            .ensure_detail(address, payment_method_key, ViviswapDetailUpdateStrategy::Add)
            .await?;
        let config = self.config.as_ref().ok_or(crate::Error::MissingConfig)?;

        let access_token = self
            .access_token
            .as_ref()
            .ok_or(crate::error::Error::MissingAccessToken)?;
        let contract_response = set_viviswap_contract(
            config,
            access_token,
            CryptoAmount::try_from(dec!(25.0))?, // any random amount works here!
            iban_method_id,
            Option::Some(iban_detail.id),
            coin_method_id,
            coin_detail.id,
        )
        .await?;

        let new_contract = contract_response
            .contract
            .ok_or(crate::Error::Viviswap(ViviswapError::Api(String::from(
                "Error creating the new contract for user.",
            ))))?;
        let bank_details = new_contract
            .details
            .ok_or(crate::Error::Viviswap(ViviswapError::Api(String::from(
                "The new contract has invalid state. Deposit details are missing!",
            ))))?;

        match bank_details {
            ViviswapApiContractDetails::BankAccount(account_details) => Ok(ViviswapDeposit {
                contract_id: new_contract.id,
                deposit_address: coin_detail.address,
                details: ViviswapDepositDetails {
                    reference: new_contract.reference,
                    beneficiary: account_details.beneficiary,
                    name_of_bank: account_details.name_of_bank,
                    address_of_bank: account_details.address_of_bank,
                    iban: account_details.address,
                    bic: account_details.bic,
                },
            }),
            _ => Err(crate::Error::Viviswap(ViviswapError::Api(String::from(
                "The new contract has invalid state. Bank deposit details are missing!",
            )))),
        }
    }

    /// create detail for viviswap user
    ///
    /// # Returns
    ///
    /// - `Result<ViviswapAddressDetail>` - The created Viviswap address detail.
    ///
    /// # Errors
    ///
    /// - [`crate::Error::ConfigInitError`] - If there is an error initializing the configuration.
    /// - [`crate::Error::ViviswapMissingUserError`] - If the viviswap user is missing.
    // MARK5:create_detail_for_viviswap
    pub async fn create_detail_for_viviswap(&mut self, pin: &EncryptionPin) -> Result<ViviswapAddressDetail> {
        let network = self.network.clone().ok_or(crate::Error::MissingNetwork)?;
        let currency = Currency::try_from(network.display_symbol)?;
        let payment_method_key = currency.to_vivi_payment_method_key();

        info!("Creating a payment detail for viviswap for {payment_method_key:?}");
        // load user entity
        let user = self.get_user().await?;

        // check if user has already a viviswap state available
        if user.viviswap_state.is_none() {
            return Err(crate::Error::Viviswap(ViviswapError::MissingUser));
        }
        let address = self.generate_new_address(pin).await?;
        // 1. check if address does already exist
        let new_detail = self
            .ensure_detail(address, payment_method_key, ViviswapDetailUpdateStrategy::Add)
            .await?;

        Ok(new_detail)
    }

    /// get payment methods and get the id for the given method key
    ///
    /// # Arguments
    ///
    /// * `payment_method_key` - The key of the payment method.
    ///
    /// # Returns
    ///
    /// - `Result<String>` - The ID of the payment method.
    ///
    /// # Errors
    ///
    /// - [`crate::Error::UserRepoNotInitialized`] - If there is an error initializing the repository.
    /// - [`crate::Error::ViviswapMissingUserError`] - If the viviswap user is missing.
    /// - [`crate::Error::ViviswapApi`] - If the payment method is not found.
    // MARK6:get_payment_method_id_viviswap
    async fn get_payment_method_id_viviswap(&mut self, payment_method_key: SwapPaymentDetailKey) -> Result<String> {
        let mut user = self.get_user().await?;

        // load repository
        let Some(repo) = &mut self.repo else {
            return Err(crate::Error::UserRepoNotInitialized);
        };

        // check if user has already a viviswap state available
        let Some(mut viviswap_state) = user.viviswap_state else {
            return Err(crate::Error::Viviswap(ViviswapError::MissingUser));
        };

        let payment_methods = match viviswap_state.payment_methods {
            Some(details) => details,
            None => {
                let access_token = self
                    .access_token
                    .as_ref()
                    .ok_or(crate::error::Error::MissingAccessToken)?;
                let config = self.config.as_ref().ok_or(crate::Error::MissingConfig)?;
                let payment_methods = get_viviswap_payment_method(config, access_token).await?;
                viviswap_state.payment_methods = Some(payment_methods.clone());
                user.viviswap_state = Some(viviswap_state.clone());
                repo.update(&user)?;
                payment_methods
            }
        };

        let method_id = payment_methods
            .methods
            .iter()
            .find(|&method| method.key == payment_method_key)
            .map(|method| method.id.clone())
            .ok_or_else(|| {
                crate::Error::Viviswap(ViviswapError::Api(format!(
                    "Payment method not found for key: {payment_method_key:?}"
                )))
            })?;

        Ok(method_id)
    }

    /// create withdrawal for viviswap user
    ///
    /// # Arguments
    ///
    /// * `amount` - The amount of the withdrawal.
    /// * `pin` - The optional PIN for verification.
    /// * `tag` - The transactions tag. Optional.
    /// * `data` - The associated data with the tag. Optional.
    /// * `message` - The transactions message. Optional.
    ///
    /// # Returns
    ///
    /// - `Result<ViviswapWithdrawal>` - The created Viviswap withdrawal.
    ///
    /// # Errors
    ///
    /// - [`crate::Error::ViviswapMissingUserError`] - If the viviswap user is missing.
    /// - [`crate::Error::ViviswapInvalidState`] - If the viviswap state is invalid.
    /// - [`crate::Error::ViviswapApi`] - If there is an error with the Viviswap API.
    // MARK7:create_withdrawal_with_viviswap
    pub async fn create_withdrawal_with_viviswap(
        &mut self,
        amount: CryptoAmount,
        pin: Option<&EncryptionPin>,
        data: Option<Vec<u8>>,
    ) -> Result<ViviswapWithdrawal> {
        info!("Creating withdrawal with viviswap");
        // load user entity
        if let Some(pin) = pin {
            self.verify_pin(pin).await?;
        }
        let user = self.get_user().await?;

        // check if user has already a viviswap state available
        let Some(viviswap_state) = user.viviswap_state else {
            return Err(crate::Error::Viviswap(ViviswapError::MissingUser));
        };

        let network = self.network.clone().ok_or(crate::Error::MissingNetwork)?;
        let currency = Currency::try_from(network.display_symbol)?;

        // check if iban exists, otherwise error
        let Some(iban_detail) = viviswap_state.current_iban else {
            return Err(crate::Error::Viviswap(ViviswapError::InvalidState));
        };

        let iban_method_id = self.get_payment_method_id_viviswap(SwapPaymentDetailKey::Sepa).await?;

        let coin_method_id = self
            .get_payment_method_id_viviswap(currency.to_vivi_payment_method_key())
            .await?;

        let access_token = self
            .access_token
            .as_ref()
            .ok_or(crate::error::Error::MissingAccessToken)?;
        let config = self.config.as_ref().ok_or(crate::Error::MissingConfig)?;

        let contract_response = set_viviswap_contract(
            config,
            access_token,
            amount,
            coin_method_id,
            Option::None,
            iban_method_id,
            iban_detail.id.clone(),
        )
        .await?;

        let new_contract = contract_response
            .contract
            .ok_or(crate::Error::Viviswap(ViviswapError::Api(String::from(
                "Error creating the new contract for user.",
            ))))?;
        let withdrawal_details =
            new_contract
                .details
                .ok_or(crate::Error::Viviswap(ViviswapError::Api(String::from(
                    "The new contract has invalid state. Withdrawal details are missing!",
                ))))?;

        match withdrawal_details {
            ViviswapApiContractDetails::Crypto(crypto_details) => {
                if let Some(pin) = pin {
                    self.send_amount(pin, &crypto_details.deposit_address, amount, data)
                        .await?;
                }
                Ok(ViviswapWithdrawal {
                    contract_id: new_contract.id,
                    deposit_address: iban_detail.address.clone(),
                    details: ViviswapWithdrawalDetails {
                        reference: new_contract.reference,
                        wallet_id: crypto_details.wallet_id,
                        crypto_address: crypto_details.deposit_address,
                    },
                })
            }
            _ => Err(crate::Error::Viviswap(ViviswapError::Api(String::from(
                "The new contract has invalid state. Crypto deposit details are missing!",
            )))),
        }
    }

    /// Get the list of swaps for the viviswap user.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing a vector of `Swap` if successful, or a [`crate::Error`] if an error occurs.
    ///
    /// # Errors
    ///
    /// Returns an `Err` variant of [`crate::Error`] if any of the following conditions are met:
    ///
    /// * Repository initialization error.
    /// * Viviswap API error.
    // MARK8:get_swap_list
    pub async fn get_swap_list(&self, start: u32, limit: u32) -> Result<OrderList> {
        let Some(_user) = &self.active_user else {
            return Err(crate::Error::UserRepoNotInitialized);
        };
        let config = self.config.as_ref().ok_or(crate::Error::MissingConfig)?;
        info!("get_swap_list request");

        let access_token = self
            .access_token
            .as_ref()
            .ok_or(crate::error::Error::MissingAccessToken)?;
        let orders = get_viviswap_orders(config, access_token, start, limit).await?;
        Ok(orders)
    }

    /// Get swap details
    ///
    /// # Arguments
    ///
    /// * `order_id` - The ID of the swap order.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the swap order details or an error.
    // MARK9:get_swap_details
    pub async fn get_swap_details(&self, order_id: String) -> Result<Order> {
        let Some(_user) = &self.active_user else {
            return Err(crate::Error::UserRepoNotInitialized);
        };
        let config = self.config.as_ref().ok_or(crate::Error::MissingConfig)?;
        info!("get_swap_details request for order_id: {order_id}");
        let access_token = self
            .access_token
            .as_ref()
            .ok_or(crate::error::Error::MissingAccessToken)?;
        match get_viviswap_order(config, access_token, &order_id).await {
            Ok(order_detail) => Ok(order_detail),
            Err(_) => Err(crate::Error::Viviswap(ViviswapError::Api(format!(
                "Swap id:{order_id} not found"
            )))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing_utils::{
        example_api_network, example_api_networks, example_bank_details, example_contract_response,
        example_crypto_details, example_exchange_rate_response, example_get_payment_details_response, example_get_user,
        example_viviswap_oder_response, set_config, ADDRESS, AUTH_PROVIDER, HEADER_X_APP_NAME, ORDER_ID, PIN, TOKEN,
        USERNAME,
    };
    //use crate::types::networks::Network;
    use crate::types::users::KycType;
    use crate::{
        core::Sdk,
        types::users::ActiveUser,
        user::MockUserRepo,
        wallet_manager::{MockWalletManager, WalletBorrow},
        wallet_user::MockWalletUser,
    };
    use api_types::api::networks::ApiNetwork;
    use api_types::api::{dlt::SetUserAddressRequest, viviswap::order::GetOrdersResponse};
    use mockito::Matcher;
    use rand::Rng;
    use rstest::rstest;

    /// Create an active user
    fn get_active_user() -> ActiveUser {
        ActiveUser {
            username: USERNAME.into(),
            wallet_manager: Box::new(MockWalletManager::new()),
        }
    }

    #[tokio::test]
    async fn test_err_get_swap_list_should_consume_only_authenticated_requests() {
        // Arrange
        let (_srv, config, _cleanup) = set_config().await;

        let mut sdk = Sdk::new(config).unwrap();
        sdk.repo = Some(Box::new(MockUserRepo::new()));
        sdk.access_token = Some(TOKEN.clone());
        sdk.active_user = None;

        // Call the function you want to test
        let result = sdk.get_swap_list(1, 2).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_err_get_swap_details_should_consume_only_authenticated_requests() {
        // Arrange
        let (_srv, config, _cleanup) = set_config().await;

        let mut sdk = Sdk::new(config).unwrap();
        sdk.repo = Some(Box::new(MockUserRepo::new()));
        sdk.access_token = Some(TOKEN.clone());
        sdk.active_user = None;

        // Call the function you want to test
        let result = sdk.get_swap_details(String::from(ORDER_ID)).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_ok_get_swap_details() {
        // Arrange
        let (mut srv, config, _cleanup) = set_config().await;
        let mut sdk = Sdk::new(config).unwrap();
        sdk.repo = Some(Box::new(MockUserRepo::new()));
        sdk.access_token = Some(TOKEN.clone());
        sdk.active_user = Some(get_active_user());

        let mock_response = example_viviswap_oder_response();
        let body = serde_json::to_string(&mock_response).unwrap();

        let mock_server = srv
            .mock("GET", format!("/api/viviswap/orders?id={}", ORDER_ID).as_str())
            .match_header(HEADER_X_APP_NAME, AUTH_PROVIDER)
            .match_header("authorization", format!("Bearer {}", TOKEN.as_str()).as_str())
            .with_status(200)
            .with_body(&body)
            .with_header("content-type", "application/json")
            .with_body(&body)
            .create();

        // Call the function you want to test
        let result = sdk.get_swap_details(String::from(ORDER_ID)).await;

        // Assert
        assert_eq!(result.unwrap(), example_viviswap_oder_response());
        mock_server.assert();
    }

    #[tokio::test]
    async fn test_ok_get_swap_list() {
        // Arrange
        let (mut srv, config, _cleanup) = set_config().await;
        let mut sdk = Sdk::new(config).unwrap();
        sdk.repo = Some(Box::new(MockUserRepo::new()));
        sdk.access_token = Some(TOKEN.clone());
        sdk.active_user = Some(get_active_user());

        let mock_order = example_viviswap_oder_response();
        let mock_response = GetOrdersResponse {
            count: 1,
            start: 2,
            limit: 3,
            orders: vec![mock_order],
        };
        let body = serde_json::to_string(&mock_response).unwrap();

        let mock_server = srv
            .mock("GET", "/api/viviswap/orders?start=2&limit=1")
            .match_header(HEADER_X_APP_NAME, AUTH_PROVIDER)
            .match_header("authorization", format!("Bearer {}", TOKEN.as_str()).as_str())
            .with_status(200)
            .with_body(&body)
            .with_header("content-type", "application/json")
            .with_body(&body)
            .create();

        // Call the function you want to test
        let result = sdk.get_swap_list(2, 1).await;

        // Assert
        let result = result.unwrap();
        assert_eq!(result.orders[0].contract_id, mock_response.orders[0].contract_id);
        assert_eq!(result.orders[0].crypto_fees, mock_response.orders[0].crypto_fees);
        assert_eq!(
            result.orders[0].fees_amount_eur,
            mock_response.orders[0].fees_amount_eur
        );
        mock_server.assert();
    }

    #[tokio::test]
    async fn test_err_get_swap_list_server_error() {
        // Arrange
        let (mut srv, config, _cleanup) = set_config().await;
        let mut sdk = Sdk::new(config).unwrap();
        sdk.repo = Some(Box::new(MockUserRepo::new()));
        sdk.access_token = Some(TOKEN.clone());
        sdk.active_user = Some(get_active_user());

        let mock_response = GetOrdersResponse {
            count: 1,
            start: 2,
            limit: 3,
            orders: vec![],
        };
        let body = serde_json::to_string(&mock_response).unwrap();

        let server_error_status = rand::rng().random_range(400..410);
        let mock_server = srv
            .mock("GET", "/api/viviswap/orders?start=2&limit=1")
            .match_header(HEADER_X_APP_NAME, AUTH_PROVIDER)
            .match_header("authorization", format!("Bearer {}", TOKEN.as_str()).as_str())
            .with_status(server_error_status)
            .with_body(&body)
            .with_header("content-type", "application/json")
            .with_body(&body)
            .create();

        // Call the function you want to test
        let result = sdk.get_swap_list(2, 1).await;

        assert!(result.is_err());
        mock_server.assert();
    }

    #[tokio::test]
    async fn test_err_get_swap_details_server_error() {
        // Arrange
        let (mut srv, config, _cleanup) = set_config().await;
        let mut sdk = Sdk::new(config).unwrap();
        sdk.repo = Some(Box::new(MockUserRepo::new()));
        sdk.access_token = Some(TOKEN.clone());
        sdk.active_user = Some(get_active_user());

        let server_error_status = rand::rng().random_range(400..410);
        let mock_server = srv
            .mock("GET", format!("/api/viviswap/orders?id={}", ORDER_ID).as_str())
            .match_header(HEADER_X_APP_NAME, AUTH_PROVIDER)
            .match_header("authorization", format!("Bearer {}", TOKEN.as_str()).as_str())
            .with_status(server_error_status)
            .with_header("content-type", "application/json")
            .create();

        // Call the function you want to test
        let result = sdk.get_swap_details(String::from(ORDER_ID)).await;

        // Assert
        assert!(result.is_err());
        mock_server.assert();
    }

    #[rstest]
    #[case(
        example_api_network(String::from("IOTA")),
        SwapPaymentDetailKey::Iota,
        "/api/viviswap/details?payment_method_key=IOTA"
    )]
    #[case(
        example_api_network(String::from("ETH")),
        SwapPaymentDetailKey::Eth,
        "/api/viviswap/details?payment_method_key=ETH"
    )]
    #[tokio::test]
    async fn it_should_create_viviswap_deposit(
        #[case] network: ApiNetwork,                      // Parametrized network
        #[case] payment_detail_key: SwapPaymentDetailKey, // Payment detail key (Iota, Eth, etc.)
        #[case] payment_method_path: &str,                // The payment method query path
    ) {
        // Arrange
        let (mut srv, config, _cleanup) = set_config().await;
        let mut sdk = Sdk::new(config).unwrap();
        sdk.access_token = Some(TOKEN.clone());

        sdk.set_networks(example_api_networks());
        sdk.set_network(network.key.clone()).await.unwrap(); // Set parametrized network
        sdk.refresh_access_token(Some(TOKEN.clone())).await.unwrap();

        let mock_user_repo = example_get_user(payment_detail_key, false, 5, KycType::Viviswap);
        sdk.repo = Some(Box::new(mock_user_repo));

        let mut mock_wallet_manager = MockWalletManager::new();
        mock_wallet_manager.expect_try_get().returning({
            move |_, _, _, _, _| {
                let mut mock_wallet = MockWalletUser::new();
                mock_wallet
                    .expect_get_address()
                    .once()
                    .returning(move || Ok(ADDRESS.to_string()));
                Ok(WalletBorrow::from(mock_wallet))
            }
        });
        sdk.active_user = Some(ActiveUser {
            username: USERNAME.into(),
            wallet_manager: Box::new(mock_wallet_manager),
        });

        // create viviswap contract
        let contract_mock_response = example_contract_response(example_bank_details());
        let body = serde_json::to_string(&contract_mock_response).unwrap();
        let create_viviswap_contract = srv
            .mock("POST", "/api/viviswap/contracts")
            .match_header(HEADER_X_APP_NAME, AUTH_PROVIDER)
            .match_header("authorization", format!("Bearer {}", TOKEN.as_str()).as_str())
            .with_status(200)
            .with_body(&body)
            .with_header("content-type", "application/json")
            .with_body(&body)
            .create();

        // Get user payment details
        let payment_details_mock_response = example_get_payment_details_response();
        let body = serde_json::to_string(&payment_details_mock_response).unwrap();
        let get_payment_details = srv
            .mock("GET", payment_method_path) // Use the parametrized payment method path
            .match_header(HEADER_X_APP_NAME, AUTH_PROVIDER)
            .match_header("authorization", format!("Bearer {}", TOKEN.as_str()).as_str())
            .with_status(200)
            .with_body(&body)
            .with_header("content-type", "application/json")
            .with_body(&body)
            .create();

        let mock_request = SetUserAddressRequest {
            address: ADDRESS.to_string(),
        };
        let body = serde_json::to_string(&mock_request).unwrap();

        let put_user_address = srv
            .mock("PUT", "/api/user/address")
            .match_header(HEADER_X_APP_NAME, AUTH_PROVIDER)
            .match_header("authorization", format!("Bearer {}", TOKEN.as_str()).as_str())
            .match_query(Matcher::Exact(format!("network_key={}", network.key)))
            .match_body(Matcher::Exact(body))
            .with_status(201)
            .expect(1)
            .with_header("content-type", "application/json")
            .create();

        // Call the function you want to test
        let _ = sdk.create_deposit_with_viviswap(&PIN).await.unwrap();

        // Assert
        put_user_address.assert();
        get_payment_details.assert();
        create_viviswap_contract.assert();
    }

    #[tokio::test]
    async fn it_should_create_withdrawal_deposit_for_iota_and_smr() {
        // Arrange
        let (mut srv, config, _cleanup) = set_config().await;

        let mut sdk = Sdk::new(config).unwrap();
        sdk.set_networks(example_api_networks());
        sdk.set_network(String::from("IOTA")).await.unwrap();

        let mock_user_repo = example_get_user(SwapPaymentDetailKey::Iota, false, 3, KycType::Viviswap);
        sdk.repo = Some(Box::new(mock_user_repo));

        sdk.access_token = Some(TOKEN.clone());
        sdk.active_user = Some(get_active_user());

        // create viviswap contract
        let contract_mock_response = example_contract_response(example_crypto_details());
        let body = serde_json::to_string(&contract_mock_response).unwrap();
        let create_viviswap_contract = srv
            .mock("POST", "/api/viviswap/contracts")
            .match_header(HEADER_X_APP_NAME, AUTH_PROVIDER)
            .match_header("authorization", format!("Bearer {}", TOKEN.as_str()).as_str())
            .with_status(200)
            .with_body(&body)
            .with_header("content-type", "application/json")
            .with_body(&body)
            .create();

        // Call the function you want to test
        let result = sdk
            .create_withdrawal_with_viviswap(dec!(50.0).try_into().unwrap(), None, Some(Vec::from([8, 16])))
            .await;

        // Assert
        result.unwrap();
        create_viviswap_contract.assert();
    }

    #[tokio::test]
    async fn it_should_get_exchange_rate() {
        // Arrange
        let (mut srv, config, _cleanup) = set_config().await;
        let mut sdk = Sdk::new(config).unwrap();
        sdk.set_networks(example_api_networks());
        sdk.set_network(String::from("IOTA")).await.unwrap();

        sdk.repo = Some(Box::new(example_get_user(
            SwapPaymentDetailKey::Iota,
            false,
            1,
            KycType::Viviswap,
        )));
        sdk.active_user = Some(get_active_user());
        sdk.access_token = Some(TOKEN.clone());

        // Get exchange rate
        let exchange_rate_mock_response = example_exchange_rate_response();
        let body = serde_json::to_string(&exchange_rate_mock_response).unwrap();
        let get_exchange_rate = srv
            .mock("GET", "/api/viviswap/courses?currency=Iota")
            .match_header(HEADER_X_APP_NAME, AUTH_PROVIDER)
            .match_header("authorization", format!("Bearer {}", TOKEN.as_str()).as_str())
            .with_status(200)
            .with_body(&body)
            .with_header("content-type", "application/json")
            .with_body(&body)
            .create();

        // Call function you want to test
        let result = sdk.get_exchange_rate().await;

        // Assert
        result.unwrap();
        get_exchange_rate.assert();
    }
}
