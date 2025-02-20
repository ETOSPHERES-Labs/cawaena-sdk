# API Reference

## Levels and Repeats

| **Category** | **Description** |
|--------------|------------------|
| **Levels**   |                  |
| Basic        | The functions at these levels are absolutely necessary to use the SDK. Without calling these functions, the SDK is never in a correct state. Optional functions can be skipped, since they will take default values, if that is the requirement. |
| Usage        | The functions at these levels can only be called once all basic level functions have successfully executed. |
| **Repeats**  |                  |
| Handle       | The functions need to be called every time a new SDK Handle (object) needs to be created via a constructor or after garbage collection of any existing old handles. |
| User         | The functions need to be called every time a new SDK user needs to be created. |
| Application  | The functions can be called any time while using the SDK, however they may fail, if the certain dependencies are not fulfilled. |

## Usage infos and warnings

!!! Warning

    - Viviswap and Postident KYC onboarding will not work if the user is already kyc verified.
    - KYC onboarding with another provider will not work if the user is already started kyc onboarding with one of the other providers.
    - Restoring a wallet backup may fail, if the wallet is already existing.
    - Initializing User and initializing wallet may fail, if the user and wallet are already initialized. Since, there is no de-init function, the  SDK handle needs to be closed, or a new handle needs to be created to re init. 
    - Multiple handles to the wallet may also fail, since only atomic access are allowed.
    - Deleting a user may fail if the backend cannot be reached.

## JavaScript / TypeScript

The API reference for the JS/TS bindings are available [here](../jstsdocs/classes/CryptpaySdk.html). Please consult the tables below for the dependencies between each function.

## SDK Initialization and Configuration

### Instantiating the SDK

| Method | Arguments | Returns | Errors | Dependencies | Level | Repeat|
|--------|-----------|---------|--------|--------------|-------|-------|
| Constructor |  | Returns an `Error` if an error occurs, otherwise the handle to the SDK | Returns an `Error` if there is an issue in loading the dynamically or statically linked binary shared library | | Basic | Handle|

=== "Rust"

    ```rust
    use sdk::Sdk;
    let mut sdk = Sdk::default();
    ```

=== "Java"

    ```java
    import com.cawaena.Wallet;
    sdk = new Wallet();
    ```
=== "Swift"

    ```swift
    import cryptpaysdk
    let sdk = Sdk()
    ```

### Set path prefix

| Method | Arguments | Returns | Errors | Dependencies | Level | Repeat|
|--------|-----------|---------|--------|--------------|-------|-------|
| Sets the path prefix | `path_prefix` - The path to the storage where SDK has read/write file I/O access | | | [Constructor](./SDK%20API%20Reference.md#instantiating-the-sdk) | Basic | Handle|

=== "Rust"

    ```rust
    fn set_path_prefix(&mut self, path_prefix: &str)
    ```

=== "Java"

    ```java
    public void setStoragePath(String path_prefix) throws Exception
    ```
=== "Swift"

    ```swift
    public func setPathPrefix(path_prefix: String)
    ```

### Set authentication provider

| Method | Arguments | Returns | Errors | Dependencies | Level | Repeat|
|--------|-----------|---------|--------|--------------|-------|-------|
| Sets the auth provider | `auth_provider` - The string identifying the auth provider | | | [Constructor](./SDK%20API%20Reference.md#instantiating-the-sdk) | Basic | Handle|

=== "Rust"

    ```rust
    fn set_auth_provider(&mut self, auth_provider: &str)
    ```
=== "Java"

    ```java
    public void authProvider(String authProvider) throws Exception
    ```
=== "Swift"

    ```swift
    public func setAuthProvider(auth_provider: String)
    ```

### Set backend URL

| Method | Arguments | Returns | Errors | Dependencies | Level | Repeat|
|--------|-----------|---------|--------|--------------|-------|-------|
| Sets the backend URL | `backend_url` - The string containing the backend URL | | | [Constructor](./SDK%20API%20Reference.md#instantiating-the-sdk) | Basic | Handle|

=== "Rust"

    ```rust
    fn set_backend_url(&mut self, backend_url: &str)
    ```
=== "Java"

    ```java
    public void backendUrl(String backendUrl) throws Exception 
    ```
=== "Swift"

    ```swift
    public func setBackendUrl(backend_url: String)
    ```

### Set log level

| Method | Arguments | Returns | Errors | Dependencies | Level | Repeat|
|--------|-----------|---------|--------|--------------|-------|-------|
| Sets the log level | `log_level` - The log level as string. Allowed values are `info`, `error`, `warn` and `debug`  | | Error, if the matching log level string cannot be parsed | [Constructor](./SDK%20API%20Reference.md#instantiating-the-sdk) | Basic | Handle|

=== "Rust"

    ```rust
    fn set_log_level(&mut self, log_level: &str)
    ```
=== "Java"

    ```java
    public void logLevel(String logLevel) throws Exception 
    ```
=== "Swift"

    ```swift
    public func setLogLevel(log_level: String)
    ```

### Set environment

| Method | Arguments | Returns | Errors | Dependencies | Level | Repeat|
|--------|-----------|---------|--------|--------------|-------|-------|
| Sets the environment| `env` - The environment as string. Allowed values are `qa`, `development`, `staging` and `production`  | | Error, if the matching environment string cannot be parsed | [Constructor](./SDK%20API%20Reference.md#instantiating-the-sdk) | Basic | Handle|

=== "Rust"

    ```rust
    fn set_env(&mut self, env: Environment)
    ```
=== "Java"

    ```java
    public void logLevel(String logLevel) throws Exception 
    ```
=== "Swift"

    ```swift
    public func setSdkEnv(sdk_env: String) throws
    ```

### Set currency

| Method | Arguments | Returns | Errors | Dependencies | Level | Repeat|
|--------|-----------|---------|--------|--------------|-------|-------|
| Sets the currency| `currency` - The currency as string. Allowed values are `smr`and `iota` | | Error, if the matching currency string cannot be parsed | [Constructor](./SDK%20API%20Reference.md#instantiating-the-sdk) | Basic | Handle|

=== "Rust"

    ```rust
    fn set_currency(&mut self, currency: Currency)
    ```
=== "Java"

    ```java
    public void setCurrency(String currency) throws Exception 
    ```
=== "Swift"

    ```swift
    public func setCurrency(currency: String) throws 
    ```

### Validate config

| Method | Arguments | Returns | Errors | Dependencies | Level | Repeat|
|--------|-----------|---------|--------|--------------|-------|-------|
| Validates the SDK configuration |  | | Error, if the configuration is invalid | [Constructor](./SDK%20API%20Reference.md#instantiating-the-sdk) | Basic (optional) | Handle|

=== "Rust"

    ```rust
    fn validate_config(&self) -> Result<(), crate::error::Error>
    ```
=== "Java"

    ```java
    public void checkConfig() throws Exception
    ```
=== "Swift"

    ```swift
    public func validateConfig() throws
    ```

### Initialize logger

| Method | Arguments | Returns | Errors | Dependencies | Level | Repeat|
|--------|-----------|---------|--------|--------------|-------|-------|
| Initializes the logger |  | | Error, if the logger cannot be initialized | [Constructor](./SDK%20API%20Reference.md#instantiating-the-sdk), [Path Prefix](./SDK%20API%20Reference.md#set-path-prefix), [Log Level](./SDK%20API%20Reference.md#set-log-level) | Basic | Handle|

=== "Rust"

    ```rust
    fn init_logger(&self) -> Result<(), crate::error::Error> 
    ```
=== "Java"

    ```java
    public void initLogger() throws Exception
    ```
=== "Swift"

    ```swift
    public func initLogger() throws
    ```

## User functions

### Creating a new user

| Method | Arguments | Returns | Errors | Dependencies | Level | Repeat|
|--------|-----------|---------|--------|--------------|-------|-------|
| Create a new user | `username` - The username of the new user. | Returns an `Error` if an error occurs, otherwise void | Returns an `Error` if there is an issue validating the configuration, initializing the repository, or creating the user. | [Constructor](./SDK%20API%20Reference.md#instantiating-the-sdk), [Path Prefix](./SDK%20API%20Reference.md#set-path-prefix) | Basic | User|

=== "Rust"

    ```rust
    async fn create_new_user(&mut self, username: &str) -> Result<(), crate::error::Error>;
    ```

=== "Java"

    ```java
    public void createNewUser(String username) throws Exception
    ```
=== "Swift"

    ```swift
    public func createNewUser(username: String) throws
    ```

### Initializing a user

| Method | Arguments | Returns | Errors | Dependencies | Level | Repeat|
|--------|-----------|---------|--------|--------------|-------|-------|
| Initialize a user | `username` - The username of the user to initialize. | Returns, if the user is initialized successfully, or an `Error` if an error occurs. | Returns an `Error` if there is an issue validating the configuration, initializing the repository, or checking the KYC status. | [Constructor](./SDK%20API%20Reference.md#instantiating-the-sdk), [Path Prefix](./SDK%20API%20Reference.md#set-path-prefix), [Refresh access token](./SDK%20API%20Reference.md#refreshing-access-token), [Create new user](./SDK%20API%20Reference.md#creating-a-new-user) | Usage | Application|

=== "Rust"

    ```rust
    async fn init_user(&mut self, username: &str) -> Result<(), crate::error::Error>;
    ```

=== "Java"

    ```java
    public void initializeUser(String username) throws Exception
    ```
=== "Swift"

    ```swift
    public func initUser(username: String) throws 
    ```

### Refreshing access token

| Method | Arguments | Returns | Errors | Dependencies | Level | Repeat|
|--------|-----------|---------|--------|--------------|-------|-------|
| Refresh access token | `access_token` - The new access token to be set. |Returns, if the access token is refreshed successfully, or an `Error` if an error occurs. | Returns an `Error` if there is an issue validating the configuration. |[Constructor](./SDK%20API%20Reference.md#instantiating-the-sdk), [Path Prefix](./SDK%20API%20Reference.md#set-path-prefix)| Basic | Application |

=== "Rust"

    ```rust
    async fn refresh_access_token(&mut self, access_token: &str) -> Result<(), crate::error::Error>
    ```

=== "Java"

    ```java
    public void refreshAccessToken(String access_token) throws Exception
    ```
=== "Swift"

    ```swift
    public func refreshAccessToken(access_token: String) throws 
    ```

### Checking KYC status

| Method | Arguments | Returns | Errors | Dependencies | Level | Repeat|
|--------|-----------|---------|--------|--------------|-------|-------|
| Check if KYC status is verified |`username` - The username of the user to check KYC status for. | Returns `true` if the KYC status is verified, or `false` if it is not verified. | Returns an `Error` if there is an issue validating the configuration, initializing the repository, or checking the KYC status. | [User initialization](./SDK%20API%20Reference.md#initializing-a-user) | Usage | Application |

=== "Rust"

    ```rust
    async fn is_kyc_status_verified(&mut self, username: &str) -> Result<bool, crate::error::Error>
    ```

=== "Java"

    ```java
    public boolean isKycVerified(String username) throws Exception
    ```
=== "Swift"

    ```swift
    public func isKycVerified(username: String) throws
    -> Bool
    ```

### Delete user

| Method | Arguments | Returns | Errors | Dependencies | Level | Repeat|
|--------|-----------|---------|--------|--------------|-------|-------|
| Delete the currently active user and their wallet | `pin` - The PIN of the user to be deleted | Returns, if the user is deleted successfully, or an `Error` if an error occurs. | Returns an `Error` if there is an issue verifying the PIN, initializing the repository, deleting the user, or deleting the wallet. | [User initialization](./SDK%20API%20Reference.md#initializing-a-user) | Usage | Application |

=== "Rust"

    ```rust
    async fn delete_user(&mut self, pin: &str) -> Result<(), crate::error::Error>
    ```

=== "Java"

    ```java
    public void deleteUser(String pin) throws Exception
    ```
=== "Swift"

    ```swift
    public func deleteUser(pin: String) throws
    ```

## Wallet functions

### Create new wallet

| Method | Arguments | Returns | Errors | Dependencies | Level | Repeat|
|--------|-----------|---------|--------|--------------|-------|-------|
| Creates a new wallet for the user with the specified PIN and password | `pin` - The PIN for the wallet, `password` - The password for the wallet | Returns the mnemonic phrase of the newly created wallet if successful, otherwise returns an `Error`. | `RepositoryInitError` - If there is an error initializing the repository, `UserInitError` - If there is an error initializing the user, `WalletInitError` - If there is an error initializing the wallet | [User initialization](./SDK%20API%20Reference.md#initializing-a-user) | Usage | User |

=== "Rust"

    ```rust
    async fn create_new_wallet(&mut self, pin: &str, password: &str) -> Result<String, crate::error::Error>
    ```

=== "Java"

    ```java
    public String createNewWallet(String pin, String password) throws Exception
    ```
=== "Swift"

    ```swift
    public func createNewWallet(pin: String, password: String) throws -> String
    ```

### Create new wallet from mnemonic

| Method | Arguments | Returns | Errors | Dependencies | Level | Repeat|
|--------|-----------|---------|--------|--------------|-------|-------|
| Migrate a wallet from existing mnemonic | `pin` - The PIN for the wallet, `password` - The password for the wallet, `mnemonic` - The mnemonic to migrate from | Returns, if the wallet is successfully created, otherwise returns an `Error`. | `RepositoryInitError` - If there is an error initializing the repository, `UserInitError` - If there is an error initializing the user, `WalletInitError` - If there is an error initializing the wallet | [User initialization](./SDK%20API%20Reference.md#initializing-a-user) | Usage | User |

=== "Rust"

    ```rust
    async fn create_wallet_from_mnemonic(
        &mut self,
        pin: &str,
        password: &str,
        mnemonic: &str,
    ) -> Result<(), crate::error::Error>
    ```

=== "Java"

    ```java
    public void createWalletFromMnemonic(String pin, String password, String mnemonic) throws Exception 
    ```
=== "Swift"

    ```swift
    public func createWalletFromMnemonic(pin: String, password: String, mnemonic: String) throws
    ```

### Create new wallet from backup

| Method | Arguments | Returns | Errors | Dependencies | Level | Repeat|
|--------|-----------|---------|--------|--------------|-------|-------|
| Migrate a wallet from existing backup | `pin` - The PIN for the wallet, `password` - The password for the backup, `backup_path` - The path to the backup file | Returns, if the wallet is successfully created, otherwise returns an `Error`. | `RepositoryInitError` - If there is an error initializing the repository, `UserInitError` - If there is an error initializing the user, `WalletInitError` - If there is an error initializing the wallet | [User initialization](./SDK%20API%20Reference.md#initializing-a-user) | Usage | Application |

=== "Rust"

    ```rust
    async fn create_wallet_from_backup(
        &mut self,
        pin: &str,
        password: &str,
        backup_path: &str,
    ) -> Result<(), crate::error::Error>
    ```

=== "Java"

    ```java
    public void createWalletFromBackup(String pin, String password, String backup_path) throws Exception 
    ```
=== "Swift"

    ```swift    
    public func createWalletFromBackup(pin: String, password: String, backup_path: String) throws
    ```

### Create a wallet backup

| Method | Arguments | Returns | Errors | Dependencies | Level | Repeat|
|--------|-----------|---------|--------|--------------|-------|-------|
| Create a wallet backup | `password` - The password for the backup | Returns the path to the created backup file as a `String` if successful, otherwise returns an `Error`. | `WalletInitError` - If there is an error initializing the wallet | [User initialization](./SDK%20API%20Reference.md#initializing-a-user) | Usage | Application |

=== "Rust"

    ```rust
    async fn create_wallet_backup(
        &mut self,
        password: &str,
    ) -> Result<(), crate::error::Error>
    ```

=== "Java"

    ```java
    public String createWalletBackup(String backup_password) throws Exception
    ```
=== "Swift"

    ```swift    
    public func createWalletBackup(password: String) throws -> String
    ```

### Initialize a wallet

| Method | Arguments | Returns | Errors | Dependencies | Level | Repeat|
|--------|-----------|---------|--------|--------------|-------|-------|
| Initialize an existing wallet | `pin` - The PIN for the wallet. | Returns, if the wallet is successfully initialized, otherwise returns an `Error`. | `RepositoryInitError` - If there is an error initializing the repository, `UserInitError` - If there is an error initializing the user, `MissingPassword` - If the encrypted password is missing, `WrongPinOrPassword` - If the PIN or password is incorrect | [User initialization](./SDK%20API%20Reference.md#initializing-a-user) | Usage | Application |

=== "Rust"

    ```rust
    async fn init_wallet(&mut self, pin: &str) -> Result<(), crate::error::Error>
    ```

=== "Java"

    ```java
    public void initializeWallet(String pin) throws Exception
    ```
=== "Swift"

    ```swift    
    public func initWallet(pin: String) throws
    ```

### Verify mnemonic

| Method | Arguments | Returns | Errors | Dependencies | Level | Repeat|
|--------|-----------|---------|--------|--------------|-------|-------|
| Verifies the mnemonic by deleting the current wallet and creating a new wallet from the provided mnemonic | `pin` - The PIN for the wallet, `password` - The password for the wallet, `mnemonic` - The mnemonic to verify | Returns, if the mnemonic is successfully verified, otherwise returns an `Error` | `WalletInitError` - If there is an error initializing the wallet, `RepositoryInitError` - If there is an error initializing the repository, `UserInitError` - If there is an error initializing the user,  `Error` - If there is an error deleting the wallet or creating a new wallet from the mnemonic. | [User initialization](./SDK%20API%20Reference.md#initializing-a-user) | Usage | User |

=== "Rust"

    ```rust
    async fn verify_mnemonic(&mut self, pin: &str, password: &str, mnemonic: &str) -> Result<(), crate::error::Error>
    ```

=== "Java"

    ```java
    public void verifyMnemonic(String pin, String password, String mnemonic) throws Exception
    ```
=== "Swift"

    ```swift    
    public func verifyMnemonic(pin: String, password: String, mnemonic: String) throws
    ```

### Delete wallet

!!! warning

    Deletes the currently active wallet, potentially resulting in loss of funds if the mnemonic or wallet is not backed up.

| Method | Arguments | Returns | Errors | Dependencies | Level | Repeat|
|--------|-----------|---------|--------|--------------|-------|-------|
| Delete the currently active wallet | | Returns, if the wallet is successfully deleted, otherwise returns an `Error`. | `WalletInitError` - If there is an error initializing the wallet,  `Error` - If there is an error closing or deleting the wallet | [Wallet initialization](./SDK%20API%20Reference.md#initialize-a-wallet) | Usage | Application |

=== "Rust"

    ```rust
    async fn delete_wallet(&mut self) -> Result<(), crate::error::Error> 
    ```

=== "Java"

    ```java
    public void deleteWallet() throws Exception
    ```
=== "Swift"

    ```swift    
    public func deleteWallet() throws
    ```

### Verify pin

| Method | Arguments | Returns | Errors | Dependencies | Level | Repeat|
|--------|-----------|---------|--------|--------------|-------|-------|
| Verifies the pin for the wallet | `pin` - The pin to verify | Returns, if the pin is verified successfully, otherwise returns an `Error`. | `RepositoryInitError` - If there is an error initializing the repository, `UserInitError` - If there is an error initializing the user, `WalletInitError` - If there is an error initializing the wallet, `MissingPassword` - If the password is missing, `WrongPinOrPassword` - If the pin or password is incorrect| [Wallet initialization](./SDK%20API%20Reference.md#initialize-a-wallet) | Usage | Application |

=== "Rust"

    ```rust
    async fn verify_pin(&self, pin: &str) -> Result<(), crate::error::Error>
    ```

=== "Java"

    ```java
    public void pinVerify(String pin) throws Exception
    ```
=== "Swift"

    ```swift    
    public func verifyPin(pin: String) throws
    ```

### Reset pin

| Method | Arguments | Returns | Errors | Dependencies | Level | Repeat|
|--------|-----------|---------|--------|--------------|-------|-------|
| Resets the pin for the wallet using the provided password and new pin | `password` - The current password for the wallet, `new_pin` - The new pin to set for the wallet | Returns, if the pin is reset successfully, otherwise returns an `Error` | `RepositoryInitError` - If there is an error initializing the repository, `UserInitError` - If there is an error initializing the user, `MissingPassword` - If the password is missing, `WrongPinOrPassword` - If the pin or password is incorrect| [Wallet initialization](./SDK%20API%20Reference.md#initialize-a-wallet) | Usage | Application |

=== "Rust"

    ```rust
    async fn reset_pin(&mut self, password: &str, new_pin: &str) -> Result<(), crate::error::Error>
    ```

=== "Java"

    ```java
    public void pinReset(String password, String pin) throws Exception 
    ```
=== "Swift"

    ```swift    
    public func resetPin(password: String, new_pin: String) throws 
    ```

### Change password

| Method | Arguments | Returns | Errors | Dependencies | Level | Repeat|
|--------|-----------|---------|--------|--------------|-------|-------|
| Changes the password for the wallet using the provided pin, current password, and new password | `pin` - The pin to verify, `current_password` - The current password for the wallet, `new_password` - The new password to set for the wallet | Returns, if the password is changed successfully, otherwise returns an `Error` | `RepositoryInitError` - If there is an error initializing the repository, `UserInitError` - If there is an error initializing the user, `WalletInitError` - If there is an error initializing the wallet | [Wallet initialization](./SDK%20API%20Reference.md#initialize-a-wallet) | Usage | Application |

=== "Rust"

    ```rust
    async fn change_password(
        &mut self,
        pin: &str,
        current_password: &str,
        new_password: &str,
    ) -> Result<(), crate::error::Error> 
    ```

=== "Java"

    ```java
    public void passwordChange(String pin, String current_password, String new_password) throws Exception
    ```
=== "Swift"

    ```swift    
    public func changePassword(pin: String, current_password: String, new_password: String) throws 
    ```

### Generate a new address

| Method | Arguments | Returns | Errors | Dependencies | Level | Repeat|
|--------|-----------|---------|--------|--------------|-------|-------|
| Generates a new receiver address for the wallet | | Returns the generated address as a `String` if successful, otherwise returns an `Error`| `UserInitError` - If there is an error initializing the user, `WalletInitError` - If there is an error initializing the wallet | [Wallet initialization](./SDK%20API%20Reference.md#initialize-a-wallet) | Usage | Application |

=== "Rust"

    ```rust
    async fn generate_new_iota_receiver_address(&self) -> Result<String, crate::error::Error>
    ```

=== "Java"

    ```java
    public String generateNewIotaReceiverAddress() throws Exception
    ```
=== "Swift"

    ```swift    
    public func generateNewIotaReceiverAddress() throws -> String
    ```

### Get balance

| Method | Arguments | Returns | Errors | Dependencies | Level | Repeat|
|--------|-----------|---------|--------|--------------|-------|-------|
| Fetches the balance of the user from the wallet | | Returns the balance as a `f64` if successful, otherwise returns an `Error`| `WalletInitError` - If there is an error initializing the wallet | [Wallet initialization](./SDK%20API%20Reference.md#initialize-a-wallet) | Usage | Application |

=== "Rust"

    ```rust
    async fn get_balance(&self) -> Result<f64, crate::error::Error>
    ```

=== "Java"

    ```java
    public double getBalance() throws Exception
    ```
=== "Swift"

    ```swift    
    public func getBalance() throws -> Double
    ```

### Get wallet transactions

| Method | Arguments | Returns | Errors | Dependencies | Level | Repeat|
|--------|-----------|---------|--------|--------------|-------|-------|
| Wallet transaction list | `page` - The page number for paginatation, `page_size` - The page size for each page | Returns the list of transactions made on the wallet as an array of `WalletTxInfo` object or a serialized JSON of the same, if successful, otherwise returns an `Error`| `WalletInitError` - If there is an error initializing the wallet | [Wallet initialization](./SDK%20API%20Reference.md#initialize-a-wallet) | Usage | Application |

=== "Rust"

    ```rust
    async fn get_wallet_tx_list(&self, page: usize, page_size: usize) -> Result<WalletTxInfoList, crate::error::Error>
    ```

=== "Java"

    ```java
    public String getWalletTransactionList(long page, long pageSize) throws Exception
    ```
=== "Swift"

    ```swift    
    public func getWalletTransactionList(page: UInt64, pageSize: UInt64) throws -> String
    ```

#### WalletTxInfo

=== "Rust"

    ```Rust
    pub struct WalletTxInfo {
    /// Tx creation date, if available
    pub date: String,
    /// Contains block id
    pub block_id: Option<String>,
    /// transaction id for particular transaction
    pub transaction_id: String,
    /// Describes type of transaction
    pub incoming: bool,
    /// Amount of transfer
    pub amount: f64,
    /// either SMR or IOTA [convert network_id to string based on the value]
    pub network: String,
    /// Status of the transfer
    pub status: String,
    /// Url of network SMR/IOTA
    pub explorer_url: Option<String>,
    // change based on the network either shimmer or iota
    // base explorer url for SMR = https://explorer.shimmer.network/shimmer/block/[block_id]
    // base explorer url for IOTA = https://explorer.iota.org/mainnet/block/[block_id]
    }
    ```

=== "JSON"

    ```json
    {
        "$schema": "<http://json-schema.org/draft-07/schema#>",
        "type": "object",
        "properties": {
            "date": {
                "type": "string",
                "description": "Tx creation date, if available"
            },
            "block_id": {
                "type": ["string", "null"],
                "description": "Contains block id"
            },
            "transaction_id": {
                "type": "string",
                "description": "transaction id for particular transaction"
            },
            "incoming": {
                "type": "boolean",
                "description": "Describes type of transaction"
            },
            "amount": {
                "type": "number",
                "description": "Amount of transfer"
            },
            "network": {
                "type": "string",
                "description": "either SMR or IOTA [convert network_id to string based on the value]"
            },
            "status": {
                "type": "string",
                "description": "Status of the transfer"
            },
            "explorer_url": {
                "type": ["string", "null"],
                "description": "Url of network SMR/IOTA explorer"
            }
        },
        "required": ["date", "transaction_id", "incoming", "amount", "network", "status"]
    }

    ```

### Get wallet transaction

| Method | Arguments | Returns | Errors | Dependencies | Level | Repeat|
|--------|-----------|---------|--------|--------------|-------|-------|
| Single wallet transaction | `tx_id` - The transaction id on the network | Returns the transactions made on the wallet with the given id as `WalletTxInfo` object or a serialized JSON of the same, if successful, otherwise returns an `Error`| `WalletInitError` - If there is an error initializing the wallet | [Wallet initialization](./SDK%20API%20Reference.md#initialize-a-wallet) | Usage | Application |

=== "Rust"

    ```rust
    async fn get_wallet_tx(&self, tx_id: &str) -> Result<WalletTxInfo, crate::error::Error>
    ```

=== "Java"

    ```java
    public String getWalletTransaction(String transactionId) throws Exception
    ```
=== "Swift"

    ```swift    
    public func getWalletTransaction(transactionId: String) throws -> String
    ```

## Viviswap functions

### Start KYC Verification for viviswap

| Method | Arguments | Returns | Errors | Dependencies | Level | Repeat|
|--------|-----------|---------|--------|--------------|-------|-------|
| Create new viviswap user and initialize kyc verification | `mail` - The email address of the user, `terms_accepted` - A boolean indicating whether the terms have been accepted | Returns`NewViviswapUser` if successful, or an `Error`, if an error occurs| Repository initialization error, User already exists, Viviswap API error, User status update error  | [User initialization](./SDK%20API%20Reference.md#initializing-a-user) | Usage | Application |

=== "Rust"

    ```rust
    async fn start_kyc_verification_for_viviswap(
        &self,
        mail: &str,
        terms_accepted: bool,
    ) -> Result<NewViviswapUser, crate::error::Error>
    ```

=== "Java"

    ```java
    public String startViviswapKyc(String mail, boolean terms_accepted) throws Exception
    ```
=== "Swift"

    ```swift    
    public func startKycVerificationForViviswap(mail: String, termsAccepted: Bool) throws -> String
    ```

#### NewViviswapUser

=== "Rust"

    ```Rust
    pub struct NewViviswapUser {
    /// Username of new viviswap user
    pub username: String,
    }
    ```

=== "JSON"
    ```json
    {
        "$schema": "<http://json-schema.org/draft-07/schema#>",
        "type": "object",
        "properties": {
            "username": {
                "type": "string",
                "description": "Username of new viviswap user"
            }
        },
        "required": ["username"]
    }
    ```

### Get KYC details for viviswap

| Method | Arguments | Returns | Errors | Dependencies | Level | Repeat|
|--------|-----------|---------|--------|--------------|-------|-------|
| Get current kyc status of viviswap | | Returns `ViviswapKycStatus` if successful, or an `Error` if an error occurs| Repository initialization error, Viviswap API error | [User initialization](./SDK%20API%20Reference.md#initializing-a-user) | Usage | Application |

=== "Rust"

    ```rust
    async fn get_kyc_details_for_viviswap(&self) -> Result<ViviswapKycStatus, crate::error::Error>
    ```

=== "Java"

    ```java
    public String getViviswapKyc() throws Exception
    ```
=== "Swift"

    ```swift    
    public func getKycDetailsForViviswap() throws -> String
    ```

#### ViviswapKycStatus

=== "Rust"

    ```Rust
    pub struct ViviswapKycStatus {
        /// full name of the user
        pub full_name: String,
        /// the current submission step in the KYC onboarding process for the user
        pub submission_step: ViviswapVerificationStep,
        /// the current verified step in the KYC onboarding process for the user
        pub verified_step: ViviswapVerificationStep,
        /// the user verification status
        pub verification_status: ViviswapVerificationStatus,
        /// The monthly swap limit in euros
        pub monthly_limit_eur: f32,
    }
    pub enum ViviswapVerificationStep {
        /// no verification step (no next verification step available)
        Undefined,
        /// general verification step
        General,
        /// personal verification step
        Personal,
        /// residence verification step
        Residence,
        /// identity verification step
        Identity,
        /// amla general verification step
        Amla,
        /// document verification step
        Documents,
    }
    pub enum ViviswapVerificationStatus {
        /// The user is fully verified
        Verified,
        /// The user is not verified
        Unverified,
        /// The user is partially verified
        PartiallyVerified,
    }
    ```

=== "JSON"

    ```json
    {
        "$schema": "http://json-schema.org/draft-07/schema#",
        "type": "object",
        "properties": {
            "full_name": {
                "type": "string",
                "description": "Full name of the user"
            },
            "submission_step": {
                "type": "string",
                "enum": ["Undefined", "General", "Personal", "Residence", "Identity", "Amla", "Documents"],
                "description": "The current submission step in the KYC onboarding process for the user"
            },
            "verified_step": {
                "type": "string",
                "enum": ["Undefined", "General", "Personal", "Residence", "Identity", "Amla", "Documents"],
                "description": "The current verified step in the KYC onboarding process for the user"
            },
            "verification_status": {
                "type": "string",
                "enum": ["Verified", "Unverified", "PartiallyVerified"],
                "description": "The user verification status"
            },
            "monthly_limit_eur": {
                "type": "number",
                "description": "The monthly swap limit in euros"
            }
        },
        "required": ["full_name", "submission_step", "verified_step", "verification_status", "monthly_limit_eur"]
    }
    ```

### Update partial KYC for viviswap

| Method | Arguments | Returns | Errors | Dependencies | Level | Repeat|
|--------|-----------|---------|--------|--------------|-------|-------|
| Update the kyc details for viviswap to be submitted | `is_individual` - Whether the user is an individual, `is_pep` - Whether the user is a politically exposed person, `is_us_citizen` - Whether the user is a US citizen, `is_regulatory_disclosure` - Whether the user has accepted the regulatory disclosure, `country_of_residence` - The country of residence of the user, `nationality` - The nationality of the user, `full_name` - The full name of the user, `date_of_birth` - The date of birth of the user | Returns `ViviswapPartiallyKycDetails` containing the partially updated KYC details or a vector of errors for each input field |  A vector of errors if any validation errors occur during the update process | [User initialization](./SDK%20API%20Reference.md#initializing-a-user) | Usage | Application |

=== "Rust"

    ```rust
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
    ) -> Result<ViviswapPartiallyKycDetails, Vec<crate::error::Error>>
    ```

=== "Java"

    ```java
    public String updateViviswapKycPartial(boolean is_individual, boolean is_pep, boolean is_us_citizen,
            boolean is_regulatory_disclosure, String country_of_residence, String nationality, String full_name,
            String date_of_birth) throws Exception
    ```
=== "Swift"

    ```swift
    public func updateKycPartiallyStatusForViviswap(
        isIndividual: Bool,
        isPep: Bool,
        isUsCitizen: Bool,
        isRegulatoryDisclosure: Bool,
        countryOfResidence: String,
        nationality: String,
        fullName: String,
        dateOfBirth: String
    ) throws -> String
    ```

#### ViviswapPartiallyKycDetails

=== "Rust"

    ```Rust
    pub struct ViviswapPartiallyKycDetails {
        /// Is the user an individual
        pub is_individual: Option<bool>,
        /// Is the user a politically exposed person
        pub is_pep: Option<bool>,
        /// Is the user a US citizen
        pub is_us_citizen: Option<bool>,
        /// Is the regulatory disclosure confirmed by user
        pub is_regulatory_disclosure: Option<bool>,
        /// The country of tax residence of the user
        pub country_of_residence: Option<String>,
        /// The user's nationality
        pub nationality: Option<String>,
        /// The full name of the user as per his legal documents
        pub full_name: Option<String>,
        /// The date of birth of the user as per his legal documents
        pub date_of_birth: Option<String>,
    }
    ```

=== "JSON"

    ```json
    {
        "$schema": "http://json-schema.org/draft-07/schema#",
        "type": "object",
        "properties": {
            "is_individual": {
                "type": ["boolean", "null"],
                "description": "Is the user an individual"
            },
            "is_pep": {
                "type": ["boolean", "null"],
                "description": "Is the user a politically exposed person"
            },
            "is_us_citizen": {
                "type": ["boolean", "null"],
                "description": "Is the user a US citizen"
            },
            "is_regulatory_disclosure": {
                "type": ["boolean", "null"],
                "description": "Is the regulatory disclosure confirmed by user"
            },
            "country_of_residence": {
                "type": ["string", "null"],
                "description": "The country of tax residence of the user"
            },
            "nationality": {
                "type": ["string", "null"],
                "description": "The user's nationality"
            },
            "full_name": {
                "type": ["string", "null"],
                "description": "The full name of the user as per his legal documents"
            },
            "date_of_birth": {
                "type": ["string", "null"],
                "description": "The date of birth of the user as per his legal documents"
            }
        }
    }
    ```

### Submit partial KYC for viviswap

| Method | Arguments | Returns | Errors | Dependencies | Level | Repeat|
|--------|-----------|---------|--------|--------------|-------|-------|
| Submit the kyc details for viviswap  | | Returns,  if the submission is successful |  Returns a vector of `Error` if any or many of the following conditions occur: Repository initialization error, Viviswap missing user error, Viviswap invalid state error, Viviswap missing field error, Viviswap API error | [User initialization](./SDK%20API%20Reference.md#initializing-a-user) | Usage | Application |

=== "Rust"

    ```rust
    async fn submit_kyc_partially_status_for_viviswap(&self) -> Result<(), Vec<crate::error::Error>>
    ```

=== "Java"

    ```java
    public void submitViviswapKycPartial() throws Exception
    ```
=== "Swift"

    ```swift
    public func submitKycPartiallyStatusForViviswap() throws
    ```

### Get IBAN for viviswap

| Method | Arguments | Returns | Errors | Dependencies | Level | Repeat|
|--------|-----------|---------|--------|--------------|-------|-------|
| Get current iban of viviswap user  | | Returns `ViviswapAddressDetail`, if successful otherwise `Error` |  Returns `ViviswapInvalidStateError` - If the viviswap state is invalid, `RepositoryInitError` - If the repository initialization fails, `ViviswapApiError` - If there is an error in the viviswap API, `UserStatusUpdateError` - If there is an error updating the user status  | [Wallet initialization](./SDK%20API%20Reference.md#initialize-a-wallet), [Get KYC Details for viviswap](./SDK%20API%20Reference.md#get-kyc-details-for-viviswap) | Usage | Application |

=== "Rust"

    ```rust
    async fn get_iban_for_viviswap(&self) -> Result<ViviswapAddressDetail, crate::error::Error>
    ```

=== "Java"

    ```java
    public String getIbanViviswap() throws Exception
    ```
=== "Swift"

    ```swift
    public func getIbanViviswap() throws -> String
    ```

#### ViviswapAddressDetail

=== "Rust"

    ```Rust
    pub struct ViviswapAddressDetail {
        /// the unique id of the address detail
        pub id: String,
        /// the address used in the detail
        pub address: String,
        /// the status from viviswap, whether the address is verified
        pub is_verified: bool,
    }
    ```

=== "JSON"

    ```json
    {
        "$schema": "http://json-schema.org/draft-07/schema#",
        "type": "object",
        "properties": {
            "id": {
                "type": "string",
                "description": "The unique id of the address detail"
            },
            "address": {
                "type": "string",
                "description": "The address used in the detail"
            },
            "is_verified": {
                "type": "boolean",
                "description": "The status from viviswap, whether the address is verified"
            }
        },
        "required": ["id", "address", "is_verified"]
    }
    ```

### Update IBAN for viviswap

| Method | Arguments | Returns | Errors | Dependencies | Level | Repeat|
|--------|-----------|---------|--------|--------------|-------|-------|
| Update IBAN of viviswap user | `pin` - The user's PIN, `address` - The new IBAN address | Returns `ViviswapAddressDetail`, if successful otherwise `Error` |  Returns `RepositoryInitError` - If the repository initialization fails, `ViviswapMissingUserError` - If the viviswap user is missing, `UserStatusUpdateError` - If there is an error updating the user status  | [Wallet initialization](./SDK%20API%20Reference.md#initialize-a-wallet), [Get KYC Details for viviswap](./SDK%20API%20Reference.md#get-kyc-details-for-viviswap) | Usage | Application |

=== "Rust"

    ```rust
    async fn update_iban_for_viviswap(
        &self,
        pin: String,
        address: String,
    ) -> Result<ViviswapAddressDetail, crate::error::Error>
    ```

=== "Java"

    ```java
    public String updateIbanViviswap(String pin, String address) throws Exception
    ```
=== "Swift"

    ```swift
    public func updateIbanViviswap(pin: String, address: String) throws -> String
    ```

### Create deposit with viviswap

| Method | Arguments | Returns | Errors | Dependencies | Level | Repeat|
|--------|-----------|---------|--------|--------------|-------|-------|
| Create deposit for viviswap user | | Returns `ViviswapDeposit`, if successful otherwise `Error` |  Returns `RepositoryInitError` - If the repository initialization fails, `ViviswapMissingUserError` - If the viviswap user is missing, `ViviswapInvalidStateError` - If the viviswap state is invalid, `ViviswapApiError` - If there is an error with the Viviswap API  | [Wallet initialization](./SDK%20API%20Reference.md#initialize-a-wallet), [Get KYC Details for viviswap](./SDK%20API%20Reference.md#get-kyc-details-for-viviswap), [Update IBAN](./SDK%20API%20Reference.md#update-iban-for-viviswap) | Usage | Application |

=== "Rust"

    ```rust
    async fn create_deposit_with_viviswap(&self) -> Result<ViviswapDeposit, crate::error::Error>
    ```

=== "Java"

    ```java
    public String depositWithViviswap() throws Exception
    ```
=== "Swift"

    ```swift
    public func depositWithViviswap() throws -> String
    ```

#### ViviswapDeposit

=== "Rust"

    ```Rust
    pub struct ViviswapDeposit {
        /// The unique UUID of the contract
        pub contract_id: String,
        /// The deposit address (crypto) where the swap will put the funds from fiat
        pub deposit_address: String,
        /// The details of the deposit (for the user)
        pub details: ViviswapDepositDetails,
    }
    pub struct ViviswapDepositDetails {
        /// The reference to be entered by the user in his SEPA bank transfer
        pub reference: String,
        /// The name of the beneficiary receiving the SEPA transfer
        pub beneficiary: String,
        /// The name of the bank of the beneficiary
        pub name_of_bank: String,
        /// The address of the bank of the beneficiary
        pub address_of_bank: String,
        /// The IBAN of the beneficiary
        pub iban: String,
        /// The BIC/SWIFT code for the SEPA transfer
        pub bic: String,
    }
    ```

=== "JSON"

    ```json
    {
        "$schema": "http://json-schema.org/draft-07/schema#",
        "type": "object",
        "properties": {
            "contract_id": {
                "type": "string",
                "description": "The unique UUID of the contract"
            },
            "deposit_address": {
                "type": "string",
                "description": "The deposit address (crypto) where the swap will put the funds from fiat"
            },
            "details": {
                "type": "object",
                "description": "The details of the deposit (for the user)",
                "properties": {
                    "reference": {
                        "type": "string",
                        "description": "The reference to be entered by the user in his SEPA bank transfer"
                    },
                    "beneficiary": {
                        "type": "string",
                        "description": "The name of the beneficiary receiving the SEPA transfer"
                    },
                    "name_of_bank": {
                        "type": "string",
                        "description": "The name of the bank of the beneficiary"
                    },
                    "address_of_bank": {
                        "type": "string",
                        "description": "The address of the bank of the beneficiary"
                    },
                    "iban": {
                        "type": "string",
                        "description": "The IBAN of the beneficiary"
                    },
                    "bic": {
                        "type": "string",
                        "description": "The BIC/SWIFT code for the SEPA transfer"
                    }
                },
                "required": ["reference", "beneficiary", "name_of_bank", "address_of_bank", "iban", "bic"]
            }
        },
        "required": ["contract_id", "deposit_address", "details"]
    }
    ```

### Create viviswap address detail

| Method | Arguments | Returns | Errors | Dependencies | Level | Repeat|
|--------|-----------|---------|--------|--------------|-------|-------|
| Create address detail for viviswap user | | Returns `ViviswapAddressDetail`, if successful otherwise `Error` |  Returns `RepositoryInitError` - If the repository initialization fails, `ViviswapMissingUserError` - If the viviswap user is missing, `ViviswapInvalidStateError` - If the viviswap state is invalid, `ViviswapApiError` - If there is an error with the Viviswap API  | [Wallet initialization](./SDK%20API%20Reference.md#initialize-a-wallet), [Get KYC Details for viviswap](./SDK%20API%20Reference.md#get-kyc-details-for-viviswap) | Usage | Application |

=== "Rust"

    ```rust
    async fn create_detail_for_viviswap(&self) -> Result<ViviswapAddressDetail, crate::error::Error>
    ```

=== "Java"

    ```java
    public String createViviswapDetail() throws Exception
    ```
=== "Swift"

    ```swift
    public func createViviswapDetail() throws -> String
    ```

### Create withdrawal with viviswap

| Method | Arguments | Returns | Errors | Dependencies | Level | Repeat|
|--------|-----------|---------|--------|--------------|-------|-------|
| Create address detail for viviswap user | `amount` - The amount of the withdrawal, `pin` - The optional PIN for verification | Returns `ViviswapWithdrawal`, if successful otherwise `Error` |  Returns `RepositoryInitError` - If the repository initialization fails, `ViviswapMissingUserError` - If the viviswap user is missing, `ViviswapInvalidStateError` - If the viviswap state is invalid, `ViviswapApiError` - If there is an error with the Viviswap API  | [Wallet initialization](./SDK%20API%20Reference.md#initialize-a-wallet), [Get KYC Details for viviswap](./SDK%20API%20Reference.md#get-kyc-details-for-viviswap), [Update IBAN](./SDK%20API%20Reference.md#update-iban-for-viviswap) | Usage | Application |

=== "Rust"

    ```rust
    async fn create_withdrawal_with_viviswap(
        &self,
        amount: f32,
        pin: Option<String>,
    ) -> Result<ViviswapWithdrawal, crate::error::Error>
    ```

=== "Java"

    ```java
    public String withdrawWithViviswap(float amount, String pin) throws Exception
    ```
=== "Swift"

    ```swift
    public func withdrawWithViviswap(amount: Float, pin: String) throws -> String 
    ```

#### ViviswapWithdrawal

=== "Rust"

    ```Rust
    pub struct ViviswapWithdrawal {
        /// The unique UUID to track the withdrawal contract
        pub contract_id: String,
        /// The deposit address, in this case the IBAN of the user, where fiat will be deposited.
        pub deposit_address: String,
        /// The details of the withdrawal
        pub details: ViviswapWithdrawalDetails,
    }
    pub struct ViviswapWithdrawalDetails {
        /// The reference used by viviswap for the SEPA transfer
        pub reference: String,
        /// The id of the unique wallet internal to viviswap
        pub wallet_id: String,
        /// The crypto address of viviswap where the crypto swap is to be sent
        pub crypto_address: String,
    }
    ```

=== "JSON"

    ```json
    {
        "$schema": "http://json-schema.org/draft-07/schema#",
        "type": "object",
        "properties": {
            "contract_id": {
                "type": "string",
                "description": "The unique UUID of the contract"
            },
            "withdrawal_address": {
                "type": "string",
                "description": "The withdrawal address (crypto) where the swap will put the funds from fiat"
            },
            "details": {
                "type": "object",
                "description": "The details of the withdrawal (for the user)",
                "properties": {
                    "reference": {
                        "type": "string",
                        "description": "The reference used by viviswap for the SEPA transfer"
                    },
                    "wallet_id": {
                        "type": "string",
                        "description": "The id of the unique wallet internal to viviswap"
                    },
                    "crypto_address": {
                        "type": "string",
                        "description": "The crypto address of viviswap where the crypto swap is to be sent"
                    }
                },
                "required": ["reference", "wallet_id", "crypto_address"]
            }
        },
        "required": ["contract_id", "withdrawal_address", "details"]
    }
    ```

### Get swap details

| Method | Arguments | Returns | Errors | Dependencies | Level | Repeat|
|--------|-----------|---------|--------|--------------|-------|-------|
| Get swap details | `order_id` - The ID of the swap order. | Returns `Order` containing the swap order details or an error. |  Returns `RepositoryInitError` - If the repository initialization fails, `ViviswapMissingUserError` - If the viviswap user is missing, `ViviswapApiError` - If there is an error with the Viviswap API  | [Wallet initialization](./SDK%20API%20Reference.md#initialize-a-wallet), [Get KYC Details for viviswap](./SDK%20API%20Reference.md#get-kyc-details-for-viviswap) | Usage | Application |

=== "Rust"

    ```rust
    async fn get_swap_details(&self, order_id: String) -> Result<Order, crate::error::Error>
    ```

=== "Java"

    ```java
    public String getSwapDetails(String order_id) throws Exception
    ```
=== "Swift"

    ```swift
    public func getSwapDetails(order_id: String) throws -> String
    ```

#### Order

=== "Rust"

    ```Rust
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
        pub refund_payment_method_id: Option<String>,
        pub status: i32,
        pub creation_date: String,
        pub incoming_payment_detail: Option<PaymentDetail>,
        pub outgoing_payment_detail: Option<PaymentDetail>,
        pub refund_payment_detail: Option<PaymentDetail>,
    }

    pub struct PaymentDetail {
        pub id: String,
        pub address: String,
        pub is_verified: Option<bool>,
    }
    ```

=== "JSON"

    ```json
    {
        "$schema": "http://json-schema.org/draft-07/schema#",
        "type": "object",
        "properties": {
            "id": {
                "type": "string",
                "description": "The unique id of the order"
            },
            "is_payed_out": {
                "type": "boolean",
                "description": "Whether the order is payed out"
            },
            "is_approved": {
                "type": "boolean",
                "description": "Whether the order is approved"
            },
            "is_canceled": {
                "type": "boolean",
                "description": "Whether the order is canceled"
            },
            "fees_amount_eur": {
                "type": "number",
                "description": "The amount of fees in EUR"
            },
            "crypto_fees": {
                "type": "number",
                "description": "The amount of crypto fees"
            },
            "contract_id": {
                "type": "string",
                "description": "The id of the contract"
            },
            "incoming_payment_method_id": {
                "type": "string",
                "description": "The id of the incoming payment method"
            },
            "incoming_payment_method_currency": {
                "type": "string",
                "description": "The currency of the incoming payment method"
            },
            "incoming_amount": {
                "type": "number",
                "description": "The amount of the incoming payment"
            },
            "incoming_course": {
                "type": "number",
                "description": "The course of the incoming payment"
            },
            "outgoing_payment_method_id": {
                "type": "string",
                "description": "The id of the outgoing payment method"
            },
            "outgoing_payment_method_currency": {
                "type": "string",
                "description": "The currency of the outgoing payment method"
            },
            "outgoing_amount": {
                "type": "number",
                "description": "The amount of the outgoing payment"
            },
            "outgoing_course": {
                "type": "number",
                "description": "The course of the outgoing payment"
            },
            "refund_amount": {
                "type": ["number", "null"],
                "description": "The amount of the refund"
            },
            "refund_course": {
                "type": ["number", "null"],
                "description": "The course of the refund"
            },
            "refund_payment_method_id": {
                "type": ["string", "null"],
                "description": "The id of the refund payment method"
            },
            "status": {
                "type": "integer",
                "description": "The status of the order"
            },
            "creation_date": {
                "type": "string",
                "description": "The creation date of the order"
            },
            "incoming_payment_detail": {
                "type": ["object", "null"],
                "description": "The details of the incoming payment",
                "properties": {
                    "id": {
                        "type": "string",
                        "description": "The unique id of the payment detail"
                    },
                    "address": {
                        "type": "string",
                        "description": "The address of the payment detail"
                    }
                },
                "required": ["id", "address"]
            },
            "outgoing_payment_detail": {
                "type": ["object", "null"],
                "description": "The details of the outgoing payment",
                "properties": {
                    "id": {
                        "type": "string",
                        "description": "The unique id of the payment detail"
                    },
                    "address": {
                        "type": "string",
                        "description": "The address of the payment detail"
                    }
                },
                "required": ["id", "address"]
            },
            "refund_payment_detail": {
                "type": ["object", "null"],
                "description": "The details of the refund payment",
                "properties": {
                    "id": {
                        "type": "string",
                        "description": "The unique id of the payment detail"
                    },
                    "address": {
                        "type": "string",
                        "description": "The address of the payment detail"
                    }
                },
                "required": ["id", "address"]
            }
        },
        "required": ["id", "is_payed_out", "is_approved", "is_canceled", "fees_amount_eur", "crypto_fees", "contract_id", "incoming_payment_method_id", "incoming_payment_method_currency", "incoming_amount", "incoming_course", "outgoing_payment_method_id", "outgoing_payment_method_currency", "outgoing_amount", "outgoing_course", "status", "creation_date"]
    }
    ```

### Get swap lists

| Method | Arguments | Returns | Errors | Dependencies | Level | Repeat|
|--------|-----------|---------|--------|--------------|-------|-------|
| Get swap list | `start` - The start page parameter, `limit` - The pagination limit parameter | Returns an array of `Order` containing the swap order details for each swap or an error. |  Returns `RepositoryInitError` - If the repository initialization fails, `ViviswapMissingUserError` - If the viviswap user is missing, `ViviswapApiError` - If there is an error with the Viviswap API  | [Wallet initialization](./SDK%20API%20Reference.md#initialize-a-wallet), [Get KYC Details for viviswap](./SDK%20API%20Reference.md#get-kyc-details-for-viviswap) | Usage | Application |

=== "Rust"

    ```rust
    async fn get_swap_list(&self, start: u32, limit: u32) -> Result<OrderList, crate::error::Error>
    ```

=== "Java"

    ```java
    public String getSwapList(long page, long limit) throws Exception
    ```
=== "Swift"

    ```swift
    public func getSwapList(page: UInt64, limit: UInt64) throws -> String
    ```

### Get exchange rate

| Method | Arguments | Returns | Errors | Dependencies | Level | Repeat|
|--------|-----------|---------|--------|--------------|-------|-------|
| The exchange rate for the selected currency | | Returns the exchange rate |  Returns `ViviswapApiError` - If there is an error with the Viviswap API  | | Usage | Application |

=== "Rust"

    ```rust
    async fn get_exchange_rate(&self) -> Result<f32, crate::error::Error>
    ```

=== "Java"

    ```java
    public float getExchangeRate() throws Exception
    ```
=== "Swift"

    ```swift
    public func getExchangeRate() throws -> Float
    ```

## Transaction functions

### Create purchase request

| Method | Arguments | Returns | Errors | Dependencies | Level | Repeat|
|--------|-----------|---------|--------|--------------|-------|-------|
| Create purchase request| `receiver` - The receiver's username, `amount` - The amount of the purchase, `product_hash` - The hash of the product, `app_data` - The application data, `purchase_type` - The type of the purchase | The purchase ID. This is an internal index used to reference the transaction in Cawaena |  Returns an error if the user or wallet is not initialized, or if there is an error creating the transaction  | [Wallet initialization](./SDK%20API%20Reference.md#initialize-a-wallet) | Usage | Application |

=== "Rust"

    ```rust
    async fn create_purchase_request(
        &self,
        receiver: &str,
        amount: f64,
        product_hash: &str,
        app_data: &str,
        purchase_type: &str,
    ) -> Result<String, crate::error::Error>
    ```

=== "Java"

    ```java
    public String purchaseRequestCreate(String receiver, double amount, String product_hash, String app_data,
            String purchase_type) throws Exception 
    ```
=== "Swift"

    ```swift
    public func createPurchaseRequest(
        receiver: String, amount: Double, product_hash: String, app_data: String, purchase_type: String
    ) throws -> String
    ```

### Get purchase status

| Method | Arguments | Returns | Errors | Dependencies | Level | Repeat|
|--------|-----------|---------|--------|--------------|-------|-------|
| Get purchase status |`purchase_id` - The ID of the purchase. | The purchase status as `TxStatus` object |  Returns an error if the user or wallet is not initialized, or if there is an error getting the transaction status  | [Wallet initialization](./SDK%20API%20Reference.md#initialize-a-wallet) | Usage | Application |

=== "Rust"

    ```rust
    async fn get_purchase_status(&self, purchase_id: &str) -> Result<TxStatus, crate::error::Error>
    ```

=== "Java"

    ```java
    public String purchaseStatus(String purchase_id) throws Exception
    ```
=== "Swift"

    ```swift
    public func getPurchaseStatus(purchase_id: String) throws -> String
    ```

#### TxStatus

=== "Rust"

    ```Rust
    pub enum TxStatus {
        #[default]
        Pending,
        Valid,
        Invalid,
        ProcessingMain,
        ProcessingAux,
        Completed,
        Failed,
    }
    ```

=== "JSON"

    ```json
    {
        "$schema": "http://json-schema.org/draft-07/schema#",
        "type": "string",
        "description": "The status of the transaction",
        "enum": ["Pending", "Valid", "Invalid", "ProcessingMain", "ProcessingAux", "Completed", "Failed"]
    }
    ```

### Get purchase details

| Method | Arguments | Returns | Errors | Dependencies | Level | Repeat|
|--------|-----------|---------|--------|--------------|-------|-------|
| Get purchase details |`purchase_id` - The ID of the purchase. | The purchase status as `PurchaseDetails` object |  Returns an error if the user or wallet is not initialized, or if there is an error getting the transaction status  | [Wallet initialization](./SDK%20API%20Reference.md#initialize-a-wallet) | Usage | Application |

=== "Rust"

    ```rust
    async fn get_purchase_details(&self, purchase_id: &str) -> Result<PurchaseDetails, crate::error::Error>
    ```

=== "Java"

    ```java
    public String purchaseDetails(String purchase_id) throws Exception
    ```
=== "Swift"

    ```swift
    public func getPurchaseDetails(purchase_id: String) throws -> String
    ```

#### PurchaseDetails

=== "Rust"

    ```Rust
    pub struct PurchaseDetails {
        /// The main address where the fees goes to.
        pub main_address: String,
        /// The auxiliary address where the amount minus fees goes.
        pub aux_address: String,
        /// The amount to be paid.
        pub amount: f64,
        /// The fees to be paid.
        pub fees: f64,
    }
    ```

=== "JSON"

    ```json
    {
        "$schema": "http://json-schema.org/draft-07/schema#",
        "type": "object",
        "properties": {
            "main_address": {
                "type": "string",
                "description": "The main address where the fees goes to"
            },
            "aux_address": {
                "type": "string",
                "description": "The auxiliary address where the amount minus fees goes"
            },
            "amount": {
                "type": "number",
                "description": "The amount to be paid"
            },
            "fees": {
                "type": "number",
                "description": "The fees to be paid"
            }
        },
        "required": ["main_address", "aux_address", "amount", "fees"]
    }
    ```

### Confirm purchase request

| Method | Arguments | Returns | Errors | Dependencies | Level | Repeat|
|--------|-----------|---------|--------|--------------|-------|-------|
| Confirm purchase request |`pin` - The PIN of the user, `purchase_id` - The ID of the purchase. | Returns, if the purchase request is confirmed successfully. |  Returns an error if the user or wallet is not initialized, if there is an error verifying the PIN, if there is an error getting the transaction details, or if there is an error committing the transaction.  | [Wallet initialization](./SDK%20API%20Reference.md#initialize-a-wallet) | Usage | Application |

=== "Rust"

    ```rust
    async fn confirm_purchase_request(&self, pin: &str, purchase_id: &str) -> Result<(), crate::error::Error>
    ```

=== "Java"

    ```java
    public void purchaseRequestConfirm(String pin, String purchase_id) throws Exception
    ```
=== "Swift"

    ```swift
    public func confirmPurchaseRequest(pin: String, purchase_id: String) throws
    ```

### Send amount

| Method | Arguments | Returns | Errors | Dependencies | Level | Repeat|
|--------|-----------|---------|--------|--------------|-------|-------|
| Confirm purchase request |`pin` - The PIN of the user, `address` - The receiver's address, `amount` - The amount to send | Returns, if the amount is sent successfully. |  Returns an error if the user or wallet is not initialized, if there is an error verifying the PIN, or if there is an error sending the amount  | [Wallet initialization](./SDK%20API%20Reference.md#initialize-a-wallet) | Usage | Application |

=== "Rust"

    ```rust
    async fn send_amount(&self, pin: &str, address: &str, amount: f64) -> Result<(), crate::error::Error>
    ```

=== "Java"

    ```java
    public void transferAmount(String pin, String address, double amount) throws Exception
    ```
=== "Swift"

    ```swift
    public func sendAmount(pin: String, address: String, amount: Double) throws
    ```

### Get Purchase list

| Method | Arguments | Returns | Errors | Dependencies | Level | Repeat|
|--------|-----------|---------|--------|--------------|-------|-------|
| Get purchase list | `page` - The page number, `limit` - The maximum number of transactions per page | Returns a list of purchases as `TxInfo` object, if successful |  Returns an error if there is a problem getting the list of purchases.  | [User initialization](./SDK%20API%20Reference.md#initializing-a-user) | Usage | Application |

=== "Rust"

    ```rust
    async fn get_tx_list(&self, page: u32, limit: u32) -> Result<TxList, crate::error::Error>
    ```

=== "Java"

    ```java
    public String txList(long page, long limit) throws Exception 
    ```
=== "Swift"

    ```swift
    public func getTxList(page: UInt64, limit: UInt64) throws -> String
    ```

#### TxInfo

=== "Rust"

    ```Rust
    pub struct TxInfo {
        /// Tx creation date, if available
        pub date: Option<String>,
        /// receiver of the transaction
        pub receiver: String,
        /// Cawaena reference id for the transaction
        pub reference_id: String,
        /// Application specific metadata attached to the tx
        pub application_metadata: Option<ApplicationMetadata>,
        /// Amount of transfer
        pub amount: f64,
        /// Currency of transfer
        pub currency: String,
        /// Status of the transfer
        pub status: TxStatus,
        /// The transaction hash on the network
        pub transaction_hash: Option<String>,
        /// Exchange rate
        pub course: f64,
    }

    pub struct ApplicationMetadata {
        pub product_hash: String,
        pub reason: String,
        pub purchase_model: String,
        pub app_data: String,
    }

    ```
=== "JSON"

    ```json
    {
        "$schema": "http://json-schema.org/draft-07/schema#",
        "type": "object",
        "properties": {
            "date": {
                "type": ["string", "null"],
                "description": "Tx creation date, if available"
            },
            "receiver": {
                "type": "string",
                "description": "receiver of the transaction"
            },
            "reference_id": {
                "type": "string",
                "description": "Cawaena reference id for the transaction"
            },
            "application_metadata": {
                "type": ["object", "null"],
                "description": "Application specific metadata attached to the tx",
                "properties": {
                    "product_hash": {
                        "type": "string",
                        "description": "The product hash"
                    },
                    "reason": {
                        "type": "string",
                        "description": "The reason for the transaction"
                    },
                    "purchase_model": {
                        "type": "string",
                        "description": "The purchase model"
                    },
                    "app_data": {
                        "type": "string",
                        "description": "The application data"
                    }
                },
                "required": ["product_hash", "reason", "purchase_model", "app_data"]
            },
            "amount": {
                "type": "number",
                "description": "Amount of transfer"
            },
            "currency": {
                "type": "string",
                "description": "Currency of transfer"
            },
            "status": {
                "type": "string",
                "description": "Status of the transfer",
                "enum": ["Pending", "Valid", "Invalid", "ProcessingMain", "ProcessingAux", "Completed", "Failed"]
            },
            "transaction_hash": {
                "type": ["string", "null"],
                "description": "The transaction hash on the network"
            },
            "course": {
                "type": "number",
                "description": "Exchange rate"
            }
        },
        "required": ["receiver", "reference_id", "amount", "currency", "status", "course"]
    }
    ```

## Postident functions

### Start kyc verification for postident

| Method | Arguments | Returns | Errors | Dependencies | Level | Repeat|
|--------|-----------|---------|--------|--------------|-------|-------|
| Start kyc verification for postident | | Returns an object `NewCaseIdResponse` if successful, or an `Error` if an error occurs. |  `RepositoryInitError` if the repository fails to initialize, `UserInitError` if the user fails to initialize, `UserAlreadyKycVerifiedError` if the user is already KYC verified.  | [User initialization](./SDK%20API%20Reference.md#initializing-a-user) | Usage | Application |

=== "Rust"

    ```rust
    async fn start_kyc_verification_for_postident(&mut self) -> Result<NewCaseIdResponse, crate::error::Error>
    ```

=== "Java"

    ```java
    public String startKycVerificationForPostident() throws Exception
    ```
=== "Swift"

    ```swift
    public func startKycVerificationForPostident() throws -> String
    ```

#### NewCaseIdResponse

=== "Rust"

    ```Rust
    pub struct NewCaseIdResponse {
        /// New Postident case id
        pub case_id: String,
        /// Case url
        pub case_url: String,
    }
    ```
=== "JSON"

    ```json
    {
        "$schema": "http://json-schema.org/draft-07/schema#",
        "type": "object",
        "properties": {
            "case_id": {
                "type": "string",
                "description": "New Postident case id"
            },
            "case_url": {
                "type": "string",
                "description": "Case url"
            }
        },
        "required": ["case_id", "case_url"]
    }
    ```

### Get case details for postident

| Method | Arguments | Returns | Errors | Dependencies | Level | Repeat|
|--------|-----------|---------|--------|--------------|-------|-------|
| Get case details for postident| | Returns an object `CaseDetailsResponse` if successful, or an `Error` if an error occurs. |   UserInitError` if the user fails to initialize  | [User initialization](./SDK%20API%20Reference.md#initializing-a-user) | Usage | Application |

=== "Rust"

    ```rust
    async fn get_kyc_details_for_postident(&self) -> Result<CaseDetailsResponse, crate::error::Error>
    ```

=== "Java"

    ```java
    public String getKycDetailsForPostident() throws Exception
    ```
=== "Swift"

    ```swift
    public func getKycDetailsForPostident() throws -> String
    ```

#### CaseDetailsResponse

=== "Rust"

    ```Rust
    pub struct CaseDetailsResponse {
        pub case_id: String,
        pub archived: bool,
        pub status: String,
    }
    ```
=== "JSON"

    ```json
        {
            "$schema": "http://json-schema.org/draft-07/schema#",
            "type": "object",
            "properties": {
                "case_id": {
                    "type": "string",
                    "description": "The Postident case id"
                },
                "archived": {
                    "type": "boolean",
                    "description": "Whether the case is archived"
                },
                "status": {
                    "type": "string",
                    "description": "The status of the case"
                }
            },
            "required": ["case_id", "archived", "status"]
        }
    ```

### Update case status for postident

| Method | Arguments | Returns | Errors | Dependencies | Level | Repeat|
|--------|-----------|---------|--------|--------------|-------|-------|
| Update case status for postident| `case_id`: The ID of the case to update. | Returns,` if successful, or an `Error` if an error occurs. | UserInitError` if the user fails to initialize  | [User initialization](./SDK%20API%20Reference.md#initializing-a-user) | Usage | Application |

=== "Rust"

    ```rust
    async fn update_kyc_status_for_postident(&self, case_id: &str) -> Result<(), crate::error::Error>
    ```

=== "Java"

    ```java
    public void updateKycStatusForPostident(String case_id) throws Exception
    ```
=== "Swift"

    ```swift
    public func updateKycStatusForPostident(case_id: String) throws
    ```

## Billing account functions

### Create a new billing customer account

| Method | Arguments | Returns | Errors | Dependencies | Level | Repeat|
|--------|-----------|---------|--------|--------------|-------|-------|
| Create a new account| `country_code` - The country code for the customer. | Returns,` if successful, or an `Error` if an error occurs. | UserInitError` if the user fails to initialize  | [User initialization](./SDK%20API%20Reference.md#initializing-a-user) | Usage | User |

=== "Rust"

    ```rust
    async fn create_customer(&self, country_code: &str) -> Result<(), crate::error::Error> 
    ```

=== "Java"

    ```java
    public void customerCreate(String country_code) throws Exception
    ```
=== "Swift"

    ```swift
    public func createCustomer(country_code: String) throws
    ```

### Get account status

| Method | Arguments | Returns | Errors | Dependencies | Level | Repeat|
|--------|-----------|---------|--------|--------------|-------|-------|
| Fetches the customer account and updates it internally | | Returns,` if successful, or an `Error` if an error occurs. | UserInitError` if the user fails to initialize, or if there is an issue retrieving the customer account details.  | [User initialization](./SDK%20API%20Reference.md#initializing-a-user) | Usage | Application |

=== "Rust"

    ```rust
    async fn get_customer(&mut self) -> Result<(), crate::error::Error> 
    ```

=== "Java"

    ```java
    public void customerGet() throws Exception
    ```
=== "Swift"

    ```swift
    public func getCustomer() throws 
    ```
