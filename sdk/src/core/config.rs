//! Configuration for SDK
//!
//!

use super::Sdk;
use crate::error::{Error, Result};
use crate::user::repository::UserRepoImpl;
use crate::user::UserRepo;
use log::info;
use std::path::Path;
use std::str::FromStr;

/// The default log file used by the sdk to write logs
const CRYPTPAY_LOGFILE: &str = "cryptpay_sdk.log";

/// Struct to configure the SDK
#[derive(Debug)]
pub struct Config {
    /// The root folder used to access the file system. It is assumed that we have full read and
    /// write permissions to this folder and that it already exists.
    pub path_prefix: Box<Path>,

    /// value of the X-APP-NAME header used to select the OAuth provider in the backend.
    pub auth_provider: String,

    /// URL to access the backend.
    pub backend_url: reqwest::Url,

    /// Log level for filtering which log messages that end up in the log file.
    pub log_level: log::LevelFilter,
}

/// Struct representing the  deserialized version of the config in JSON format.
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
struct DeserializedConfig {
    backend_url: String,

    #[serde(default = "default_log_level")]
    log_level: String,

    #[serde(default = "default_storage_path")]
    storage_path: String,

    auth_provider: String,
}

#[cfg(test)]
impl Default for DeserializedConfig {
    /// Create a new [`DeserializedConfig`].
    fn default() -> Self {
        Self {
            backend_url: "http://example.com".to_string(),
            auth_provider: "standalone".to_string(),
            log_level: default_log_level(),
            storage_path: default_storage_path(),
        }
    }
}

fn default_log_level() -> String {
    "INFO".to_string()
}
fn default_storage_path() -> String {
    ".".to_string()
}

/// To be used by bindings to deserialize JSON to the [`DeserializedConfig`] struct.
impl FromStr for DeserializedConfig {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self> {
        serde_json::from_str(s)
            .map_err(|e| crate::Error::SetConfig(format!("Could not deserialize JSON config: {e:#?}")))
    }
}

impl TryFrom<DeserializedConfig> for Config {
    type Error = Error;

    fn try_from(value: DeserializedConfig) -> Result<Self> {
        let path_prefix = Path::new(&value.storage_path);
        #[cfg(not(target_arch = "wasm32"))]
        if !path_prefix.is_dir() {
            return Err(crate::Error::SetConfig(
                "storage_path must be a valid existing directory".to_string(),
            ));
        }

        if value.auth_provider.is_empty() {
            return Err(crate::Error::SetConfig("auth_provider must not be empty".to_string()));
        }

        Ok(Self {
            backend_url: reqwest::Url::parse(&value.backend_url).map_err(|e| crate::Error::SetConfig(e.to_string()))?,
            log_level: log::LevelFilter::from_str(&value.log_level)
                .map_err(|e| crate::Error::SetConfig(format!("Could not parse log level: {e:#?}")))?,
            auth_provider: value.auth_provider,
            path_prefix: path_prefix.into(),
        })
    }
}

impl Config {
    /// Load the [`Config`] directly from a JSON-formatted [`String`] or [`str`].
    pub fn from_json(json: impl AsRef<str>) -> Result<Self> {
        let json = json.as_ref();
        let deserialized_config: DeserializedConfig = json.parse()?;
        Self::try_from(deserialized_config)
    }
}

impl Sdk {
    /// Set the [`Config`] needed by the SDK
    pub fn set_config(&mut self, config: Config) -> Result<()> {
        info!("Setting config: {config:?}");
        // TODO: do any destructing things if config already exists

        // initialize the logger if we are not on wasm
        #[cfg(not(target_arch = "wasm32"))]
        {
            let log_file = config.path_prefix.join(CRYPTPAY_LOGFILE);
            let log_file_str = log_file
                .to_str()
                .ok_or_else(|| crate::Error::SetConfig(format!("config file path is not valid utf-8: {log_file:?}")))?;
            crate::logger::init_logger(config.log_level, log_file_str)?;
        }

        self.config = Some(config);

        // now do any necessary setup steps
        self.initialize_user_repository()?;

        Ok(())
    }

    /// Set path prefix
    fn initialize_user_repository(&mut self) -> Result<()> {
        // initialize jammdb
        #[cfg(feature = "jammdb_repo")]
        let repo: Box<dyn UserRepo + Send + Sync> = {
            let config = self.config.as_ref().ok_or(crate::Error::MissingConfig)?;
            Box::new(UserRepoImpl::new(crate::user::file_storage::FileUserStorage::new(
                &config.path_prefix,
            )?))
        };

        // for wasm: try browser, and fallback to in-memory if it fails!
        #[cfg(target_arch = "wasm32")]
        let repo: Box<dyn UserRepo + Send + Sync> = {
            // try to access to browser local storage
            let browser_storage = crate::user::web_storage::BrowserLocalStorage::new();
            if browser_storage.is_available() {
                Box::new(UserRepoImpl::new(browser_storage))
            } else {
                log::warn!("Browser Local Storage is not available, falling back to in-memory user storage!");
                Box::new(UserRepoImpl::new(crate::user::memory_storage::MemoryUserStorage::new()))
            }
        };

        // if we are not compiling for wasm and the jammdb_repo feature is not active, use an
        // in-memory storage.
        #[cfg(all(not(target_arch = "wasm32"), not(feature = "jammdb_repo")))]
        let repo: Box<dyn UserRepo + Send + Sync> =
            Box::new(UserRepoImpl::new(crate::user::memory_storage::MemoryUserStorage::new()));

        self.repo = Some(repo);
        Ok(())
    }
}

#[cfg(test)]
#[allow(clippy::expect_used)] // this is testing code so expect is fine
impl Config {
    /// Create a new [`Config`] with a [`testing::CleanUp`] as the path_prefix.
    pub fn new_test_with_cleanup() -> (Self, testing::CleanUp) {
        let cleanup = testing::CleanUp::default();
        (
            Self {
                backend_url: reqwest::Url::parse("http://example.com").expect("should be a valid url"),
                path_prefix: Path::new(&cleanup.path_prefix).into(),
                auth_provider: "standalone".to_string(),
                log_level: log::LevelFilter::Debug,
            },
            cleanup,
        )
    }

    /// Create a new [`Config`] with the specified backend url and a [`testing::CleanUp`] as the path_prefix.
    pub fn new_test_with_cleanup_url(url: &str) -> (Self, testing::CleanUp) {
        let cleanup = testing::CleanUp::default();
        (
            Self {
                backend_url: url.parse().expect("should be a valid url"),
                path_prefix: Path::new(&cleanup.path_prefix).into(),
                auth_provider: "standalone".to_string(),
                log_level: log::LevelFilter::Debug,
            },
            cleanup,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    fn valid_deserialized_config() -> DeserializedConfig {
        DeserializedConfig {
            backend_url: "http://example.com".to_string(),
            log_level: "INFO".to_string(),
            storage_path: ".".to_string(),
            auth_provider: "nonempty".to_string(),
        }
    }

    #[test]
    fn test_valid_config() {
        Config::try_from(valid_deserialized_config()).unwrap();
    }

    #[test]
    fn test_path_prefix_error() {
        let mut config = valid_deserialized_config();
        config.storage_path = "nonexistent_file.txt".to_string();

        Config::try_from(config).unwrap_err();
    }

    #[test]
    fn test_empty_auth_provider_error() {
        let mut config = valid_deserialized_config();
        config.auth_provider = "".to_string();

        Config::try_from(config).unwrap_err();
    }

    #[test]
    fn test_invalid_backend_url_error() {
        let mut config = valid_deserialized_config();
        config.backend_url = "invalid url".to_string();

        Config::try_from(config).unwrap_err();
    }

    #[test]
    fn test_default_log_level() {
        let log_level = default_log_level();
        assert_eq!(log_level, "INFO".to_string())
    }

    #[test]
    fn test_default_storage_path() {
        let storage_path = default_storage_path();
        assert_eq!(storage_path, ".".to_string())
    }

    #[rstest]
    #[case(
        r#"{
            "backend_url": "http://example.com",
            "storage_path": ".",
            "log_level": "INFO",
            "auth_provider": "standalone"
          }"#,
        Ok(DeserializedConfig::default())
    )]
    #[case(
        r#"{
            "backend_url": "http://example.com
          }"#,
        Err(crate::Error::SetConfig("Could not deserialize JSON config: ...".to_string())),
    )]
    fn test_deserialized_config_from_str(#[case] input: &str, #[case] expected: Result<DeserializedConfig>) {
        let result = DeserializedConfig::from_str(input);
        match expected {
            Ok(resp) => {
                assert_eq!(result.unwrap(), resp);
            }
            Err(_expected_err) => {
                result.unwrap_err();
            }
        }
    }
}
