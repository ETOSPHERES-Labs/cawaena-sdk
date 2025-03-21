//! Transactions
//!
//! This module contains functions for interacting with transactions in the backend.
//! It provides functions for creating new transactions, committing transactions,
//! getting transaction status, getting transaction details, and getting a list of transactions.
//!
//! The main functions in this module are:
//!
//! - `create_new_transaction`: Creates a new transaction.
//! - `commit_transaction`: Commits a transaction.
//! - `get_transaction_status`: Gets the status of a transaction.
//! - `get_transaction_details`: Gets the details of a transaction.
//! - `get_transactions_list`: Gets a list of transactions.
//!
//! The module also includes several supporting types and structs used in the transaction operations.
//!
//! For more information, see the individual function documentation.
//!

use super::error::{ApiError, Result};
use crate::{
    core::config::Config,
    types::{currencies::CryptoAmount, newtypes::AccessToken},
};
use api_types::api::transactions::{
    ApiApplicationMetadata, CreateTransactionRequest, CreateTransactionResponse, GetTransactionDetailsResponse,
    GetTransactionStatusRequest, GetTxsDetailsResponse, TxsDetailsQuery,
};
use log::{debug, error, info};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

/// Request body to commit a transaction
#[derive(Debug, Deserialize, Serialize, Clone)]
struct CommitTransactionRequest {
    /// unique transaction index
    pub index: String,
    /// transaction id
    pub transaction_id: String,
}

/// Create new transaction
///
/// # Arguments
///
/// * `config` - The configuration object.
/// * `access_token` - The access token for authentication.
/// * `sender` - The sender's username.
/// * `receiver` - The receiver's username.
/// * `amount` - The amount of the transaction.
/// * `metadata` - The application metadata.
///
/// # Returns
///
/// Returns a `Result` containing the `CreateTransactionResponse` if successful, or an `Error` if an error occurs.
///
/// # Errors
///
/// Returns an `Error::Unauthorized` if the request is unauthorized, or an `Error::UnhandledError` if an unhandled error occurs.
pub async fn create_new_transaction(
    config: &Config,
    access_token: &AccessToken,
    receiver: &str,
    network_id: String,
    amount: CryptoAmount,
    metadata: ApiApplicationMetadata,
) -> Result<CreateTransactionResponse> {
    let base_url = &config.backend_url;
    let url = format!("{base_url}/transactions/create");

    let body = CreateTransactionRequest {
        amount: amount.inner(),
        network_key: network_id,
        receiver: receiver.into(),
        application_metadata: metadata,
    };
    info!("Used url: {url:#?}");
    info!("Create new transaction to {receiver} with the amount of {amount:?}");

    let client = reqwest::Client::new();
    let response = client
        .post(&url)
        .bearer_auth(access_token.as_str())
        .header("X-APP-NAME", &config.auth_provider)
        .json(&body)
        .send()
        .await?;
    debug!("Response: {response:#?}");
    let tx_response = match response.status() {
        StatusCode::CREATED => response.json::<CreateTransactionResponse>().await?,
        StatusCode::UNAUTHORIZED => return Err(ApiError::MissingAccessToken),
        _ => {
            let status = response.status();
            let text = response.text().await?;
            error!(
                "Failed to create a new transaction: Response status: {}, Response text: {}",
                status, text
            );
            return Err(ApiError::UnexpectedResponse {
                code: status,
                body: text,
            });
        }
    };

    Ok(tx_response)
}

/// Commit transaction
///
/// # Arguments
///
/// * `config` - The configuration object.
/// * `access_token` - The access token for authentication.
/// * `username` - The username.
/// * `index` - The index of the transaction. This is the internal transaction index
/// * `tx_id` - The ID of the transaction on the DLT network.
///
/// # Returns
///
/// Returns a `Result` containing `()` if successful, or an `Error` if an error occurs.
///
/// # Errors
///
/// Returns an `Error::Unauthorized` if the request is unauthorized, or an `Error::UnhandledError` if an unhandled error occurs.
pub async fn commit_transaction(config: &Config, access_token: &AccessToken, index: &str, tx_id: &str) -> Result<()> {
    let base_url = &config.backend_url;
    let url = format!("{base_url}/transactions/commit");
    let body = CommitTransactionRequest {
        index: index.into(),
        transaction_id: tx_id.into(),
    };
    info!("Used url: {url:#?}");
    info!("Commit transaction for {index}");

    let client = reqwest::Client::new();
    let response = client
        .post(&url)
        .bearer_auth(access_token.as_str())
        .header("X-APP-NAME", &config.auth_provider)
        .json(&body)
        .send()
        .await?;
    debug!("Response: {response:#?}");
    match response.status() {
        StatusCode::ACCEPTED => Ok(()),
        StatusCode::UNAUTHORIZED => Err(ApiError::MissingAccessToken),
        _ => {
            let status = response.status();
            let text = response.text().await?;
            error!(
                "Failed to commit transaction `{index}`: Response status: {}, Response text: {}",
                status, text
            );
            Err(ApiError::UnexpectedResponse {
                code: status,
                body: text,
            })
        }
    }
}

/// Get transaction details
///
/// # Arguments
///
/// * `config` - The configuration object.
/// * `access_token` - The access token for authentication.
/// * `username` - The username.
/// * `index` - The index of the transaction. This is the internal transaction index
///
/// # Returns
///
/// Returns a `Result` containing `GetTransactionDetailsResponse` if successful, or an `Error` if an error occurs.
///
/// # Errors
///
/// Returns an `Error::Unauthorized` if the request is unauthorized, or an `Error::UnhandledError` if an unhandled error occurs.
pub async fn get_transaction_details(
    config: &Config,
    access_token: &AccessToken,
    index: &str,
) -> Result<GetTransactionDetailsResponse> {
    let base_url = &config.backend_url;
    let url = format!("{base_url}/transactions/details");
    let query = GetTransactionStatusRequest { index: index.into() };
    info!("Get transaction status for {index}");
    info!("Used url: {url:#?}");
    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .bearer_auth(access_token.as_str())
        .header("X-APP-NAME", &config.auth_provider)
        .query(&query)
        .send()
        .await?;
    debug!("Response: {response:#?}");
    match response.status() {
        StatusCode::OK => Ok(response.json::<GetTransactionDetailsResponse>().await?),
        StatusCode::UNAUTHORIZED => Err(ApiError::MissingAccessToken),
        _ => {
            let status = response.status();
            let text = response.text().await?;
            error!(
                "Failed to get transaction `{index}` details: Response status: {}, Response text: {}",
                status, text
            );
            Err(ApiError::UnexpectedResponse {
                code: status,
                body: text,
            })
        }
    }
}

/// Get transaction list (paginated)
///
/// # Arguments
///
/// * `config` - The configuration object.
/// * `access_token` - The access token for authentication.
/// * `username` - The username.
/// * `start` - The start number for the transaction list
/// * `limit` - The limit of transactions per page
///
/// # Returns
///
/// Returns a `Result` containing `GetTransactionDetailsResponse` if successful, or an `Error` if an error occurs.
///
/// # Errors
///
/// Returns an `Error::Unauthorized` if the request is unauthorized, or an `Error::UnhandledError` if an unhandled error occurs.
pub async fn get_transactions_list(
    config: &Config,
    access_token: &AccessToken,
    start: u32,
    limit: u32,
) -> Result<GetTxsDetailsResponse> {
    let base_url = &config.backend_url;
    let url = format!("{base_url}/transactions/txs-details");
    let query = TxsDetailsQuery {
        date: None,
        partner: None,
        is_sender: false,
        start,
        limit,
    };

    info!("Get transaction list");
    info!("Used url: {url:#?}");
    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .bearer_auth(access_token.as_str())
        .header("X-APP-NAME", &config.auth_provider)
        .query(&query)
        .send()
        .await?;
    debug!("Response: {response:#?}");
    let tx_response = match response.status() {
        StatusCode::OK => response.json::<GetTxsDetailsResponse>().await?,
        StatusCode::UNAUTHORIZED => return Err(ApiError::MissingAccessToken),
        _ => {
            let status = response.status();
            let text = response.text().await?;
            error!(
                "Failed to get a list of transactions: Response status: {}, Response text: {}",
                status, text
            );
            return Err(ApiError::UnexpectedResponse {
                code: status,
                body: text,
            });
        }
    };

    Ok(tx_response)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing_utils::{
        example_tx_details, example_tx_metadata, set_config, AMOUNT, AUTH_PROVIDER, HEADER_X_APP_NAME, LIMIT, RECEIVER,
        START, TOKEN, TX_INDEX,
    };
    use mockito::Matcher;

    #[rstest::rstest]
    #[case(201, Ok(CreateTransactionResponse { index: TX_INDEX.into() }))]
    #[case(401, Err(ApiError::MissingAccessToken))]
    #[case(500, Err(ApiError::UnexpectedResponse {
        code: StatusCode::INTERNAL_SERVER_ERROR,
        body: "".to_string() 
    }))]
    #[case(501, Err(ApiError::UnexpectedResponse {
        code: StatusCode::NOT_IMPLEMENTED,
        body: "".to_string() 
    }))]
    #[tokio::test]
    async fn test_create_new_transaction(
        #[case] status_code: usize,
        #[case] expected: Result<CreateTransactionResponse>,
    ) {
        // Arrange
        let (mut srv, config, _cleanup) = set_config().await;

        let mock_request = CreateTransactionRequest {
            amount: AMOUNT.inner(),
            network_key: String::from("IOTA"),
            receiver: RECEIVER.into(),
            application_metadata: example_tx_metadata(),
        };
        let request_body = serde_json::to_string(&mock_request).unwrap();

        let body = CreateTransactionResponse { index: TX_INDEX.into() };
        let mock_body_response = serde_json::to_string(&body).unwrap();

        let mut mock_server = srv
            .mock("POST", "/api/transactions/create")
            .match_header(HEADER_X_APP_NAME, AUTH_PROVIDER)
            .match_header("authorization", format!("Bearer {}", TOKEN.as_str()).as_str())
            .with_status(status_code)
            .match_body(Matcher::Exact(request_body))
            .with_header("content-type", "application/json");
        // Conditionally add the response body only for the 201 status code
        if status_code == 201 {
            mock_server = mock_server.with_body(&mock_body_response);
        }
        let mock_server = mock_server.expect(1).create();

        // Act
        let response = create_new_transaction(
            &config,
            &TOKEN,
            RECEIVER,
            String::from("IOTA"),
            AMOUNT,
            example_tx_metadata(),
        )
        .await;

        // Assert
        match expected {
            Ok(resp) => {
                assert_eq!(response.unwrap(), resp);
            }
            Err(ref expected_err) => {
                assert_eq!(response.err().unwrap().to_string(), expected_err.to_string());
            }
        }
        mock_server.assert();
    }

    #[rstest::rstest]
    #[case(202, Ok(()))]
    #[case(401, Err(ApiError::MissingAccessToken))]
    #[case(500, Err(ApiError::UnexpectedResponse {
        code: StatusCode::INTERNAL_SERVER_ERROR,
        body: "".to_string() 
    }))]
    #[case(501, Err(ApiError::UnexpectedResponse {
        code: StatusCode::NOT_IMPLEMENTED,
        body: "".to_string() 
    }))]
    #[tokio::test]
    async fn test_commit_transaction(#[case] status_code: usize, #[case] expected: Result<()>) {
        // Arrange
        let (mut srv, config, _cleanup) = set_config().await;

        let mock_request = CommitTransactionRequest {
            index: TX_INDEX.into(),
            transaction_id: TX_INDEX.into(),
        };
        let request_body = serde_json::to_string(&mock_request).unwrap();

        let mock_server = srv
            .mock("POST", "/api/transactions/commit")
            .match_header(HEADER_X_APP_NAME, AUTH_PROVIDER)
            .match_header("authorization", format!("Bearer {}", TOKEN.as_str()).as_str())
            .match_body(Matcher::Exact(request_body))
            .with_status(status_code)
            .expect(1)
            .create();

        // Act
        let response = commit_transaction(&config, &TOKEN, TX_INDEX, TX_INDEX).await;

        // Assert
        match expected {
            Ok(_) => response.unwrap(),
            Err(ref expected_err) => {
                assert_eq!(response.err().unwrap().to_string(), expected_err.to_string());
            }
        }
        mock_server.assert();
    }

    #[rstest::rstest]
    #[case(200, Ok(example_tx_details()))]
    #[case(401, Err(ApiError::MissingAccessToken))]
    #[case(500, Err(ApiError::UnexpectedResponse {
        code: StatusCode::INTERNAL_SERVER_ERROR,
        body: "".to_string() 
    }))]
    #[case(501, Err(ApiError::UnexpectedResponse {
        code: StatusCode::NOT_IMPLEMENTED,
        body: "".to_string() 
    }))]
    #[tokio::test]
    async fn test_get_transaction_details(
        #[case] status_code: usize,
        #[case] expected: Result<GetTransactionDetailsResponse>,
    ) {
        // Arrange
        let (mut srv, config, _cleanup) = set_config().await;

        let mock_body_response = serde_json::to_string(&example_tx_details()).unwrap();

        let mut mock_server = srv
            .mock("GET", "/api/transactions/details")
            .match_header(HEADER_X_APP_NAME, AUTH_PROVIDER)
            .match_header("authorization", format!("Bearer {}", TOKEN.as_str()).as_str())
            .with_status(status_code)
            .match_query(Matcher::Exact(format!("index={TX_INDEX}")))
            .with_header("content-type", "application/json");
        // Conditionally add the response body only for the 200 status code
        if status_code == 200 {
            mock_server = mock_server.with_body(&mock_body_response);
        }
        let mock_server = mock_server.expect(1).create();

        // Act
        let response = get_transaction_details(&config, &TOKEN, TX_INDEX).await;

        // Assert
        match expected {
            Ok(resp) => {
                assert_eq!(response.unwrap(), resp);
            }
            Err(ref expected_err) => {
                assert_eq!(response.err().unwrap().to_string(), expected_err.to_string());
            }
        }
        mock_server.assert();
    }

    #[rstest::rstest]
    #[case(200, Ok(GetTxsDetailsResponse { txs: vec![] }))]
    #[case(401, Err(ApiError::MissingAccessToken))]
    #[case(500, Err(ApiError::UnexpectedResponse {
        code: StatusCode::INTERNAL_SERVER_ERROR,
        body: "".to_string() 
    }))]
    #[case(501, Err(ApiError::UnexpectedResponse {
        code: StatusCode::NOT_IMPLEMENTED,
        body: "".to_string() 
    }))]
    #[tokio::test]
    async fn test_get_transaction_list(#[case] status_code: usize, #[case] expected: Result<GetTxsDetailsResponse>) {
        // Arrange
        let (mut srv, config, _cleanup) = set_config().await;

        let mock_response = GetTxsDetailsResponse { txs: vec![] };
        let mock_body_response = serde_json::to_string(&mock_response).unwrap();

        let mut mock_server = srv
            .mock("GET", "/api/transactions/txs-details")
            .match_header(HEADER_X_APP_NAME, AUTH_PROVIDER)
            .match_header("authorization", format!("Bearer {}", TOKEN.as_str()).as_str())
            .with_status(status_code)
            .match_query(Matcher::Exact(format!("is_sender=false&start={START}&limit={LIMIT}")))
            .with_header("content-type", "application/json");
        // Conditionally add the response body only for the 200 status code
        if status_code == 200 {
            mock_server = mock_server.with_body(&mock_body_response);
        }
        let mock_server = mock_server.expect(1).create();

        // Act
        let response = get_transactions_list(&config, &TOKEN, START, LIMIT).await;

        // Assert
        match expected {
            Ok(resp) => {
                assert_eq!(response.unwrap(), resp);
            }
            Err(ref expected_err) => {
                assert_eq!(response.err().unwrap().to_string(), expected_err.to_string());
            }
        }
        mock_server.assert();
    }
}
