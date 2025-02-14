//! Viviswap
//!
//! This module provides functions for interacting with the Viviswap backend API.
//! It includes functions for creating a new Viviswap user, getting KYC status,
//! setting KYC general and personal details, deleting and creating payment details,
//! and getting payment details.
//!

use super::error::{ApiError, Result};
use crate::core::Config;
use crate::types::currencies::{CryptoAmount, Currency};
use crate::types::newtypes::AccessToken;
use api_types::api::viviswap::contract::{ContractRequestBody, ViviswapContractCreationResponse};
use api_types::api::viviswap::course::{GetCourseRequestQueries, GetCourseResponse};
use api_types::api::viviswap::detail::{
    DeleteDetailRequestQueries, GetPaymentDetailsRequestQueries, GetPaymentDetailsResponse, SetDetailRequestBody,
    SetDetailRequestQueries, SetPaymentDetailResponse, SwapPaymentDetailKey,
};
use api_types::api::viviswap::kyc::{
    AnswerData, GetKycAmlaQuestionsResponse, GetKycDocumentsResponse, IdentityOfficialDocumentData,
    IdentityPersonalDocumentData, KycDetailsResponse, SetDocumentDataRequest, SetGeneralDataRequest,
    SetIdentityDataRequest, SetPersonalDataRequest, SetResidenceDataRequest,
};
use api_types::api::viviswap::order::{GetOrderQuery, GetOrdersQuery, Order, OrderList};
use api_types::api::viviswap::payment::ViviPaymentMethodsResponse;
use api_types::api::viviswap::user::{UserDataRequest, UserDataResponse};
use log::{debug, error, info};
use reqwest::{Method, StatusCode};
use rust_decimal::Decimal;
use std::future::Future;

/// helper object for calling the backend (reduces boiler-plate code)
struct ViviswapBackendCall {
    request_builder: reqwest::RequestBuilder,
    method: reqwest::Method,
    url: String,
}

impl ViviswapBackendCall {
    pub fn new(
        config: &Config,
        access_token: &AccessToken,
        username: &str,
        method: reqwest::Method,
        url_path: &str,
    ) -> Self {
        let url = format!("{}{url_path}", config.backend_url);
        info!("Used url: {url:#?}");

        let client = reqwest::Client::new();
        let request_builder = client
            .request(method.clone(), &url)
            .bearer_auth(access_token.as_str())
            .header("X-APP-NAME", &config.auth_provider)
            .header("X-APP-USERNAME", username);

        Self {
            request_builder,
            method,
            url: url_path.to_string(),
        }
    }

    pub fn with_query<T: serde::Serialize>(self, query: &T) -> Self {
        Self {
            request_builder: self.request_builder.query(query),
            method: self.method,
            url: self.url,
        }
    }

    pub fn with_body<T: serde::Serialize>(self, body: &T) -> Self {
        Self {
            request_builder: self.request_builder.json(body),
            method: self.method,
            url: self.url,
        }
    }

    async fn execute_inner<R, F, Fut>(self, ok_transform: F) -> Result<R>
    where
        F: FnOnce(reqwest::Response) -> Fut,
        Fut: Future<Output = Result<R>>,
    {
        let response = self.request_builder.send().await?;
        debug!("Response: {response:#?}");

        match response.status() {
            StatusCode::OK => ok_transform(response).await,
            StatusCode::UNAUTHORIZED => Err(ApiError::MissingAccessToken),
            _ => {
                let status = response.status();
                let text = response.text().await?;
                let method = self.method.as_str();
                let url = self.url;
                error!(
                    "Failed to {method} {}: Response status: {}, Response text: {}",
                    url, status, text
                );
                Err(ApiError::UnexpectedResponse {
                    code: status,
                    body: text,
                })
            }
        }
    }
    /// Execute this request and parse the response as JSON
    pub async fn execute_parse<R: for<'de> serde::Deserialize<'de>>(self) -> Result<R> {
        // since we cannot pass async closures, lets just define the function here
        async fn parse<R: for<'de> serde::Deserialize<'de>>(response: reqwest::Response) -> Result<R> {
            Ok(response.json::<R>().await?)
        }

        self.execute_inner(parse).await
    }

    /// Execute this request and ignore the return content
    pub async fn execute(self) -> Result<()> {
        // since we cannot pass async closures, lets just define the function here
        async fn ok(_response: reqwest::Response) -> Result<()> {
            Ok(())
        }

        self.execute_inner(ok).await
    }
}

/// Creates a new viviswap user
///
/// # Arguments
///
/// * `config` - The configuration object.
/// * `access_token` - The access token for authentication.
/// * `username` - The username of the user.
/// * `mail` - The email of the user.
/// * `terms_accepted` - A boolean indicating whether the terms have been accepted.
///
/// # Returns
///
/// Returns a `Result` containing the `UserDataResponse` if successful, or an `Error` if an error occurs.
///
/// # Errors
///
/// This function can return an `Error` if the request fails or if the response status is unauthorized.
pub async fn create_viviswap_user(
    config: &Config,
    access_token: &AccessToken,
    username: &str,
    mail: &str,
    terms_accepted: bool,
) -> Result<UserDataResponse> {
    info!("Create new viviswap user for {username}");

    let request = UserDataRequest {
        mail: mail.to_string(),
        terms_accepted,
    };

    ViviswapBackendCall::new(config, access_token, username, Method::POST, "/viviswap/users")
        .with_body(&request)
        .execute_parse()
        .await
}

/// get viviswap kyc status
///
/// # Arguments
///
/// * `config` - The configuration object.
/// * `access_token` - The access token for authentication.
/// * `username` - The username of the user.
///
/// # Returns
///
/// Returns a `Result` containing the `KycDetailsResponse` if successful, or an `Error` if an error occurs.
///
/// # Errors
///
/// This function can return an `Error` if the request fails or if the response status is unauthorized.
pub async fn get_viviswap_kyc_status(
    config: &Config,
    access_token: &AccessToken,
    username: &str,
) -> Result<KycDetailsResponse> {
    info!("Get viviswap kyc status for {username}");

    ViviswapBackendCall::new(config, access_token, username, Method::GET, "/viviswap/kyc/status")
        .execute_parse()
        .await
}

#[allow(clippy::too_many_arguments)]
/// set kyc general details for viviswap
///
/// # Arguments
///
/// * `config` - The configuration object.
/// * `access_token` - The access token for authentication.
/// * `username` - The username of the user.
/// * `is_individual` - A boolean indicating whether the user is an individual.
/// * `is_pep` - A boolean indicating whether the user is a politically exposed person.
/// * `is_us_citizen` - A boolean indicating whether the user is a US citizen.
/// * `is_regulatory_disclosure` - A boolean indicating whether the user has made a regulatory disclosure.
/// * `country_of_residence` - The country of residence of the user.
/// * `nationality` - The nationality of the user.
///
/// # Returns
///
/// Returns a `Result` indicating success or failure.
///
/// # Errors
///
/// This function can return an `Error` if the request fails or if the response status is unauthorized.
pub async fn set_viviswap_kyc_general_details(
    config: &Config,
    access_token: &AccessToken,
    username: &str,
    is_individual: bool,
    is_pep: bool,
    is_us_citizen: bool,
    is_regulatory_disclosure: bool,
    country_of_residence: &str,
    nationality: &str,
) -> Result<()> {
    info!("Set viviswap kyc general details for user {username}");

    let request = SetGeneralDataRequest {
        is_individual,
        is_pep,
        is_us_citizen,
        is_regulatory_disclosure,
        country_of_residence: country_of_residence.into(),
        nationality: nationality.into(),
    };

    ViviswapBackendCall::new(config, access_token, username, Method::POST, "/viviswap/kyc/general")
        .with_body(&request)
        .execute()
        .await
}

/// Set kyc personal details for viviswap
///
/// # Arguments
///
/// * `config` - The configuration object.
/// * `access_token` - The access token for authentication.
/// * `username` - The username of the user.
/// * `full_name` - The full name of the user.
/// * `date_of_birth` - The date of birth of the user.
///
/// # Returns
///
/// Returns a `Result` indicating success or failure.
///
/// # Errors
///
/// This function can return an `Error` if the request fails or if the response status is unauthorized.
pub async fn set_viviswap_kyc_personal_details(
    config: &Config,
    access_token: &AccessToken,
    username: &str,
    full_name: &str,
    date_of_birth: &str,
) -> Result<()> {
    info!("Set viviswap kyc personal details for user {username}");

    let request = SetPersonalDataRequest {
        full_name: full_name.into(),
        date_of_birth: date_of_birth.into(),
    };

    ViviswapBackendCall::new(config, access_token, username, Method::POST, "/viviswap/kyc/personal")
        .with_body(&request)
        .execute()
        .await
}

/// Set kyc identity details for viviswap
///
/// # Arguments
///
/// * `config` - The configuration object.
/// * `access_token` - The access token for authentication.
/// * `username` - The username of the user.
/// * `official_document` - An optional official document data structure.
/// * `personal_documen` - An optional personal document data structure.
///
/// # Returns
///
/// Returns a `Result` indicating success or failure.
///
/// # Errors
///
/// This function can return an `Error` if the request fails or if the response status is unauthorized.
pub async fn set_viviswap_kyc_identity_details(
    config: &Config,
    access_token: &AccessToken,
    username: &str,
    official_document: IdentityOfficialDocumentData,
    personal_document: IdentityPersonalDocumentData,
) -> Result<()> {
    info!("Set viviswap kyc identity details for user {username}");

    let request = SetIdentityDataRequest {
        official_document,
        personal_document,
    };

    ViviswapBackendCall::new(config, access_token, username, Method::POST, "/viviswap/kyc/identity")
        .with_body(&request)
        .execute()
        .await
}

/// Set kyc residence details for viviswap
///
///
/// # Arguments
///
/// * `config` - The configuration object.
/// * `access_token` - The access token for authentication.
/// * `username` - The username of the user.
/// * `residence_details` - The residence details to set
///
/// # Returns
///
/// Returns a `Result` indicating success or failure.
///
/// # Errors
///
/// This function can return an `Error` if the request fails or if the response status is unauthorized.
pub async fn set_viviswap_kyc_residence_details(
    config: &Config,
    access_token: &AccessToken,
    username: &str,
    residence_details: SetResidenceDataRequest,
) -> Result<()> {
    info!("Set viviswap kyc residence details for user {username}");

    ViviswapBackendCall::new(config, access_token, username, Method::POST, "/viviswap/kyc/residence")
        .with_body(&residence_details)
        .execute()
        .await
}

/// Get kyc amla open questions for viviswap
///
///
/// # Arguments
///
/// * `config` - The configuration object.
/// * `access_token` - The access token for authentication.
/// * `username` - The username of the user.
///
/// # Returns
///
/// * the open questions to answer
///
/// Returns a `Result` indicating success or failure.
///
/// # Errors
///
/// This function can return an `Error` if the request fails or if the response status is unauthorized.
pub async fn get_viviswap_kyc_amla_open_questions(
    config: &Config,
    access_token: &AccessToken,
    username: &str,
) -> Result<GetKycAmlaQuestionsResponse> {
    info!("Set viviswap kyc open amla questions for user {username}");

    ViviswapBackendCall::new(config, access_token, username, Method::GET, "/viviswap/kyc/questions")
        .execute_parse()
        .await
}

/// Set answer to kyc amla question for viviswap
///
///
/// # Arguments
///
/// * `config` - The configuration object.
/// * `access_token` - The access token for authentication.
/// * `username` - The username of the user.
/// * `answer` - The answer to set.
///
/// # Returns
///
/// Returns a `Result` indicating success or failure.
///
/// # Errors
///
/// This function can return an `Error` if the request fails or if the response status is unauthorized.
pub async fn set_viviswap_kyc_amla_answer(
    config: &Config,
    access_token: &AccessToken,
    username: &str,
    answer: AnswerData,
) -> Result<()> {
    info!("Set viviswap kyc amla question answer for user {username}");

    ViviswapBackendCall::new(config, access_token, username, Method::POST, "/viviswap/kyc/questions")
        .with_body(&answer)
        .execute()
        .await
}

/// Get kyc open documents for viviswap
///
///
/// # Arguments
///
/// * `config` - The configuration object.
/// * `access_token` - The access token for authentication.
/// * `username` - The username of the user.
///
/// # Returns
///
/// * the open documents that need to be provided.
///
/// Returns a `Result` indicating success or failure.
///
/// # Errors
///
/// This function can return an `Error` if the request fails or if the response status is unauthorized.
pub async fn get_viviswap_kyc_open_documents(
    config: &Config,
    access_token: &AccessToken,
    username: &str,
) -> Result<GetKycDocumentsResponse> {
    info!("Set viviswap kyc open documents for user {username}");

    ViviswapBackendCall::new(config, access_token, username, Method::GET, "/viviswap/kyc/documents")
        .execute_parse()
        .await
}

/// Set / upload kyc document for viviswap
///
///
/// # Arguments
///
/// * `config` - The configuration object.
/// * `access_token` - The access token for authentication.
/// * `username` - The username of the user.
/// * `document` - The document to set/upload.
///
/// # Returns
///
/// Returns a `Result` indicating success or failure.
///
/// # Errors
///
/// This function can return an `Error` if the request fails or if the response status is unauthorized.
pub async fn set_viviswap_kyc_document(
    config: &Config,
    access_token: &AccessToken,
    username: &str,
    document: SetDocumentDataRequest,
) -> Result<()> {
    info!("Set viviswap kyc document for user {username}");

    ViviswapBackendCall::new(config, access_token, username, Method::POST, "/viviswap/kyc/documents")
        .with_body(&document)
        .execute()
        .await
}

/// delete viviswap detail
///
/// # Arguments
///
/// * `config` - The configuration object.
/// * `access_token` - The access token for authentication.
/// * `username` - The username of the user.
/// * `payment_method_key` - The payment method key.
/// * `payment_detail_id` - The payment detail ID.
///
/// # Returns
///
/// Returns a `Result` indicating success or failure.
///
/// # Errors
///
/// This function can return an `Error` if the request fails or if the response status is unauthorized.
pub async fn delete_viviswap_detail(
    config: &Config,
    access_token: &AccessToken,
    username: &str,
    payment_method_key: SwapPaymentDetailKey,
    payment_detail_id: &str,
) -> Result<()> {
    info!("Delete viviswap detail for user:{username} payment-method:{payment_method_key:?} and detail_id:{payment_detail_id}");

    let query = DeleteDetailRequestQueries {
        payment_method_key,
        payment_detail_id: payment_detail_id.to_string(),
    };

    ViviswapBackendCall::new(config, access_token, username, Method::DELETE, "/viviswap/details")
        .with_query(&query)
        .execute()
        .await
}

/// create detail for viviswap
///
/// # Arguments
///
/// * `config` - The configuration object.
/// * `access_token` - The access token for authentication.
/// * `username` - The username of the user.
/// * `payment_method_key` - The payment method key.
/// * `address` - The address for the detail.
///
/// # Returns
///
/// Returns a `Result` indicating success or failure.
///
/// # Errors
///
/// This function can return an `Error` if the request fails or if the response status is unauthorized.
pub async fn set_viviswap_detail(
    config: &Config,
    access_token: &AccessToken,
    username: &str,
    payment_method_key: SwapPaymentDetailKey,
    address: &str,
) -> Result<SetPaymentDetailResponse> {
    info!("Set viviswap detail for user {username} and payment-method {payment_method_key:?}");

    let query = SetDetailRequestQueries { payment_method_key };

    let request = SetDetailRequestBody {
        address: String::from(address),
    };
    ViviswapBackendCall::new(config, access_token, username, Method::POST, "/viviswap/details")
        .with_query(&query)
        .with_body(&request)
        .execute_parse()
        .await
}

/// get details for viviswap
///
/// # Arguments
///
/// * `config` - The configuration object.
/// * `access_token` - The access token for authentication.
/// * `username` - The username of the user.
/// * `payment_method_key` - The payment method key.
///
/// # Returns
///
/// Returns a `Result` containing the `GetPaymentDetailsResponse` if successful.
///
/// # Errors
///
/// This function can return an `Error` if the request fails or if the response status is unauthorized.
pub async fn get_viviswap_details(
    config: &Config,
    access_token: &AccessToken,
    username: &str,
    payment_method_key: SwapPaymentDetailKey,
) -> Result<GetPaymentDetailsResponse> {
    info!("Get viviswap details for user {username} and payment-method {payment_method_key:?}");

    let query = GetPaymentDetailsRequestQueries { payment_method_key };

    ViviswapBackendCall::new(config, access_token, username, Method::GET, "/viviswap/details")
        .with_query(&query)
        .execute_parse()
        .await
}

/// Set viviswap contract.
///
/// # Arguments
///
/// * `config` - The configuration object.
/// * `access_token` - The access token for authentication.
/// * `username` - The username of the user.
/// * `amount` - The amount of the contract.
/// * `incoming_payment_method_id` - The ID of the incoming payment method.
/// * `incoming_payment_detail_id` - The ID of the incoming payment detail (optional).
/// * `outgoing_payment_method_id` - The ID of the outgoing payment method.
/// * `outgoing_payment_detail_id` - The ID of the outgoing payment detail.
///
/// # Returns
///
/// Returns a `Result` containing the `ViviswapContractCreationResponse` if successful.
///
/// # Errors
///
/// This function can return an `Error` if the request fails or if the response status is unauthorized.
#[allow(clippy::too_many_arguments)]
pub async fn set_viviswap_contract(
    config: &Config,
    access_token: &AccessToken,
    username: &str,
    amount: CryptoAmount,
    incoming_payment_method_id: String,
    incoming_payment_detail_id: Option<String>,
    outgoing_payment_method_id: String,
    outgoing_payment_detail_id: String,
) -> Result<ViviswapContractCreationResponse> {
    info!("Set viviswap contract for user {username}");

    let request = ContractRequestBody {
        amount: Option::Some(amount.inner()),
        incoming_payment_method_id,
        incoming_payment_detail_id,
        outgoing_payment_method_id,
        outgoing_payment_detail_id,
    };

    ViviswapBackendCall::new(config, access_token, username, Method::POST, "/viviswap/contracts")
        .with_body(&request)
        .execute_parse()
        .await
}

/// Get viviswap exchange rate.
///
/// # Arguments
///
/// * `config` - The configuration object.
/// * `access_token` - The access token for authentication.
/// * `username` - The username of the user.
///
/// # Returns
///
/// Returns a `Result` containing the exchange rate as `f32` if successful.
///
/// # Errors
///
/// This function can return an `Error` if the request fails or if the response status is unauthorized.
pub async fn get_viviswap_exchange_rate(
    config: &Config,
    access_token: &AccessToken,
    username: &str,
    currency: Currency,
) -> Result<Decimal> {
    info!("get_viviswap_exchange_rate for currency = {:?}", currency);

    let query = GetCourseRequestQueries {
        currency: currency.into(),
    };

    let response: GetCourseResponse =
        ViviswapBackendCall::new(config, access_token, username, Method::GET, "/viviswap/courses")
            .with_query(&query)
            .execute_parse()
            .await?;

    Ok(response.course.course)
}

/// Get viviswap payment methods.
///
/// # Arguments
///
/// * `config` - The configuration object.
/// * `access_token` - The access token for authentication.
/// * `username` - The username of the user.
///
/// # Returns
///
/// Returns a `Result` containing the `ViviPaymentMethodsResponse` if successful.
///
/// # Errors
///
/// This function can return an `Error` if the request fails or if the response status is unauthorized.
pub async fn get_viviswap_payment_method(
    config: &Config,
    access_token: &AccessToken,
    username: &str,
) -> Result<ViviPaymentMethodsResponse> {
    info!("get_viviswap_payment_method");

    ViviswapBackendCall::new(config, access_token, username, Method::GET, "/viviswap/methods")
        .execute_parse()
        .await
}

/// gets the details of a viviswap order.
///
/// # Arguments
///
/// * `config` - The configuration object.
/// * `access_token` - The access token for authentication.
/// * `username` - The username of the user.
/// * `order_id` - The ID of the order.
///
/// # Returns
///
/// Returns a `Result` containing the `Order` if successful.
///
/// # Errors
///
/// This function can return an `Error` if the request fails or if the response status is unauthorized.
pub async fn get_viviswap_order(
    config: &Config,
    access_token: &AccessToken,
    username: &str,
    order_id: &str,
) -> Result<Order> {
    info!("Get viviswap order details for user {username}");

    let query = GetOrderQuery {
        id: order_id.to_string(),
    };

    ViviswapBackendCall::new(config, access_token, username, Method::GET, "/viviswap/orders")
        .with_query(&query)
        .execute_parse()
        .await
}

/// gets all viviswap orders of a user.
///
/// # Arguments
///
/// * `config` - The configuration object.
/// * `access_token` - The access token for authentication.
/// * `username` - The username of the user.
/// * `start` - The starting index of the orders.
/// * `limit` - The maximum number of orders to retrieve.
///
/// # Returns
///
/// Returns a `Result` containing the `OrderList` if successful.
///
/// # Errors
///
/// This function can return an `Error` if the request fails or if the response status is unauthorized.
pub async fn get_viviswap_orders(
    config: &Config,
    access_token: &AccessToken,
    username: &str,
    start: u32,
    limit: u32,
) -> Result<OrderList> {
    let base_url = &config.backend_url;
    let url = format!("{base_url}/viviswap/orders");
    info!("Used url: {url:#?}");
    info!("Get viviswap list of order details for user {username}");

    let query = GetOrdersQuery { start, limit };

    ViviswapBackendCall::new(config, access_token, username, Method::GET, "/viviswap/orders")
        .with_query(&query)
        .execute_parse()
        .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing_utils::{
        example_bank_details, example_contract_response, example_exchange_rate_response,
        example_get_payment_details_response, example_viviswap_oder_response, set_config, ADDRESS, AUTH_PROVIDER,
        HEADER_X_APP_NAME, HEADER_X_APP_USERNAME, PAYMENT_DETAIL_ID, PAYMENT_METHOD_ID, PAYMENT_METHOD_KEY,
        PAYMENT_METHOD_KEY_SERIALIZED, TOKEN, USERNAME,
    };
    use api_types::api::viviswap::{
        detail::PaymentDetail,
        kyc::{File, KycAmlaQuestion, KycOpenDocument, KycStep, KycVerificationStatus, OfficialDocumentType},
    };
    use mockito::Matcher;

    fn example_kyc_status_response() -> KycDetailsResponse {
        KycDetailsResponse {
            is_individual: true,
            is_verified: true,
            full_name: "Satoshi Satoshi".into(),
            submission_step: KycStep::Identity,
            verification_status: KycVerificationStatus::PartiallyVerified,
            verified_step: KycStep::Personal,
            monthly_limit_eur: 20_000.0,
        }
    }

    fn example_set_payment_details_response() -> SetPaymentDetailResponse {
        SetPaymentDetailResponse {
            payment_detail: Some(PaymentDetail {
                id: PAYMENT_DETAIL_ID.into(),
                address: ADDRESS.into(),
                is_verified: Some(true),
            }),
        }
    }

    fn mock_object_kyc_identity() -> (IdentityOfficialDocumentData, IdentityPersonalDocumentData) {
        (
            IdentityOfficialDocumentData {
                r#type: OfficialDocumentType::Id,
                expiration_date: "".into(),
                document_number: "0".into(),
                front_image: File::from_bytes(&[], "front.png"),
                back_image: None,
            },
            IdentityPersonalDocumentData {
                video: File::from_bytes(&[], "front.png"),
            },
        )
    }

    fn mock_object_kyc_residence() -> SetResidenceDataRequest {
        SetResidenceDataRequest {
            country_code: "DE".to_string(),
            region: "".to_string(),
            zip_code: "".to_string(),
            city: "".to_string(),
            address_line_1: "".to_string(),
            address_line_2: "".to_string(),
            is_public_entry: false,
            public_entry_reference: None,
            has_no_official_document: true,
            document_residence_proof: None,
        }
    }

    fn response_object_kyc_amla() -> GetKycAmlaQuestionsResponse {
        GetKycAmlaQuestionsResponse {
            questions: vec![KycAmlaQuestion {
                id: "".to_string(),
                question: "".to_string(),
                possible_answers: vec![],
                is_free_text: false,
                min_answers: 0,
                max_answers: 10,
            }],
        }
    }

    fn mock_object_kyc_amla_question() -> AnswerData {
        AnswerData {
            question_id: "".to_string(),
            answers: vec!["".to_string(), "".to_string(), "".to_string()],
            freetext_answer: None,
        }
    }

    fn response_object_kyc_documents() -> GetKycDocumentsResponse {
        GetKycDocumentsResponse {
            documents: vec![KycOpenDocument {
                id: "".to_string(),
                is_back_image_required: false,
                r#type: "".to_string(),
                description: "".to_string(),
            }],
        }
    }

    fn mock_object_kyc_document() -> SetDocumentDataRequest {
        SetDocumentDataRequest {
            document_id: "".to_string(),
            expiration_date: "".to_string(),
            document_number: "".to_string(),
            front_image: Some(File::try_from_base64("", "front.png").unwrap()),
            back_image: None,
        }
    }

    #[rstest::rstest]
    #[case(200, Ok(UserDataResponse {
        username: USERNAME.into(),
    }))]
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
    async fn test_create_viviswap_user(#[case] status_code: usize, #[case] expected: Result<UserDataResponse>) {
        // Arrange
        let (mut srv, config, _cleanup) = set_config().await;

        let mock_request = UserDataRequest {
            mail: format!("{USERNAME}@mail.com"),
            terms_accepted: true,
        };
        let request_body = serde_json::to_string(&mock_request).unwrap();

        let mock_response = UserDataResponse {
            username: USERNAME.into(),
        };
        let body = serde_json::to_string(&mock_response).unwrap();

        let mut mock_server = srv
            .mock("POST", "/api/viviswap/users")
            .match_header(HEADER_X_APP_NAME, AUTH_PROVIDER)
            .match_header(HEADER_X_APP_USERNAME, USERNAME)
            .match_header("authorization", format!("Bearer {}", TOKEN.as_str()).as_str())
            .with_status(status_code)
            .match_body(Matcher::Exact(request_body))
            .with_header("content-type", "application/json");
        // Conditionally add the response body only for the 200 status code
        if status_code == 200 {
            mock_server = mock_server.with_body(&body);
        }
        let mock_server = mock_server.expect(1).create();

        // Act
        let response =
            create_viviswap_user(&config, &TOKEN, USERNAME, format!("{USERNAME}@mail.com").as_str(), true).await;

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
    #[case(200, Ok(example_kyc_status_response()))]
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
    async fn test_get_viviswap_kyc_status(#[case] status_code: usize, #[case] expected: Result<KycDetailsResponse>) {
        // Arrange
        let (mut srv, config, _cleanup) = set_config().await;

        let body = serde_json::to_string(&example_kyc_status_response()).unwrap();

        let mut mock_server = srv
            .mock("GET", "/api/viviswap/kyc/status")
            .match_header(HEADER_X_APP_NAME, AUTH_PROVIDER)
            .match_header(HEADER_X_APP_USERNAME, USERNAME)
            .match_header("authorization", format!("Bearer {}", TOKEN.as_str()).as_str())
            .with_status(status_code)
            .with_header("content-type", "application/json");
        // Conditionally add the response body only for the 200 status code
        if status_code == 200 {
            mock_server = mock_server.with_body(&body);
        }
        let mock_server = mock_server.expect(1).create();

        // Act
        let response = get_viviswap_kyc_status(&config, &TOKEN, USERNAME).await;

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
    #[case(200, Ok(()))]
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
    async fn test_set_viviswap_kyc_general_details(#[case] status_code: usize, #[case] expected: Result<()>) {
        // Arrange
        let (mut srv, config, _cleanup) = set_config().await;

        let mock_request = SetGeneralDataRequest {
            is_individual: true,
            is_pep: false,
            is_regulatory_disclosure: true,
            is_us_citizen: false,
            country_of_residence: "DE".to_string(),
            nationality: "DE".to_string(),
        };
        let request_body = serde_json::to_string(&mock_request).unwrap();

        let mock_server = srv
            .mock("POST", "/api/viviswap/kyc/general")
            .match_header(HEADER_X_APP_NAME, AUTH_PROVIDER)
            .match_header(HEADER_X_APP_USERNAME, USERNAME)
            .match_header("authorization", format!("Bearer {}", TOKEN.as_str()).as_str())
            .match_body(Matcher::Exact(request_body))
            .with_status(status_code)
            .expect(1)
            .create();

        // Act
        let response =
            set_viviswap_kyc_general_details(&config, &TOKEN, USERNAME, true, false, false, true, "DE", "DE").await;

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
    #[case(200, Ok(()))]
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
    async fn test_set_viviswap_kyc_personal_details(#[case] status_code: usize, #[case] expected: Result<()>) {
        // Arrange
        let (mut srv, config, _cleanup) = set_config().await;

        let mock_request = SetPersonalDataRequest {
            full_name: "Satoshi Satoshi".into(),
            date_of_birth: "1990-01-01".into(),
        };
        let request_body = serde_json::to_string(&mock_request).unwrap();

        let mock_server = srv
            .mock("POST", "/api/viviswap/kyc/personal")
            .match_header(HEADER_X_APP_NAME, AUTH_PROVIDER)
            .match_header(HEADER_X_APP_USERNAME, USERNAME)
            .match_header("authorization", format!("Bearer {}", TOKEN.as_str()).as_str())
            .match_header("content-type", "application/json")
            .match_body(Matcher::Exact(request_body))
            .with_status(status_code)
            .expect(1)
            .create();

        // Act
        let response =
            set_viviswap_kyc_personal_details(&config, &TOKEN, USERNAME, "Satoshi Satoshi", "1990-01-01").await;

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
    #[case(200, Ok(()))]
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
    async fn test_delete_viviswap_payment_details(#[case] status_code: usize, #[case] expected: Result<()>) {
        // Arrange
        let (mut srv, config, _cleanup) = set_config().await;

        let mock_server = srv
            .mock("DELETE", "/api/viviswap/details")
            .match_header(HEADER_X_APP_NAME, AUTH_PROVIDER)
            .match_header(HEADER_X_APP_USERNAME, USERNAME)
            .match_header("authorization", format!("Bearer {}", TOKEN.as_str()).as_str())
            .match_query(Matcher::Exact(format!(
                "payment_method_key={PAYMENT_METHOD_KEY_SERIALIZED}&payment_detail_id={PAYMENT_DETAIL_ID}"
            )))
            .with_status(status_code)
            .expect(1)
            .create();

        // Act
        let response = delete_viviswap_detail(&config, &TOKEN, USERNAME, PAYMENT_METHOD_KEY, PAYMENT_DETAIL_ID).await;

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
    #[case(200, Ok(example_set_payment_details_response()))]
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
    async fn test_set_viviswap_payment_details(
        #[case] status_code: usize,
        #[case] expected: Result<SetPaymentDetailResponse>,
    ) {
        // Arrange
        let (mut srv, config, _cleanup) = set_config().await;

        let mock_request = SetDetailRequestBody {
            address: ADDRESS.into(),
        };
        let request_body = serde_json::to_string(&mock_request).unwrap();

        let body = example_set_payment_details_response();
        let mock_body_response = serde_json::to_string(&body).unwrap();

        let mut mock_server = srv
            .mock("POST", "/api/viviswap/details")
            .match_header(HEADER_X_APP_NAME, AUTH_PROVIDER)
            .match_header(HEADER_X_APP_USERNAME, USERNAME)
            .match_header("authorization", format!("Bearer {}", TOKEN.as_str()).as_str())
            .match_body(Matcher::Exact(request_body))
            .match_query(Matcher::Exact(format!(
                "payment_method_key={PAYMENT_METHOD_KEY_SERIALIZED}"
            )))
            .with_status(status_code);
        // Conditionally add the response body only for the 200 status code
        if status_code == 200 {
            mock_server = mock_server.with_body(&mock_body_response);
        }
        let mock_server = mock_server.expect(1).create();

        // Act
        let response = set_viviswap_detail(&config, &TOKEN, USERNAME, PAYMENT_METHOD_KEY, ADDRESS).await;

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
    #[case(200, Ok(example_get_payment_details_response()))]
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
    async fn test_get_viviswap_payment_details(
        #[case] status_code: usize,
        #[case] expected: Result<GetPaymentDetailsResponse>,
    ) {
        // Arrange
        let (mut srv, config, _cleanup) = set_config().await;

        let body = serde_json::to_string(&example_get_payment_details_response()).unwrap();

        let mut mock_server = srv
            .mock("GET", "/api/viviswap/details")
            .match_header(HEADER_X_APP_NAME, AUTH_PROVIDER)
            .match_header(HEADER_X_APP_USERNAME, USERNAME)
            .match_header("authorization", format!("Bearer {}", TOKEN.as_str()).as_str())
            .match_query(Matcher::Exact(format!(
                "payment_method_key={PAYMENT_METHOD_KEY_SERIALIZED}"
            )))
            .with_status(status_code);
        // Conditionally add the response body only for the 200 status code
        if status_code == 200 {
            mock_server = mock_server.with_body(&body);
        }
        let mock_server = mock_server.expect(1).create();

        // Act
        let response = get_viviswap_details(&config, &TOKEN, USERNAME, PAYMENT_METHOD_KEY).await;

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
    #[case(200, Ok(example_contract_response(example_bank_details())))]
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
    async fn test_set_viviswap_contract(
        #[case] status_code: usize,
        #[case] expected: Result<ViviswapContractCreationResponse>,
    ) {
        // Arrange

        use rust_decimal_macros::dec;
        let (mut srv, config, _cleanup) = set_config().await;

        let mock_request = ContractRequestBody {
            amount: Some(dec!(15.0)),
            incoming_payment_detail_id: Some(PAYMENT_DETAIL_ID.into()),
            incoming_payment_method_id: PAYMENT_METHOD_ID.into(),
            outgoing_payment_detail_id: PAYMENT_DETAIL_ID.into(),
            outgoing_payment_method_id: PAYMENT_METHOD_ID.into(),
        };
        let request_body = serde_json::to_string(&mock_request).unwrap();

        let mock_response = example_contract_response(example_bank_details());
        let body = serde_json::to_string(&mock_response).unwrap();

        let mut mock_server = srv
            .mock("POST", "/api/viviswap/contracts")
            .match_header(HEADER_X_APP_NAME, AUTH_PROVIDER)
            .match_header(HEADER_X_APP_USERNAME, USERNAME)
            .match_header("authorization", format!("Bearer {}", TOKEN.as_str()).as_str())
            .match_body(Matcher::Exact(request_body))
            .with_status(status_code);
        // Conditionally add the response body only for the 200 status code
        if status_code == 200 {
            mock_server = mock_server.with_body(&body);
        }
        let mock_server = mock_server.expect(1).create();

        // Act
        let response = set_viviswap_contract(
            &config,
            &TOKEN,
            USERNAME,
            dec!(15.0).try_into().unwrap(),
            PAYMENT_METHOD_ID.into(),
            Some(PAYMENT_DETAIL_ID.into()),
            PAYMENT_METHOD_ID.into(),
            PAYMENT_DETAIL_ID.into(),
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
    #[case(200, Ok(example_exchange_rate_response()))]
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
    async fn test_get_exchange_rate(#[case] status_code: usize, #[case] expected: Result<GetCourseResponse>) {
        // Arrange
        let (mut srv, config, _cleanup) = set_config().await;

        let body = serde_json::to_string(&example_exchange_rate_response()).unwrap();

        let mut mock_server = srv
            .mock("GET", "/api/viviswap/courses")
            .match_header(HEADER_X_APP_NAME, AUTH_PROVIDER)
            .match_header(HEADER_X_APP_USERNAME, USERNAME)
            .match_header("authorization", format!("Bearer {}", TOKEN.as_str()).as_str())
            .match_query(Matcher::Exact("currency=Iota".to_string()))
            .with_status(status_code);
        // Conditionally add the response body only for the 200 status code
        if status_code == 200 {
            mock_server = mock_server.with_body(&body);
        }
        let mock_server = mock_server.expect(1).create();

        // Act
        let response = get_viviswap_exchange_rate(&config, &TOKEN, USERNAME, Currency::Iota).await;

        // Assert
        match expected {
            Ok(resp) => {
                assert_eq!(response.unwrap(), resp.course.course);
            }
            Err(ref expected_err) => {
                assert_eq!(response.err().unwrap().to_string(), expected_err.to_string());
            }
        }
        mock_server.assert();
    }

    #[rstest::rstest]
    #[case(200, Ok(ViviPaymentMethodsResponse { methods: vec![] }))]
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
    async fn test_get_payment_methods(
        #[case] status_code: usize,
        #[case] expected: Result<ViviPaymentMethodsResponse>,
    ) {
        // Arrange
        let (mut srv, config, _cleanup) = set_config().await;

        let mock_response = ViviPaymentMethodsResponse { methods: vec![] };
        let body = serde_json::to_string(&mock_response).unwrap();

        let mut mock_server = srv
            .mock("GET", "/api/viviswap/methods")
            .match_header(HEADER_X_APP_NAME, AUTH_PROVIDER)
            .match_header(HEADER_X_APP_USERNAME, USERNAME)
            .match_header("authorization", format!("Bearer {}", TOKEN.as_str()).as_str())
            .with_status(status_code);
        // Conditionally add the response body only for the 200 status code
        if status_code == 200 {
            mock_server = mock_server.with_body(&body);
        }
        let mock_server = mock_server.expect(1).create();

        // Act
        let response = get_viviswap_payment_method(&config, &TOKEN, USERNAME).await;

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
    #[case(200, Ok(example_viviswap_oder_response()))]
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
    async fn test_get_viviswap_order(#[case] status_code: usize, #[case] expected: Result<Order>) {
        // Arrange
        let (mut srv, config, _cleanup) = set_config().await;

        let body = serde_json::to_string(&example_viviswap_oder_response()).unwrap();

        let mut mock_server = srv
            .mock("GET", "/api/viviswap/orders")
            .match_header(HEADER_X_APP_NAME, AUTH_PROVIDER)
            .match_header(HEADER_X_APP_USERNAME, USERNAME)
            .match_query(Matcher::Exact("id=order-id".into()))
            .match_header("authorization", format!("Bearer {}", TOKEN.as_str()).as_str())
            .with_status(status_code);
        // Conditionally add the response body only for the 200 status code
        if status_code == 200 {
            mock_server = mock_server.with_body(&body);
        }
        let mock_server = mock_server.expect(1).create();

        // Act
        let response = get_viviswap_order(&config, &TOKEN, USERNAME, "order-id").await;

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
    #[case(200, Ok(OrderList { orders: vec![] }))]
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
    async fn test_get_viviswap_orders(#[case] status_code: usize, #[case] expected: Result<OrderList>) {
        // Arrange
        let (mut srv, config, _cleanup) = set_config().await;

        let mock_response = OrderList { orders: vec![] };
        let body = serde_json::to_string(&mock_response).unwrap();

        let mut mock_server = srv
            .mock("GET", "/api/viviswap/orders")
            .match_header(HEADER_X_APP_NAME, AUTH_PROVIDER)
            .match_header(HEADER_X_APP_USERNAME, USERNAME)
            .match_query(Matcher::Exact("start=1&limit=1".into()))
            .match_header("authorization", format!("Bearer {}", TOKEN.as_str()).as_str())
            .with_status(status_code);
        // Conditionally add the response body only for the 200 status code
        if status_code == 200 {
            mock_server = mock_server.with_body(&body);
        }
        let mock_server = mock_server.expect(1).create();

        // Act
        let response = get_viviswap_orders(&config, &TOKEN, USERNAME, 1u32, 1u32).await;

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
    #[case(200, Ok(()))]
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
    async fn test_set_viviswap_kyc_identity_details(#[case] status_code: usize, #[case] expected: Result<()>) {
        // Arrange
        let (mut srv, config, _cleanup) = set_config().await;

        let (official_document, personal_document) = mock_object_kyc_identity();
        let mock_request = SetIdentityDataRequest {
            official_document: official_document.clone(),
            personal_document: personal_document.clone(),
        };
        let request_body = serde_json::to_string(&mock_request).unwrap();

        let mock_server = srv
            .mock("POST", "/api/viviswap/kyc/identity")
            .match_header(HEADER_X_APP_NAME, AUTH_PROVIDER)
            .match_header(HEADER_X_APP_USERNAME, USERNAME)
            .match_header("authorization", format!("Bearer {}", TOKEN.as_str()).as_str())
            .match_header("content-type", "application/json")
            .match_body(Matcher::Exact(request_body))
            .with_status(status_code)
            .expect(1)
            .create();

        // Act
        let response =
            set_viviswap_kyc_identity_details(&config, &TOKEN, USERNAME, official_document, personal_document).await;

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
    #[case(200, Ok(()))]
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
    async fn test_set_viviswap_kyc_residence_details(#[case] status_code: usize, #[case] expected: Result<()>) {
        // Arrange
        let (mut srv, config, _cleanup) = set_config().await;

        let mock_request = mock_object_kyc_residence();
        let request_body = serde_json::to_string(&mock_request).unwrap();

        let mock_server = srv
            .mock("POST", "/api/viviswap/kyc/residence")
            .match_header(HEADER_X_APP_NAME, AUTH_PROVIDER)
            .match_header(HEADER_X_APP_USERNAME, USERNAME)
            .match_header("authorization", format!("Bearer {}", TOKEN.as_str()).as_str())
            .match_header("content-type", "application/json")
            .match_body(Matcher::Exact(request_body))
            .with_status(status_code)
            .expect(1)
            .create();

        // Act
        let response = set_viviswap_kyc_residence_details(&config, &TOKEN, USERNAME, mock_request).await;

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
    #[case(200, Ok(response_object_kyc_amla()))]
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
    async fn test_get_kyc_amla_open_questions(
        #[case] status_code: usize,
        #[case] expected: Result<GetKycAmlaQuestionsResponse>,
    ) {
        // Arrange
        let (mut srv, config, _cleanup) = set_config().await;

        let expected_response = response_object_kyc_amla();
        let response_body = serde_json::to_string(&expected_response).unwrap();

        let mut mock_server = srv
            .mock("GET", "/api/viviswap/kyc/questions")
            .match_header(HEADER_X_APP_NAME, AUTH_PROVIDER)
            .match_header(HEADER_X_APP_USERNAME, USERNAME)
            .match_header("authorization", format!("Bearer {}", TOKEN.as_str()).as_str())
            .with_status(status_code);
        // Conditionally add the response body only for the 200 status code
        if status_code == 200 {
            mock_server = mock_server.with_body(&response_body);
        }
        let mock_server = mock_server.expect(1).create();

        // Act
        let response = get_viviswap_kyc_amla_open_questions(&config, &TOKEN, USERNAME).await;

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
    #[case(200, Ok(()))]
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
    async fn test_set_kyc_amla_answer(#[case] status_code: usize, #[case] expected: Result<()>) {
        // Arrange
        let (mut srv, config, _cleanup) = set_config().await;

        let mock_request = mock_object_kyc_amla_question();
        let request_body = serde_json::to_string(&mock_request).unwrap();

        let mock_server = srv
            .mock("POST", "/api/viviswap/kyc/questions")
            .match_header(HEADER_X_APP_NAME, AUTH_PROVIDER)
            .match_header(HEADER_X_APP_USERNAME, USERNAME)
            .match_header("authorization", format!("Bearer {}", TOKEN.as_str()).as_str())
            .match_header("content-type", "application/json")
            .match_body(Matcher::Exact(request_body))
            .with_status(status_code)
            .expect(1)
            .create();

        // Act
        let response = set_viviswap_kyc_amla_answer(&config, &TOKEN, USERNAME, mock_request).await;

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
    #[case(200, Ok(response_object_kyc_documents()))]
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
    async fn test_get_kyc_open_documents(
        #[case] status_code: usize,
        #[case] expected: Result<GetKycDocumentsResponse>,
    ) {
        // Arrange
        let (mut srv, config, _cleanup) = set_config().await;

        let expected_response = response_object_kyc_documents();
        let response_body = serde_json::to_string(&expected_response).unwrap();

        let mut mock_server = srv
            .mock("GET", "/api/viviswap/kyc/documents")
            .match_header(HEADER_X_APP_NAME, AUTH_PROVIDER)
            .match_header(HEADER_X_APP_USERNAME, USERNAME)
            .match_header("authorization", format!("Bearer {}", TOKEN.as_str()).as_str())
            .with_status(status_code);
        // Conditionally add the response body only for the 200 status code
        if status_code == 200 {
            mock_server = mock_server.with_body(&response_body);
        }
        let mock_server = mock_server.expect(1).create();

        // Act
        let response = get_viviswap_kyc_open_documents(&config, &TOKEN, USERNAME).await;

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
    #[case(200, Ok(()))]
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
    async fn test_set_kyc_document(#[case] status_code: usize, #[case] expected: Result<()>) {
        // Arrange
        let (mut srv, config, _cleanup) = set_config().await;

        let mock_request = mock_object_kyc_document();
        let request_body = serde_json::to_string(&mock_request).unwrap();

        let mock_server = srv
            .mock("POST", "/api/viviswap/kyc/documents")
            .match_header(HEADER_X_APP_NAME, AUTH_PROVIDER)
            .match_header(HEADER_X_APP_USERNAME, USERNAME)
            .match_header("authorization", format!("Bearer {}", TOKEN.as_str()).as_str())
            .match_header("content-type", "application/json")
            .match_body(Matcher::Exact(request_body))
            .with_status(status_code)
            .expect(1)
            .create();

        // Act
        let response = set_viviswap_kyc_document(&config, &TOKEN, USERNAME, mock_request).await;

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
