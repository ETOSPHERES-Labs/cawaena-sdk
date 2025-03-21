//! This module provides functionality for managing a user in the backend. It currently only has the delete user function.
//!
//! The `delete_user_account` function is used to delete a user account by sending a DELETE request to the backend API.
//! It requires a `Config` object, an access token, and the username of the user to be deleted.

use super::error::{ApiError, Result};
use crate::{core::Config, types::newtypes::AccessToken};
use api_types::api::dlt::{GetPreferredNetworkResponse, SetPreferredNetworkRequest};
use log::{debug, error, info};
use reqwest::StatusCode;

/// Delete the user
///
/// # Arguments
///
/// * `config` - The `Config` object.
/// * `access_token` - The access token.
/// * `username` - The username of the user to be deleted.
///
/// # Returns
///
/// Returns `Ok(())` if the user account is successfully deleted.
///
/// # Errors
///
/// Returns an `Err` variant of [`ApiError`] if there is an error deleting the user account.
pub async fn delete_user_account(config: &Config, access_token: &AccessToken) -> Result<()> {
    let base_url = &config.backend_url;
    let url = format!("{base_url}/user");
    info!("Used url: {url:#?}");
    info!("Deleting user account");

    let client = reqwest::Client::new();
    let response = client
        .delete(&url)
        .bearer_auth(access_token.as_str())
        .header("X-APP-NAME", &config.auth_provider)
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
                "Failed to delete user account: Response status: {}, Response text: {}",
                status, text
            );
            Err(ApiError::UnexpectedResponse {
                code: status,
                body: text,
            })
        }
    }
}

/// Set user's preferred currency
///
/// # Arguments
///
/// * `config` - The `Config` object.
/// * `access_token` - The access token.
/// * `username` - The username of the user.
/// * `currency` - The currency to set as the preferred currency, or None if it should be cleared.
///
/// # Returns
///
/// Returns `Ok(())` if the user's preferred currency was successfully updated.
///
/// # Errors
///
/// Returns an `Err` variant of [`ApiError`] if there is an error updaing the preferred currency.
pub async fn set_preferred_network(
    config: &Config,
    access_token: &AccessToken,
    network_key: Option<String>,
) -> Result<()> {
    let base_url = &config.backend_url;
    let url = format!("{base_url}/user/network");
    info!("Used url: {url:#?}");
    info!("Setting preferred network");

    let body = SetPreferredNetworkRequest { network_key };

    let client = reqwest::Client::new();
    let response = client
        .put(&url)
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
                "Failed to set preferred currency: Response status: {}, Response text: {}",
                status, text
            );
            Err(ApiError::UnexpectedResponse {
                code: status,
                body: text,
            })
        }
    }
}

/// Get user's preferred network (id)
///
/// # Arguments
///
/// * `config` - The `Config` object.
/// * `access_token` - The access token.
/// * `username` - The username of the user.
///
/// # Returns
///
/// Returns the user's preferred network (id), or None if it is not set.
///
/// # Errors
///
/// Returns an `Err` variant of [`ApiError`] if there is an error getting the preferred network (id).
pub async fn get_preferred_network(config: &Config, access_token: &AccessToken) -> Result<Option<String>> {
    let base_url = &config.backend_url;
    let url = format!("{base_url}/user/network");
    info!("Used url: {url:#?}");
    info!("Getting preferred network");

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .bearer_auth(access_token.as_str())
        .header("X-APP-NAME", &config.auth_provider)
        .send()
        .await?;
    debug!("Response: {response:#?}");

    match response.status() {
        StatusCode::OK => Ok(response.json::<GetPreferredNetworkResponse>().await?.network_key),
        StatusCode::UNAUTHORIZED => Err(ApiError::MissingAccessToken),
        _ => {
            let status = response.status();
            let text = response.text().await?;
            error!(
                "Failed to get preferred currency: Response status: {}, Response text: {}",
                status, text
            );
            Err(ApiError::UnexpectedResponse {
                code: status,
                body: text,
            })
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing_utils::{set_config, AUTH_PROVIDER, HEADER_X_APP_NAME, TOKEN};

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
    async fn test_delete_user(#[case] status_code: usize, #[case] expected: Result<()>) {
        // Arrange
        let (mut srv, config, _cleanup) = set_config().await;

        let mock_server = srv
            .mock("DELETE", "/api/user")
            .match_header(HEADER_X_APP_NAME, AUTH_PROVIDER)
            .match_header("authorization", format!("Bearer {}", TOKEN.as_str()).as_str())
            .with_status(status_code)
            .expect(1)
            .create();

        // Act
        let response = delete_user_account(&config, &TOKEN).await;

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
    #[case(200, Ok(Some(String::from("IOTA"))))]
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
    async fn test_get_preferred_network(#[case] status_code: usize, #[case] expected: Result<Option<String>>) {
        // Arrange
        let (mut srv, config, _cleanup) = set_config().await;

        let mut mock_server = srv
            .mock("GET", "/api/user/network")
            .match_header(HEADER_X_APP_NAME, AUTH_PROVIDER)
            .match_header("authorization", format!("Bearer {}", TOKEN.as_str()).as_str())
            .with_status(status_code)
            .with_header("content-type", "application/json");
        // Conditionally add the response body only for the 200 status code
        if status_code == 200 {
            mock_server = mock_server.with_body("{\"network_key\":\"IOTA\"}");
        }
        let mock_server = mock_server.expect(1).create();

        // Act
        let response = get_preferred_network(&config, &TOKEN).await;

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
    async fn test_set_preferred_currency(#[case] status_code: usize, #[case] expected: Result<()>) {
        // Arrange
        let (mut srv, config, _cleanup) = set_config().await;

        let mock_server = srv
            .mock("PUT", "/api/user/network")
            .match_header(HEADER_X_APP_NAME, AUTH_PROVIDER)
            .match_header("authorization", format!("Bearer {}", TOKEN.as_str()).as_str())
            .with_status(status_code)
            .expect(1)
            .create();

        // Act
        let response = set_preferred_network(&config, &TOKEN, Some(String::from("IOTA"))).await;

        // Assert
        match expected {
            Ok(_) => response.unwrap(),
            Err(ref expected_err) => {
                assert_eq!(response.err().unwrap().to_string(), expected_err.to_string());
            }
        }
        mock_server.assert();
    }
}
