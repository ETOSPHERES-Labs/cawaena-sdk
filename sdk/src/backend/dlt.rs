//! Connects to the dlt-service and puts the user address.

use super::error::{ApiError, Result};
use crate::{core::config::Config, types::newtypes::AccessToken};
use api_types::api::{
    dlt::{AddressQueryParameters, ApiGetNetworksResponse, SetUserAddressRequest},
    networks::ApiNetwork,
};
use log::{debug, error, info};
use reqwest::StatusCode;

/// Puts the user crypto currency address in the backend
///
/// # Arguments
/// * `config` - The configuration object.
/// * `username` - The username of the customer.
/// * `access_token` - The access token for authentication.
/// * `address` - The crypto currency address of the user from the wallet.
///
/// # Errors
///
/// Returns an `Err` variant with the following possible values:
/// * [`ApiError::MissingAccessToken`] if the request is unauthorized.
/// * [`ApiError::UnexpectedResponse`] if an unhandled error occurs.
pub async fn put_user_address(
    config: &Config,
    access_token: &AccessToken,
    network_key: String,
    address: &str,
) -> Result<()> {
    let base_url = &config.backend_url;
    let url = format!("{base_url}/user/address");
    let query = AddressQueryParameters {
        network_key: network_key.clone(),
    };

    info!("Used url: {url:#?}");
    let body = SetUserAddressRequest {
        address: address.to_string(),
    };
    info!("Putting user address {address}");

    let client = reqwest::Client::new();
    let response = client
        .put(&url)
        .bearer_auth(access_token.as_str())
        .header("X-APP-NAME", &config.auth_provider)
        .query(&query)
        .json(&body)
        .send()
        .await?;
    debug!("Response: {response:#?}");
    match response.status() {
        StatusCode::CREATED => (),
        StatusCode::UNAUTHORIZED => return Err(ApiError::MissingAccessToken),
        _ => {
            let status = response.status();
            let text = response.text().await?;
            error!(
                "Failed to put user address: Response status: {}, Response text: {}",
                status, text
            );
            return Err(ApiError::UnexpectedResponse {
                code: status,
                body: text,
            });
        }
    };

    Ok(())
}

/// Get networks from backend.
///
/// # Arguments
///
/// * `config` - The configuration object.
/// * `access_token` - The access token for authentication.
/// * `username` - The username of the customer.
///
/// # Returns
///
/// Returns a `Result` containing the networks if successful, or an `Error` if an error occurs.
///
/// # Errors
///
/// Returns an `Err` variant with the following possible values:
/// * [`ApiError::MissingAccessToken`] if the request is unauthorized.
/// * [`ApiError::UnexpectedResponse`] if an unhandled error occurs.
pub async fn get_networks(config: &Config, access_token: &AccessToken) -> Result<Vec<ApiNetwork>> {
    let base_url = &config.backend_url;
    let url = format!("{base_url}/config/networks");

    info!("Used url: {url:#?}");
    info!("Getting networks ..");

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .bearer_auth(access_token.as_str())
        .header("X-APP-NAME", &config.auth_provider)
        .send()
        .await?;
    debug!("Response: {response:#?}");

    match response.status() {
        StatusCode::OK => {
            let networks = response.json::<ApiGetNetworksResponse>().await?.networks;
            Ok(networks)
        }
        StatusCode::UNAUTHORIZED => Err(ApiError::MissingAccessToken),
        _ => {
            let status = response.status();
            let text = response.text().await?;
            error!(
                "Failed to get node urls from backend: Response status: {}, Response text: {:?}",
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
    use crate::testing_utils::{example_api_network, set_config, ADDRESS, AUTH_PROVIDER, HEADER_X_APP_NAME, TOKEN};
    use mockito::Matcher;

    #[rstest::rstest]
    #[case(201, Ok(()))]
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
    async fn test_put_user_address(#[case] status_code: usize, #[case] expected: Result<()>) {
        // Arrange
        let (mut srv, config, _cleanup) = set_config().await;

        let iota_network_key = "IOTA";

        let mock_request = SetUserAddressRequest {
            address: ADDRESS.into(),
        };
        let body = serde_json::to_string(&mock_request).unwrap();

        let mock = srv
            .mock("PUT", "/api/user/address")
            .match_header(HEADER_X_APP_NAME, AUTH_PROVIDER)
            .match_header("authorization", format!("Bearer {}", TOKEN.as_str()).as_str())
            .match_header("content-type", "application/json")
            .match_query(Matcher::Exact(format!("network_key={}", iota_network_key)))
            .match_body(Matcher::Exact(body))
            .with_status(status_code)
            .expect(1)
            .with_header("content-type", "application/json")
            .create();

        // Act
        let response = put_user_address(&config, &TOKEN, String::from("IOTA"), ADDRESS).await;

        // Assert
        match expected {
            Ok(_) => response.unwrap(),
            Err(ref err) => {
                assert_eq!(response.unwrap_err().to_string(), err.to_string());
            }
        }
        mock.assert();
    }

    #[rstest::rstest]
    #[case(200, Ok(ApiGetNetworksResponse {networks: vec![example_api_network(String::from("IOTA")), example_api_network(String::from("ETH"))]}))]
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
    async fn test_get_networks(#[case] status_code: usize, #[case] expected: Result<ApiGetNetworksResponse>) {
        // Arrange
        let (mut srv, config, _cleanup) = set_config().await;

        let resp_body = ApiGetNetworksResponse {
            networks: vec![
                example_api_network(String::from("IOTA")),
                example_api_network(String::from("ETH")),
            ],
        };
        let mock_body_response = serde_json::to_string(&resp_body).unwrap();

        let mut mock_server = srv
            .mock("GET", "/api/config/networks")
            .match_header(HEADER_X_APP_NAME, AUTH_PROVIDER)
            .match_header("authorization", format!("Bearer {}", TOKEN.as_str()).as_str())
            .with_status(status_code)
            .with_header("content-type", "application/json");
        // Conditionally add the response body only for the 200 status code
        if status_code == 200 {
            mock_server = mock_server.with_body(&mock_body_response);
        }
        let mock_server = mock_server.expect(1).create();

        // Act
        let response = get_networks(&config, &TOKEN).await;

        // Assert
        match expected {
            Ok(resp) => {
                assert_eq!(response.unwrap(), resp.networks);
            }
            Err(ref expected_err) => {
                assert_eq!(response.err().unwrap().to_string(), expected_err.to_string());
            }
        }
        mock_server.assert();
    }
}
