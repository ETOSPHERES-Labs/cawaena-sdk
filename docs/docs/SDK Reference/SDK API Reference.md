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

The API reference for the JS/TS bindings are available [here](../jstsdocs/classes/ETOPaySdk.html). Please consult the tables below for the dependencies between each function.

## Java

The Javadoc API reference for the Java bindings is available [here](../javadoc/com/etospheres/etopay/ETOPaySdk.html). Please consult the tables below for the dependencies between each function.

## Rust

The Rustdoc API reference is available [here](../rust-docs/doc/etopay_sdk/index.html). Please consult the tables below for the dependencies between each function.


## SDK Initialization and Configuration

### Instantiating the SDK

| Method | Arguments | Returns | Dependencies | Level | Repeat|
|--------|-----------|---------|--------------|-------|-------|
| Constructor | | Returns an `Error` if there is an issue in loading the dynamically or statically linked binary shared library, otherwise the handle to the SDK | | Basic | Handle |

=== "Rust"
    [constructor](../rust-docs/doc/etopay_sdk/core/struct.Sdk.html#method.new)

=== "Java"
    [constructor](../javadoc/com/etospheres/etopay/ETOPaySdk.html#<init>())

=== "Typescript"
    [constructor](../jstsdocs/classes/ETOPaySdk.html#constructor)

=== "Swift"
    Not available yet!

    ```swift
    import ETOPaySdk
    let sdk = ETOPaySdk()
    ```

### Set configuration

| Method | Arguments | Returns | Dependencies | Level | Repeat|
|--------|-----------|---------|--------------|-------|-------|
| Set the SDK configuration parameters. | `config` - The JSON formatted string containing the configuration parameters. See [SDK Configuration](../SDK%20Configuration/Configuration.md) for more information of the available options. | | [Constructor](./SDK%20API%20Reference.md#instantiating-the-sdk) | Basic | Handle |

=== "Rust"
    [set_config](../rust-docs/doc/etopay_sdk/core/struct.Sdk.html#method.set_config)

=== "Java"
    [setConfig](../javadoc/com/etospheres/etopay/ETOPaySdk.html#setConfig(java.lang.String))

=== "Typescript"
    [setConfig](../jstsdocs/classes/ETOPaySdk.html#setConfig)

=== "Swift"
    Not available yet!

    ```swift
    public func setConfig(config: String) throws
    ```

### Get supported networks

| Method | Arguments | Returns | Dependencies | Level | Repeat|
|--------|-----------|---------|--------------|-------|-------|
| Get the supported networks. | | Returns a list of ApiNetwork. | | [Constructor](./SDK%20API%20Reference.md#instantiating-the-sdk), [Set Configuration](./SDK%20API%20Reference.md#set-configuration), [Refresh access token](./SDK%20API%20Reference.md#refreshing-access-token), [User initialization](./SDK%20API%20Reference.md#initializing-a-user) | Basic | Handle |

=== "Rust"
    [getNetworks](../rust-docs/doc/etopay_sdk/core/struct.Sdk.html#method.get_networks)

=== "Java"
    [getNetworks](../javadoc/com/etospheres/etopay/ETOPaySdk.html#get_networks())

=== "Typescript"
    [getNetworks](../jstsdocs/classes/ETOPaySdk.html#get_networks)

=== "Swift"
    Not available yet!

    ```swift
    public func getNetworks() throws -> String
    ```

### Set network

| Method | Arguments | Returns | Dependencies | Level | Repeat|
|--------|-----------|---------|--------------|-------|-------|
| Sets the network | `network_id` - The network_id as string.| | [Constructor](./SDK%20API%20Reference.md#instantiating-the-sdk) | Basic | Handle |

=== "Rust"
    [set_network](../rust-docs/doc/etopay_sdk/core/struct.Sdk.html#method.set_network)

=== "Java"
    [setNetwork](../javadoc/com/etospheres/etopay/ETOPaySdk.html#setNetwork(java.lang.String))

=== "Typescript"
    [setNetwork](../jstsdocs/classes/ETOPaySdk.html#setNetwork)

=== "Swift"
    Not available yet!

    ```swift
    public func setNetwork(network_id: String) throws 
    ```

### Get build information 

| Method | Arguments | Returns | Dependencies | Level | Repeat|
|--------|-----------|---------|--------------|-------|-------|
| Get the SDK build information | | Returns a multi-line String containing: `Branch name` (e.g. main), `Commit hash` (e.g. 92cedead), `Build time` (e.g. 2024-10-29 12:10:09 +00:00), `Rust version` (e.g. 1.80.1 3f5fd8dd4 2024-08-06), `Toolchain channel` (e.g. stable-x86_64-unknown-linux-gnu) | Usage | Application |

=== "Rust"
    [get_build_info](../rust-docs/doc/etopay_sdk/core/struct.Sdk.html#method.get_build_info)

=== "Java"
    [getBuildInfo](../javadoc/com/etospheres/etopay/ETOPaySdk.html#getBuildInfo())

=== "Typescript"
    [getBuildInfo](../jstsdocs/classes/ETOPaySdk.html#getBuildInfo)

=== "Swift"
    Not available yet!

    ```swift
    public func getBuildInfo() throws -> String
    ```

## User functions

### Creating a new user

| Method | Arguments | Returns | Dependencies | Level | Repeat|
|--------|-----------|---------|--------------|-------|-------|
| Create a new user | `username` - The username of the new user. | | [Constructor](./SDK%20API%20Reference.md#instantiating-the-sdk), [Set Configuration](./SDK%20API%20Reference.md#set-configuration) | Basic | User |

=== "Rust"
    [create_new_user](../rust-docs/doc/etopay_sdk/core/struct.Sdk.html#method.create_new_user)

=== "Java"
    [createNewUser](../javadoc/com/etospheres/etopay/ETOPaySdk.html#createNewUser(java.lang.String))

=== "Typescript"
    [createNewUser](../jstsdocs/classes/ETOPaySdk.html#createNewUser)

=== "Swift"
    Not available yet!

    ```swift
    public func createNewUser(username: String) throws
    ```

### Initializing a user

| Method | Arguments | Returns | Dependencies | Level | Repeat|
|--------|-----------|---------|--------------|-------|-------|
| Initialize a user | `username` - The username of the user to initialize. | | [Constructor](./SDK%20API%20Reference.md#instantiating-the-sdk), [Set Configuration](./SDK%20API%20Reference.md#set-configuration), [Refresh access token](./SDK%20API%20Reference.md#refreshing-access-token), [Create new user](./SDK%20API%20Reference.md#creating-a-new-user) | Usage | Application |

=== "Rust"
    [init_user](../rust-docs/doc/etopay_sdk/core/struct.Sdk.html#method.init_user)

=== "Java"
    [initializeUser](../javadoc/com/etospheres/etopay/ETOPaySdk.html#initializeUser(java.lang.String))

=== "Typescript"
    [initializeUser](../jstsdocs/classes/ETOPaySdk.html#initializeUser)

=== "Swift"
    Not available yet!

    ```swift
    public func initUser(username: String) throws 
    ```

### Refreshing access token

| Method | Arguments | Returns | Dependencies | Level | Repeat|
|--------|-----------|---------|--------------|-------|-------|
| Refresh access token | `access_token` - The new access token to be set. | | [Constructor](./SDK%20API%20Reference.md#instantiating-the-sdk), [Set Configuration](./SDK%20API%20Reference.md#set-configuration)| Basic | Application |

=== "Rust"
    [refresh_access_token](../rust-docs/doc/etopay_sdk/core/struct.Sdk.html#method.refresh_access_token)

=== "Java"
    [refreshAccessToken](../javadoc/com/etospheres/etopay/ETOPaySdk.html#refreshAccessToken(java.lang.String))

=== "Typescript"
    [refreshAccessToken](../jstsdocs/classes/ETOPaySdk.html#refreshAccessToken)

=== "Swift"
    Not available yet!

    ```swift
    public func refreshAccessToken(access_token: String) throws 
    ```

### Checking KYC status

| Method | Arguments | Returns | Dependencies | Level | Repeat|
|--------|-----------|---------|--------------|-------|-------|
| Check if the user's KYC status is verified | `username` - The username of the user to check KYC status for. | Returns `true` if the KYC status is verified, or `false` if it is not verified. | [User initialization](./SDK%20API%20Reference.md#initializing-a-user) | Usage | Application |

=== "Rust"
    [is_kyc_status_verified](../rust-docs/doc/etopay_sdk/core/struct.Sdk.html#method.is_kyc_status_verified)

=== "Java"
    [isKycVerified](../javadoc/com/etospheres/etopay/ETOPaySdk.html#isKycVerified(java.lang.String))

=== "Typescript"
    [isKycVerified](../jstsdocs/classes/ETOPaySdk.html#isKycVerified)

=== "Swift"
    Not available yet!

    ```swift
    public func isKycVerified(username: String) throws
    -> Bool
    ```

### Delete user

| Method | Arguments | Returns | Dependencies | Level | Repeat|
|--------|-----------|---------|--------------|-------|-------|
| Delete the currently active user and their wallet | `pin` - The PIN of the user to be deleted. Required only if the user has created a wallet. | | [User initialization](./SDK%20API%20Reference.md#initializing-a-user) | Usage | Application |

=== "Rust"
    [delete_user](../rust-docs/doc/etopay_sdk/core/struct.Sdk.html#method.delete_user)

=== "Java"
    [deleteUser](../javadoc/com/etospheres/etopay/ETOPaySdk.html#deleteUser(java.lang.String))

=== "Typescript"
    [deleteUser](../jstsdocs/classes/ETOPaySdk.html#deleteUser)

=== "Swift"
    Not available yet!

    ```swift
    public func deleteUser(pin: String) throws
    ```

## Wallet functions

### Create new wallet

| Method | Arguments | Returns | Dependencies | Level | Repeat|
|--------|-----------|---------|--------------|-------|-------|
| Creates a new wallet for the user with the specified PIN and password | `pin` - The PIN for the wallet | Returns the mnemonic phrase of the newly created wallet if successful. | [User initialization](./SDK%20API%20Reference.md#initializing-a-user) | Usage | User |

=== "Rust"
    [create_wallet_from_new_mnemonic](../rust-docs/doc/etopay_sdk/core/struct.Sdk.html#method.create_wallet_from_new_mnemonic)

=== "Java"
    [createNewWallet](../javadoc/com/etospheres/etopay/ETOPaySdk.html#createNewWallet(java.lang.String))

=== "Typescript"
    [createNewWallet](../jstsdocs/classes/ETOPaySdk.html#createNewWallet)

=== "Swift"
    Not available yet!

    ```swift
    public func createNewWallet(pin: String) throws -> String
    ```

### Create new wallet from mnemonic

| Method | Arguments | Returns | Dependencies | Level | Repeat|
|--------|-----------|---------|--------------|-------|-------|
| Create a wallet from existing mnemonic | `pin` - The PIN for the wallet, `mnemonic` - The mnemonic to migrate from | | [User initialization](./SDK%20API%20Reference.md#initializing-a-user) | Usage | User |

=== "Rust"
    [create_wallet_from_existing_mnemonic](../rust-docs/doc/etopay_sdk/core/struct.Sdk.html#method.create_wallet_from_existing_mnemonic)

=== "Java"
    [createWalletFromMnemonic](../javadoc/com/etospheres/etopay/ETOPaySdk.html#createWalletFromMnemonic(java.lang.String,java.lang.String))

=== "Typescript"
    [createWalletFromMnemonic](../jstsdocs/classes/ETOPaySdk.html#createWalletFromMnemonic)

=== "Swift"
    Not available yet!

    ```swift
    public func createWalletFromMnemonic(pin: String, mnemonic: String) throws
    ```

### Create new wallet from backup

| Method | Arguments | Returns | Dependencies | Level | Repeat|
|--------|-----------|---------|--------------|-------|-------|
| Create a wallet from existing backup | `pin` - The PIN for the wallet, `backup` - The bytes representing the backup file contents, `backup_password` - The password used when creating the backup | | [User initialization](./SDK%20API%20Reference.md#initializing-a-user) | Usage | Application |

=== "Rust"
    [create_wallet_from_backup](../rust-docs/doc/etopay_sdk/core/struct.Sdk.html#method.create_wallet_from_backup)

=== "Java"
    [createWalletFromBackup](../javadoc/com/etospheres/etopay/ETOPaySdk.html#createWalletFromBackup(java.lang.String,byte%5B%5D,java.lang.String))

=== "Typescript"
    [createWalletFromBackup](../jstsdocs/classes/ETOPaySdk.html#createWalletFromBackup)

=== "Swift"
    Not available yet!

    ```swift    
    public func restoreWalletFromBackup(pin: String, backup: RustVec<UInt8>, backup_password: String) throws
    ```

### Create a wallet backup

| Method | Arguments | Returns | Dependencies | Level | Repeat|
|--------|-----------|---------|--------------|-------|-------|
| Create a wallet backup | `backup_password` - The password for the backup | Returns the bytes of the created backup file if successful. | [User initialization](./SDK%20API%20Reference.md#initializing-a-user) | Usage | Application |

=== "Rust"
    [create_wallet_backup](../rust-docs/doc/etopay_sdk/core/struct.Sdk.html#method.create_wallet_backup)

=== "Java"
    [createWalletBackup](../javadoc/com/etospheres/etopay/ETOPaySdk.html#createWalletBackup(java.lang.String,java.lang.String))

=== "Typescript"
    [createWalletBackup](../jstsdocs/classes/ETOPaySdk.html#createWalletBackup)

=== "Swift"
    Not available yet!

    ```swift    
    public func createWalletBackup(backup_password: String) throws -> RustVec<UInt8>
    ```

### Verify mnemonic

| Method | Arguments | Returns | Dependencies | Level | Repeat|
|--------|-----------|---------|--------------|-------|-------|
| Verifies the mnemonic by checking if it matches the stored mnemonic | `pin` - The PIN for the wallet, `mnemonic` - The mnemonic to verify | Returns `true` or `false` whether the mnemonic is successfully verified. | [User initialization](./SDK%20API%20Reference.md#initializing-a-user) | Usage | User |

=== "Rust"
    [verify_mnemonic](../rust-docs/doc/etopay_sdk/core/struct.Sdk.html#method.verify_mnemonic)

=== "Java"
    [verifyMnemonic](../javadoc/com/etospheres/etopay/ETOPaySdk.html#verifyMnemonic(java.lang.String,java.lang.String))

=== "Typescript"
    [verifyMnemonic](../jstsdocs/classes/ETOPaySdk.html#verifyMnemonic)

=== "Swift"
    Not available yet!

    ```swift    
    public func verifyMnemonic(pin: String, mnemonic: String) throws -> Bool
    ```

### Delete wallet

!!! warning

    Deletes the currently active wallet, potentially resulting in loss of funds if the mnemonic or wallet is not backed up.

| Method | Arguments | Returns | Dependencies | Level | Repeat|
|--------|-----------|---------|--------------|-------|-------|
| Delete the currently active wallet | `pin` - The PIN for the wallet | | [Wallet initialization](./SDK%20API%20Reference.md#create-new-wallet) | Usage | Application |

=== "Rust"
    [delete_wallet](../rust-docs/doc/etopay_sdk/core/struct.Sdk.html#method.delete_wallet)

=== "Java"
    [deleteWallet](../javadoc/com/etospheres/etopay/ETOPaySdk.html#deleteWallet(java.lang.String))

=== "Typescript"
    [deleteWallet](../jstsdocs/classes/ETOPaySdk.html#deleteWallet)

=== "Swift"
    Not available yet!

    ```swift    
    public func deleteWallet(pin: String) throws
    ```

### Verify pin

| Method | Arguments | Returns | Dependencies | Level | Repeat|
|--------|-----------|---------|--------------|-------|-------|
| Verifies the pin for the wallet | `pin` - The pin to verify | | [Wallet initialization](./SDK%20API%20Reference.md#create-new-wallet) | Usage | Application |

=== "Rust"
    [verify_pin](../rust-docs/doc/etopay_sdk/core/struct.Sdk.html#method.verify_pin)

=== "Java"
    [pinVerify](../javadoc/com/etospheres/etopay/ETOPaySdk.html#pinVerify(java.lang.String))

=== "Typescript"
    [verifyPin](../jstsdocs/classes/ETOPaySdk.html#verifyPin)

=== "Swift"
    Not available yet!

    ```swift    
    public func verifyPin(pin: String) throws
    ```

### Reset pin

| Method | Arguments | Returns | Dependencies | Level | Repeat|
|--------|-----------|---------|--------------|-------|-------|
| Resets the pin for the wallet with a new pin by using the existing pin | `pin` - The current pin for the wallet, `new_pin` - The new pin to set for the wallet | | [Wallet initialization](./SDK%20API%20Reference.md#create-new-wallet) | Usage | Application |

=== "Rust"
    [change_pin](../rust-docs/doc/etopay_sdk/core/struct.Sdk.html#method.change_pin)

=== "Java"
    [pinReset](../javadoc/com/etospheres/etopay/ETOPaySdk.html#pinReset(java.lang.String,java.lang.String))

=== "Typescript"
    [resetPin](../jstsdocs/classes/ETOPaySdk.html#resetPin)

=== "Swift"
    Not available yet!

    ```swift    
    public func resetPin(pin: String, new_pin: String) throws 
    ```

### Set wallet password

| Method | Arguments | Returns | Dependencies | Level | Repeat|
|--------|-----------|---------|--------------|-------|-------|
| Set the password for the wallet using the provided pin and new password. If the password is already set, this changes it to the new password. Use [`is_wallet_password_set`](./SDK%20API%20Reference.md#is-wallet-password-set) to check if the password is already set. | `pin` - The new or existing PIN for the wallet, `new_password` - The new password to set for the wallet | [Wallet initialization](./SDK%20API%20Reference.md#create-new-wallet) | Usage | Application |

=== "Rust"
    [set_wallet_password](../rust-docs/doc/etopay_sdk/core/struct.Sdk.html#method.set_wallet_password)

=== "Java"
    [setWalletPassword](../javadoc/com/etospheres/etopay/ETOPaySdk.html#setWalletPassword(java.lang.String,java.lang.String))

=== "Typescript"
    [setWalletPassword](../jstsdocs/classes/ETOPaySdk.html#setWalletPassword)

=== "Swift"
    Not available yet!

    ```swift
    public func setWalletPassword(pin: String, new_password: String) throws
    ```

### Is wallet password set

| Method | Arguments | Returns | Dependencies | Level | Repeat|
|--------|-----------|---------|--------------|-------|-------|
| Check if the password is already set. Useful to prompt the user to setup one if it has not yet been done. See also [`set_wallet_password`](./SDK%20API%20Reference.md#set-wallet-password) for how to set a new password and change an existing password. | | Returns `true` or `false` whether or not the password is already set. | [Wallet initialization](./SDK%20API%20Reference.md#create-new-wallet) | Usage | Application |

=== "Rust"
    [is_wallet_password_set](../rust-docs/doc/etopay_sdk/core/struct.Sdk.html#method.is_wallet_password_set)

=== "Java"
    [isWalletPasswordSet](../javadoc/com/etospheres/etopay/ETOPaySdk.html#isWalletPasswordSet())

=== "Typescript"
    [isWalletPasswordSet](../jstsdocs/classes/ETOPaySdk.html#isWalletPasswordSet)

=== "Swift"
    Not available yet!

    ```swift
    public func isWalletPasswordSet() throws
    ```

### Generate a new address

| Method | Arguments | Returns | Dependencies | Level | Repeat|
|--------|-----------|---------|--------------|-------|-------|
| Generates a new receiver address for the wallet based on the selected network. | `pin` - The PIN for the wallet | Returns the generated address as a `String` if successful. | [Wallet initialization](./SDK%20API%20Reference.md#create-new-wallet), [Set network](./SDK%20API%20Reference.md#set-network) | Usage | Application |

=== "Rust"
    [generate_new_address](../rust-docs/doc/etopay_sdk/core/struct.Sdk.html#method.generate_new_address)

=== "Java"
    [generateNewAddress](../javadoc/com/etospheres/etopay/ETOPaySdk.html#generateNewAddress(java.lang.String))

=== "Typescript"
    [generateNewAddress](../jstsdocs/classes/ETOPaySdk.html#generateNewAddress)

=== "Swift"
    Not available yet!

    ```swift    
    public func generateNewAddress(pin: String) throws -> String
    ```

### Get balance

| Method | Arguments | Returns | Dependencies | Level | Repeat|
|--------|-----------|---------|--------------|-------|-------|
| Fetches the balance of the user from the wallet | `pin` - The PIN for the wallet | Returns the balance as a `f64` if successful. | [Wallet initialization](./SDK%20API%20Reference.md#create-new-wallet) | Usage | Application |

=== "Rust"
    [get_balance](../rust-docs/doc/etopay_sdk/core/struct.Sdk.html#method.get_balance)

=== "Java"
    [getWalletBalance](../javadoc/com/etospheres/etopay/ETOPaySdk.html#getWalletBalance(java.lang.String))

=== "Typescript"
    [getWalletBalance](../jstsdocs/classes/ETOPaySdk.html#getWalletBalance)

=== "Swift"
    Not available yet!

    ```swift    
    public func getBalance(pin: String) throws -> Float
    ```

### Get wallet transactions

| Method | Arguments | Returns | Dependencies | Level | Repeat|
|--------|-----------|---------|--------------|-------|-------|
| Wallet transaction list | `pin` - The PIN for the wallet, `start` - The starting page number for paginatation, `limit` - The page limit size for each page | Returns the list of transactions made on the wallet as an array of `WalletTxInfo` object or a serialized JSON of the same, if successful. | [Wallet initialization](./SDK%20API%20Reference.md#create-new-wallet) | Usage | Application |

=== "Rust"
    [get_wallet_tx_list](../rust-docs/doc/etopay_sdk/core/struct.Sdk.html#method.get_wallet_tx_list)

=== "Java"
    [getWalletTransactionList](../javadoc/com/etospheres/etopay/ETOPaySdk.html#getWalletTransactionList(java.lang.String,long,long))

=== "Typescript"
    [getWalletTransactionList](../jstsdocs/classes/ETOPaySdk.html#getWalletTransactionList)

=== "Swift"
    Not available yet!

    ```swift    
    public func getWalletTransactionList(pin: String, start: UInt64, limit: UInt64) throws -> Rustvec<WalletTxInfo>
    ```

#### WalletTxInfo

=== "Rust"

    ```Rust
    pub struct WalletTxInfo {
    /// Transaction creation date
    pub date: String,
    /// Contains block id
    pub block_id: Option<String>,
    /// Transaction id for particular transaction
    pub transaction_id: String,
    /// Describes type of transaction
    pub incoming: bool,
    /// Amount of transfer
    pub amount: f64,
    /// Name of the network [convert network_id to string based on the value]
    pub network: String,
    /// Status of the transfer
    pub status: String,
    /// Url of network
    pub explorer_url: Option<String>,
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
                "description": "Transaction creation date"
            },
            "block_id": {
                "type": ["string", "null"],
                "description": "Contains block id"
            },
            "transaction_id": {
                "type": "string",
                "description": "Transaction id for particular transaction"
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
                "description": "Name of the network [convert network_id to string based on the value]"
            },
            "status": {
                "type": "string",
                "description": "Status of the transfer"
            },
            "explorer_url": {
                "type": ["string", "null"],
                "description": "Url of network"
            }
        },
        "required": ["date", "transaction_id", "incoming", "amount", "network", "status"]
    }

    ```

### Get wallet transaction

| Method | Arguments | Returns | Dependencies | Level | Repeat|
|--------|-----------|---------|--------------|-------|-------|
| Single wallet transaction | `pin` - The PIN for the wallet, `tx_id` - The transaction id on the network | Returns the transactions made on the wallet with the given id as `WalletTxInfo` object or a serialized JSON of the same, if successful. | [Wallet initialization](./SDK%20API%20Reference.md#create-new-wallet) | Usage | Application |

=== "Rust"
    [get_wallet_tx](../rust-docs/doc/etopay_sdk/core/struct.Sdk.html#method.get_wallet_tx)

=== "Java"
    [getWalletTransaction](../javadoc/com/etospheres/etopay/ETOPaySdk.html#getWalletTransaction(java.lang.String,java.lang.String))

=== "Typescript"
    [getWalletTransaction](../jstsdocs/classes/ETOPaySdk.html#getWalletTransaction)

=== "Swift"
    Not available yet!

    ```swift    
    public func getWalletTransaction(pin: String, transactionId: String) throws -> WalletTxInfo
    ```

### Set recovery share 

| Method | Arguments | Returns | Dependencies | Level | Repeat|
|--------|-----------|---------|--------------|-------|-------|
| Sets the recovery share for the users wallet. | `share` - The recovery share to upload. | | [User initialization](./SDK%20API%20Reference.md#initializing-a-user) | Usage | Application |

=== "Rust"
    [set_recovery_share](../rust-docs/doc/etopay_sdk/core/struct.Sdk.html#method.set_recovery_share)

=== "Java"
    [setRecoveryShare](../javadoc/com/etospheres/etopay/ETOPaySdk.html#setRecoveryShare(java.lang.String))

=== "Typescript"
    [setRecoveryShare](../jstsdocs/classes/ETOPaySdk.html#setRecoveryShare)

=== "Swift"
    Not available yet!

    ```swift
    public func setRecoveryShare(share: String) throws
    ```

### Get recovery share 

| Method | Arguments | Returns | Dependencies | Level | Repeat|
|--------|-----------|---------|--------------|-------|-------|
| Get the recovery share for the users wallet. | | | [User initialization](./SDK%20API%20Reference.md#initializing-a-user) | Usage | Application |

=== "Rust"
    [get_recovery_share](../rust-docs/doc/etopay_sdk/core/struct.Sdk.html#method.get_recovery_share)

=== "Java"
    [getRecoveryShare](../javadoc/com/etospheres/etopay/ETOPaySdk.html#getRecoveryShare())

=== "Typescript"
    [getRecoveryShare](../jstsdocs/classes/ETOPaySdk.html#getRecoveryShare)

=== "Swift"
    Not available yet!

    ```swift
    public func getRecoveryShare() throws -> String
    ```

## Viviswap functions

### Start KYC Verification for viviswap

| Method | Arguments | Returns | Dependencies | Level | Repeat|
|--------|-----------|---------|--------------|-------|-------|
| Create a new viviswap user and initialize KYC verification | `mail` - The email address of the user, `terms_accepted` - A boolean indicating whether the terms have been accepted | Returns `NewViviswapUser` object if successful. | [User initialization](./SDK%20API%20Reference.md#initializing-a-user) | Usage | Application |

=== "Rust"
    [start_kyc_verification_for_viviswap](../rust-docs/doc/etopay_sdk/core/struct.Sdk.html#method.start_kyc_verification_for_viviswap)

=== "Java"
    [startViviswapKyc](../javadoc/com/etospheres/etopay/ETOPaySdk.html#startViviswapKyc(java.lang.String,boolean))

=== "Typescript"
    [startKycVerificationForViviswap](../jstsdocs/classes/ETOPaySdk.html#startKycVerificationForViviswap)

=== "Swift"
    Not available yet!

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

| Method | Arguments | Returns | Dependencies | Level | Repeat|
|--------|-----------|---------|--------------|-------|-------|
| Get current kyc status of viviswap | | Returns `ViviswapKycStatus` object if successful. | [User initialization](./SDK%20API%20Reference.md#initializing-a-user) | Usage | Application |

=== "Rust"
    [get_kyc_details_for_viviswap](../rust-docs/doc/etopay_sdk/core/struct.Sdk.html#method.get_kyc_details_for_viviswap)

=== "Java"
    [getViviswapKyc](../javadoc/com/etospheres/etopay/ETOPaySdk.html#getViviswapKyc())

=== "Typescript"
    [getKycDetailsForViviswap](../jstsdocs/classes/ETOPaySdk.html#getKycDetailsForViviswap)

=== "Swift"
    Not available yet!

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

| Method | Arguments | Returns | Dependencies | Level | Repeat|
|--------|-----------|---------|--------------|-------|-------|
| Update the kyc details for viviswap to be submitted | `is_individual` - Whether the user is an individual, `is_pep` - Whether the user is a politically exposed person, `is_us_citizen` - Whether the user is a US citizen, `is_regulatory_disclosure` - Whether the user has accepted the regulatory disclosure, `country_of_residence` - The country of residence of the user, `nationality` - The nationality of the user, `full_name` - The full name of the user, `date_of_birth` - The date of birth of the user | Returns `ViviswapPartiallyKycDetails` object containing the partially updated KYC details. | [User initialization](./SDK%20API%20Reference.md#initializing-a-user) | Usage | Application |

=== "Rust"
    [update_kyc_partially_status_for_viviswap](../rust-docs/doc/etopay_sdk/core/struct.Sdk.html#method.update_kyc_partially_status_for_viviswap)

=== "Java"
    [updateViviswapKycPartial](../javadoc/com/etospheres/etopay/ETOPaySdk.html#updateViviswapKycPartial(boolean,boolean,boolean,boolean,java.lang.String,java.lang.String,java.lang.String,java.lang.String))

=== "Typescript"
    [updateKycPartiallyStatusForViviswap](../jstsdocs/classes/ETOPaySdk.html#updateKycPartiallyStatusForViviswap)

=== "Swift"
    Not available yet!

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

| Method | Arguments | Returns | Dependencies | Level | Repeat|
|--------|-----------|---------|--------------|-------|-------|
| Submit the kyc details for viviswap | | | [User initialization](./SDK%20API%20Reference.md#initializing-a-user) | Usage | Application |

=== "Rust"
    [submit_kyc_partially_status_for_viviswap](../rust-docs/doc/etopay_sdk/core/struct.Sdk.html#method.submit_kyc_partially_status_for_viviswap)

=== "Java"
    [submitViviswapKycPartial](../javadoc/com/etospheres/etopay/ETOPaySdk.html#submitViviswapKycPartial())

=== "Typescript"
    [submitKycPartiallyStatusForViviswap](../jstsdocs/classes/ETOPaySdk.html#submitKycPartiallyStatusForViviswap)

=== "Swift"
    Not available yet!

    ```swift
    public func submitKycPartiallyStatusForViviswap() throws
    ```

### Get IBAN for viviswap

| Method | Arguments | Returns | Dependencies | Level | Repeat|
|--------|-----------|---------|--------------|-------|-------|
| Get current iban of viviswap user | | Returns `ViviswapAddressDetail` object if successful. | [Wallet initialization](./SDK%20API%20Reference.md#create-new-wallet), [Get KYC Details for viviswap](./SDK%20API%20Reference.md#get-kyc-details-for-viviswap) | Usage | Application |

=== "Rust"
    [get_iban_for_viviswap](../rust-docs/doc/etopay_sdk/core/struct.Sdk.html#method.get_iban_for_viviswap)

=== "Java"
    [getIbanViviswap](../javadoc/com/etospheres/etopay/ETOPaySdk.html#getIbanViviswap())

=== "Typescript"
    [getIbanViviswap](../jstsdocs/classes/ETOPaySdk.html#getIbanViviswap)

=== "Swift"
    Not available yet!

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

| Method | Arguments | Returns | Dependencies | Level | Repeat|
|--------|-----------|---------|--------------|-------|-------|
| Update IBAN of viviswap user | `pin` - The user's PIN, `address` - The new IBAN address | Returns `ViviswapAddressDetail` object if successful. | [Wallet initialization](./SDK%20API%20Reference.md#create-new-wallet), [Get KYC Details for viviswap](./SDK%20API%20Reference.md#get-kyc-details-for-viviswap) | Usage | Application |

=== "Rust"
    [update_iban_for_viviswap](../rust-docs/doc/etopay_sdk/core/struct.Sdk.html#method.update_iban_for_viviswap)

=== "Java"
    [updateIbanViviswap](../javadoc/com/etospheres/etopay/ETOPaySdk.html#updateIbanViviswap(java.lang.String,java.lang.String))

=== "Typescript"
    [updateIbanViviswap](../jstsdocs/classes/ETOPaySdk.html#updateIbanViviswap)

=== "Swift"
    Not available yet!

    ```swift
    public func updateIbanViviswap(pin: String, address: String) throws -> String
    ```

### Create deposit with viviswap

| Method | Arguments | Returns | Dependencies | Level | Repeat|
|--------|-----------|---------|--------------|-------|-------|
| Create deposit for viviswap user | `pin` - The PIN for the wallet | Returns `ViviswapDeposit` object if successful. | [Wallet initialization](./SDK%20API%20Reference.md#create-new-wallet), [Get KYC Details for viviswap](./SDK%20API%20Reference.md#get-kyc-details-for-viviswap), [Update IBAN](./SDK%20API%20Reference.md#update-iban-for-viviswap) | Usage | Application |

=== "Rust"
    [create_deposit_with_viviswap](../rust-docs/doc/etopay_sdk/core/struct.Sdk.html#method.create_deposit_with_viviswap)

=== "Java"
    [depositWithViviswap](../javadoc/com/etospheres/etopay/ETOPaySdk.html#depositWithViviswap(java.lang.String))

=== "Typescript"
    [createDepositWithViviswap](../jstsdocs/classes/ETOPaySdk.html#createDepositWithViviswap)

=== "Swift"
    Not available yet!

    ```swift
    public func depositWithViviswap(pin: String) throws -> String
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

| Method | Arguments | Returns | Dependencies | Level | Repeat|
|--------|-----------|---------|--------------|-------|-------|
| Create address detail for viviswap user | `pin` - The PIN for the wallet | Returns `ViviswapAddressDetail` object if successful. | [Wallet initialization](./SDK%20API%20Reference.md#create-new-wallet), [Get KYC Details for viviswap](./SDK%20API%20Reference.md#get-kyc-details-for-viviswap) | Usage | Application |

=== "Rust"
    [create_detail_for_viviswap](../rust-docs/doc/etopay_sdk/core/struct.Sdk.html#method.create_detail_for_viviswap)

=== "Java"
    [createViviswapDetail](../javadoc/com/etospheres/etopay/ETOPaySdk.html#createViviswapDetail(java.lang.String))

=== "Typescript"
    [createDetailForViviswap](../jstsdocs/classes/ETOPaySdk.html#createDetailForViviswap)

=== "Swift"
    Not available yet!

    ```swift
    public func createViviswapDetail(pin: String) throws -> String
    ```

### Create withdrawal with viviswap

| Method | Arguments | Returns | Dependencies | Level | Repeat|
|--------|-----------|---------|--------------|-------|-------|
| Create address detail for viviswap user | `amount` - The amount of the withdrawal, `pin` - The optional PIN for verification, `data` - Optional data which can be assigned to the transaction | Returns `ViviswapWithdrawal` object if successful. | [Wallet initialization](./SDK%20API%20Reference.md#create-new-wallet), [Get KYC Details for viviswap](./SDK%20API%20Reference.md#get-kyc-details-for-viviswap), [Update IBAN](./SDK%20API%20Reference.md#update-iban-for-viviswap) | Usage | Application |

=== "Rust"
    [create_withdrawal_with_viviswap](../rust-docs/doc/etopay_sdk/core/struct.Sdk.html#method.create_withdrawal_with_viviswap)

=== "Java"
    [withdrawWithViviswap](../javadoc/com/etospheres/etopay/ETOPaySdk.html#withdrawWithViviswap(double,java.lang.String,byte%5B%5D))

=== "Typescript"
    [createWithdrawalWithViviswap](../jstsdocs/classes/ETOPaySdk.html#createWithdrawalWithViviswap)

=== "Swift"
    Not available yet!

    ```swift
    public func withdrawWithViviswap(amount: Float, pin: String, data: [UInt8]) throws -> ViviswapWithdrawal
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

| Method | Arguments | Returns | Dependencies | Level | Repeat|
|--------|-----------|---------|--------------|-------|-------|
| Get swap details | `order_id` - The ID of the swap order. | Returns `Order` object containing the swap order details. | [Wallet initialization](./SDK%20API%20Reference.md#create-new-wallet), [Get KYC Details for viviswap](./SDK%20API%20Reference.md#get-kyc-details-for-viviswap) | Usage | Application |

=== "Rust"
    [get_swap_details](../rust-docs/doc/etopay_sdk/core/struct.Sdk.html#method.get_swap_details)

=== "Java"
    [getSwapDetails](../javadoc/com/etospheres/etopay/ETOPaySdk.html#getSwapDetails(java.lang.String))

=== "Typescript"
    [getSwapDetails](../jstsdocs/classes/ETOPaySdk.html#getSwapDetails)

=== "Swift"
    Not available yet!

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

| Method | Arguments | Returns | Dependencies | Level | Repeat|
|--------|-----------|---------|--------------|-------|-------|
| Get swap list | `start` - The start page parameter, `limit` - The pagination limit parameter | Returns an array of `Order` object containing the swap order details for each swap. | [Wallet initialization](./SDK%20API%20Reference.md#create-new-wallet), [Get KYC Details for viviswap](./SDK%20API%20Reference.md#get-kyc-details-for-viviswap) | Usage | Application |

=== "Rust"
    [get_swap_list](../rust-docs/doc/etopay_sdk/core/struct.Sdk.html#method.get_swap_list)

=== "Java"
    [getSwapList](../javadoc/com/etospheres/etopay/ETOPaySdk.html#getSwapList(long,long))

=== "Typescript"
    [getSwapList](../jstsdocs/classes/ETOPaySdk.html#getSwapList)

=== "Swift"
    Not available yet!

    ```swift
    public func getSwapList(page: UInt64, limit: UInt64) throws -> String
    ```

### Get exchange rate

| Method | Arguments | Returns | Dependencies | Level | Repeat|
|--------|-----------|---------|--------------|-------|-------|
| Get the exchange rate for the selected currency | | Returns the latest exchange rate | | Usage | Application |

=== "Rust"
    [get_exchange_rate](../rust-docs/doc/etopay_sdk/core/struct.Sdk.html#method.get_exchange_rate)

=== "Java"
    [getExchangeRate](../javadoc/com/etospheres/etopay/ETOPaySdk.html#getExchangeRate())

=== "Typescript"
    [getExchangeRate](../jstsdocs/classes/ETOPaySdk.html#getExchangeRate)

=== "Swift"
    Not available yet!

    ```swift
    public func getExchangeRate() throws -> Float
    ```

## Transaction functions

### Create purchase request

| Method | Arguments | Returns | Dependencies | Level | Repeat|
|--------|-----------|---------|--------------|-------|-------|
| Create purchase request | `receiver` - The receiver's username, `amount` - The amount of the purchase, `product_hash` - The hash of the product, `app_data` - The application data, `purchase_type` - The type of the purchase | Returns the purchase ID. This is an internal index used to reference the transaction in etopay | [Wallet initialization](./SDK%20API%20Reference.md#create-new-wallet) | Usage | Application |

=== "Rust"
    [create_purchase_request](../rust-docs/doc/etopay_sdk/core/struct.Sdk.html#method.create_purchase_request)

=== "Java"
    [purchaseRequestCreate](../javadoc/com/etospheres/etopay/ETOPaySdk.html#purchaseRequestCreate(java.lang.String,double,java.lang.String,java.lang.String,java.lang.String))

=== "Typescript"
    [createPurchaseRequest](../jstsdocs/classes/ETOPaySdk.html#createPurchaseRequest)

=== "Swift"
    Not available yet!

    ```swift
    public func createPurchaseRequest(
        receiver: String, amount: Double, product_hash: String, app_data: String, purchase_type: String
    ) throws -> String
    ```

### Get purchase details

| Method | Arguments | Returns | Dependencies | Level | Repeat|
|--------|-----------|---------|--------------|-------|-------|
| Get purchase details | `purchase_id` - The ID of the purchase. | Returns the purchase details as `PurchaseDetails` object | [Wallet initialization](./SDK%20API%20Reference.md#create-new-wallet) | Usage | Application |

=== "Rust"
    [get_purchase_details](../rust-docs/doc/etopay_sdk/core/struct.Sdk.html#method.get_purchase_details)

=== "Java"
    [purchaseDetails](../javadoc/com/etospheres/etopay/ETOPaySdk.html#purchaseDetails(java.lang.String))

=== "Typescript"
    [getPurchaseDetails](../jstsdocs/classes/ETOPaySdk.html#getPurchaseDetails)

=== "Swift"
    Not available yet!

    ```swift
    public func getPurchaseDetails(purchase_id: String) throws -> String
    ```

#### PurchaseDetails

=== "Rust"

    ```rust
        pub struct PurchaseDetails {
            /// The sender address where the fees goes to.
            pub system_address: String,
            /// The amount to be paid.
            pub amount: Decimal,
            /// The status of transaction
            pub status: ApiTxStatus,
        }
    ```

=== "JSON"

    ```json
    {
        "$schema": "http://json-schema.org/draft-07/schema#",
        "type": "object",
        "properties": {
            "system_address": {
                "type": "string",
                "description": "The sender address where the fees goes to."
            },
            "amount": {
                "type": "number",
                "description": "The amount to be paid"
            },
            "status": {
                "type": "object",
                "description": "Status of the transfer",
                "enum": ["Pending", "WaitingForVerification", "Valid", "Invalid", "ProcessingIncoming", "ProcessingOutgoing", "Completed", "Failed"]
            },
        },
        "required": ["system_address", "amount", "status"]
    }
    ```

### Confirm purchase request

| Method | Arguments | Returns | Dependencies | Level | Repeat|
|--------|-----------|---------|--------------|-------|-------|
| Confirm purchase request | `pin` - The PIN of the user, `purchase_id` - The ID of the purchase. | | [Wallet initialization](./SDK%20API%20Reference.md#create-new-wallet) | Usage | Application |

=== "Rust"
    [confirm_purchase_request](../rust-docs/doc/etopay_sdk/core/struct.Sdk.html#method.confirm_purchase_request)

=== "Java"
    [purchaseRequestConfirm](../javadoc/com/etospheres/etopay/ETOPaySdk.html#purchaseRequestConfirm(java.lang.String,java.lang.String))

=== "Typescript"
    [confirmPurchaseRequest](../jstsdocs/classes/ETOPaySdk.html#confirmPurchaseRequest)

=== "Swift"
    Not available yet!

    ```swift
    public func confirmPurchaseRequest(pin: String, purchase_id: String) throws
    ```

### Send amount

| Method | Arguments | Returns | Dependencies | Level | Repeat|
|--------|-----------|---------|--------------|-------|-------|
| Send amount to the receiver address. | `pin` - The PIN of the user, `address` - The receiver's address, `amount` - The amount to send, `data` - Optional data which can be assigned to the transaction | | [Wallet initialization](./SDK%20API%20Reference.md#create-new-wallet) | Usage | Application |

=== "Rust"
    [send_amount](../rust-docs/doc/etopay_sdk/core/struct.Sdk.html#method.send_amount)

=== "Java"
    [sendAmount](../javadoc/com/etospheres/etopay/ETOPaySdk.html#sendAmount(java.lang.String,java.lang.String,double,byte%5B%5D))

=== "Typescript"
    [sendAmount](../jstsdocs/classes/ETOPaySdk.html#sendAmount)

=== "Swift"
    Not available yet!

    ```swift
    public func sendAmount(pin: String, address: String, amount: Double, data: [UInt8]) throws
    ```

### Get Purchase list

| Method | Arguments | Returns | Dependencies | Level | Repeat|
|--------|-----------|---------|--------------|-------|-------|
| Get purchase list | `start` - The starting page number, `limit` - The maximum number of transactions per page | Returns a list of purchases as `TxInfo` object, if successful. | [User initialization](./SDK%20API%20Reference.md#initializing-a-user) | Usage | Application |

=== "Rust"
    [get_tx_list](../rust-docs/doc/etopay_sdk/core/struct.Sdk.html#method.get_tx_list)

=== "Java"
    [txList](../javadoc/com/etospheres/etopay/ETOPaySdk.html#txList(long,long))

=== "Typescript"
    [getTransactionList](../jstsdocs/classes/ETOPaySdk.html#getTransactionList)

=== "Swift"
    Not available yet!

    ```swift
    public func getTxList(start: UInt64, limit: UInt64) throws -> String
    ```

#### TxInfo

=== "Rust"

    ```Rust
    pub struct TxInfo {
        /// Tx creation date, if available
        pub date: Option<String>,
        /// receiver of the transaction
        pub receiver: String,
        /// etopay reference id for the transaction
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
                "description": "etopay reference id for the transaction"
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

| Method | Arguments | Returns | Dependencies | Level | Repeat|
|--------|-----------|---------|--------------|-------|-------|
| Start kyc verification for postident | | Returns an object `NewCaseIdResponse` if successful. | [User initialization](./SDK%20API%20Reference.md#initializing-a-user) | Usage | Application |

=== "Rust"
    [start_kyc_verification_for_postident](../rust-docs/doc/etopay_sdk/core/struct.Sdk.html#method.start_kyc_verification_for_postident)

=== "Java"
    [startKycVerificationForPostident](../javadoc/com/etospheres/etopay/ETOPaySdk.html#startKycVerificationForPostident())

=== "Typescript"
    [startKycVerificationForPostident](../jstsdocs/classes/ETOPaySdk.html#startKycVerificationForPostident)

=== "Swift"
    Not available yet!

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

| Method | Arguments | Returns | Dependencies | Level | Repeat|
|--------|-----------|---------|--------------|-------|-------|
| Get case details for postident| | Returns an object `CaseDetailsResponse` if successful. | [User initialization](./SDK%20API%20Reference.md#initializing-a-user) | Usage | Application |

=== "Rust"
    [get_kyc_details_for_postident](../rust-docs/doc/etopay_sdk/core/struct.Sdk.html#method.get_kyc_details_for_postident)

=== "Java"
    [getKycDetailsForPostident](../javadoc/com/etospheres/etopay/ETOPaySdk.html#getKycDetailsForPostident())

=== "Typescript"
    [getKycDetailsForPostident](../jstsdocs/classes/ETOPaySdk.html#getKycDetailsForPostident)

=== "Swift"
    Not available yet!

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

| Method | Arguments | Returns | Dependencies | Level | Repeat|
|--------|-----------|---------|--------------|-------|-------|
| Update case status for postident | `case_id`: The ID of the case to update. | | [User initialization](./SDK%20API%20Reference.md#initializing-a-user) | Usage | Application |

=== "Rust"
    [update_kyc_status_for_postident](../rust-docs/doc/etopay_sdk/core/struct.Sdk.html#method.update_kyc_status_for_postident)

=== "Java"
    [updateKycStatusForPostident](../javadoc/com/etospheres/etopay/ETOPaySdk.html#updateKycStatusForPostident(java.lang.String))

=== "Typescript"
    [updateKycStatusForPostident](../jstsdocs/classes/ETOPaySdk.html#updateKycStatusForPostident)

=== "Swift"
    Not available yet!

    ```swift
    public func updateKycStatusForPostident(case_id: String) throws
    ```

### Get user preferred network

| Method | Arguments | Returns | Dependencies | Level | Repeat|
|--------|-----------|---------|--------------|-------|-------|
| Fetches the users preferred network | | Returns the network if successful or an empty value if no preferred network has been set. | [User initialization](./SDK%20API%20Reference.md#initializing-a-user) | Usage | Application |

=== "Rust"
    [get_preferred_network](../rust-docs/doc/etopay_sdk/core/struct.Sdk.html#method.get_preferred_network)

=== "Java"
    [getPreferredNetwork](../javadoc/com/etospheres/etopay/ETOPaySdk.html#getPreferredNetwork())

=== "Typescript"
    [getPreferredNetwork](../jstsdocs/classes/ETOPaySdk.html#getPreferredNetwork)

=== "Swift"
    Not available yet!

    ```swift
    public func getPreferredNetwork() throws -> PreferredNetwork
    ```

### Set user preferred network

| Method | Arguments | Returns | Dependencies | Level | Repeat|
|--------|-----------|---------|--------------|-------|-------|
| Sets the users preferred network, or resets it if an empty value is provided. | `network_id` - The preferred user network. Optional value. | | [User initialization](./SDK%20API%20Reference.md#initializing-a-user) | Usage | Application |

=== "Rust"
    [set_preferred_network](../rust-docs/doc/etopay_sdk/core/struct.Sdk.html#method.set_preferred_network)

=== "Java"
    [setPreferredNetwork](../javadoc/com/etospheres/etopay/ETOPaySdk.html#setPreferredNetwork(java.lang.String))

=== "Typescript"
    [setPreferredNetwork](../jstsdocs/classes/ETOPaySdk.html#setPreferredNetwork)

=== "Swift"
    Not available yet!

    ```swift
    public func setPreferredNetwork(network_id: String) throws
    ```
