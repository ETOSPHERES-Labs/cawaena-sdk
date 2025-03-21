# Examples

The ETOPay SDK is built in `rust`. It is primarily an implementation of the various interfaces for managing users, wallets, on-boarding of users through KYC (Know Your Customer) processes, payment methods and listing usage information. The flows discussed in this document show examples using the rust language. For examples related to the specific language, refer to the corresponding pages.

The ETOPay SDK can be used only if the following pre-requisites have been fulfilled and the information resulting from these conditions is available:

- Oauth2.0 Identity and Access Management Provider is configured correctly,
- The backend base URL of the ETOPay system is known,
- The path to a certain file storage is available, where the application has read/write rights to create, modify and delete files related to the SDK like log files, local key-value DBs, and wallet files.

Once this information is available, the SDK can be instantiated and the basic functions can be used.

The examples shows the usage of the SDK in rust for creating a user. The user credentials are taken from the environment but could also be easily a user input.

The environment configuration to `Development` attaches the SDK to the development backend of ETOPay automatically. It also configures the authentication provider correctly with the one used by the development team internally. This configuration is used by ETOPay developers and is only restricted to the users controlled by the identity provider configured for ETOPay internal testing.

## 0. Shared Setup Code

=== "Rust"

    ```rust
    --8<-- "examples/utils.rs::53"
    ```

=== "Java"

    ```java
    --8<-- "bindings/android/tests/src/main/java/com/etogruppe/examples/utils.java"
    ```

=== "Swift"

    ```swift
    --8<-- "bindings/swift/examples/Sources/utils/utils.swift"
    ```

=== "JS/TS"

    ```typescript
    --8<-- "bindings/wasm/examples/utils.ts"
    ```

## 1. Create New User

=== "Rust"

    ```rust
    --8<-- "examples/01_create_new_user.rs"
    ```

=== "Java"

    ```java
    --8<-- "bindings/android/tests/src/main/java/com/etogruppe/examples/CreateNewUser01.java"
    ```

=== "Swift"

    ```swift
    --8<-- "bindings/swift/examples/Sources/01_create_new_user/main.swift"
    ```

=== "JS/TS"

    ```typescript
    --8<-- "bindings/wasm/examples/01-create_new_user.ts"
    ```

## 2. Onboard User Postident

=== "Rust"

    ```rust
    --8<-- "examples/02_onboard_user_postident.rs"
    ```

=== "Java"

    ```java
    --8<-- "bindings/android/tests/src/main/java/com/etogruppe/examples/OnboardUserPostident02.java"
    ```

=== "Swift"

    ```swift
    --8<-- "bindings/swift/examples/Sources/02_onboard_user_postident/main.swift"
    ```

=== "JS/TS"

    ```typescript
    --8<-- "bindings/wasm/examples/02-onboard_user_via_postident.ts"
    ```

## 4. Migrate Wallet From Mnemonic

=== "Rust"

    ```rust
    --8<-- "examples/04_migrate_wallet_from_mnemonic.rs"
    ```

=== "Java"

    ```java
    --8<-- "bindings/android/tests/src/main/java/com/etogruppe/examples/MigrateWalletFromMnemonic04.java"
    ```

=== "Swift"

    ```swift
    --8<-- "bindings/swift/examples/Sources/04_migrate_wallet_from_mnemonic/main.swift"
    ```

=== "JS/TS"

    ```typescript
    --8<-- "bindings/wasm/examples/04-migrate_wallet_from_mnemonic.ts"
    ```

## 5. Migrate Wallet From Backup

=== "Rust"

    ```rust
    --8<-- "examples/05_migrate_wallet_from_backup.rs"
    ```

=== "Java"

    ```java
    --8<-- "bindings/android/tests/src/main/java/com/etogruppe/examples/MigrateWalletFromBackup05.java"
    ```

=== "Swift"

    ```swift
    --8<-- "bindings/swift/examples/Sources/05_migrate_wallet_from_backup/main.swift"
    ```

=== "JS/TS"

    ```typescript
    --8<-- "bindings/wasm/examples/05-migrate_wallet_from_backup.ts"
    ```

## 6. Generate New Receiver Address

=== "Rust"

    ```rust
    --8<-- "examples/06_generate_new_address.rs"
    ```

=== "Java"

    ```java
    --8<-- "bindings/android/tests/src/main/java/com/etogruppe/examples/GenerateNewAddress06.java"
    ```

=== "Swift"

    ```swift
    --8<-- "bindings/swift/examples/Sources/06_generate_new_address/main.swift"
    ```

=== "JS/TS"

    ```typescript
    --8<-- "bindings/wasm/examples/06-generate_new_address.ts"
    ```

## 7. Get Balance

=== "Rust"

    ```rust
    --8<-- "examples/07_get_balance.rs"
    ```

=== "Java"

    ```java
    --8<-- "bindings/android/tests/src/main/java/com/etogruppe/examples/GetBalance07.java"
    ```

=== "Swift"

    ```swift
    --8<-- "bindings/swift/examples/Sources/07_get_balance/main.swift"
    ```

=== "JS/TS"

    ```typescript
    --8<-- "bindings/wasm/examples/07-get_balance.ts"
    ```

## 8. Create Purchase Request

=== "Rust"

    ```rust
    --8<-- "examples/08_create_purchase_request.rs"
    ```

=== "Java"

    ```java
    --8<-- "bindings/android/tests/src/main/java/com/etogruppe/examples/CreatePurchaseRequest08.java"
    ```

=== "Swift"

    ```swift
    --8<-- "bindings/swift/examples/Sources/08_create_purchase_request/main.swift"
    ```

=== "JS/TS"

    ```typescript
    --8<-- "bindings/wasm/examples/08-create_purchase_request.ts"
    ```

## 9. Onboard a User on Viviswap

=== "Rust"

    ```rust
    --8<-- "examples/09_onboard_user_viviswap.rs"
    ```

=== "Java"

    ```java
    --8<-- "bindings/android/tests/src/main/java/com/etogruppe/examples/OnboardUserViviswap09.java"
    ```

=== "Swift"

    ```swift
    --8<-- "bindings/swift/examples/Sources/09_onboard_user_viviswap/main.swift"
    ```

=== "JS/TS"

    ```typescript
    --8<-- "bindings/wasm/examples/09-onboard_user_via_viviswap.ts"
    ```

## 10. Verify Pin

=== "Rust"

    ```rust
    --8<-- "examples/10_verify_pin.rs"
    ```

=== "Java"

    ```java
    --8<-- "bindings/android/tests/src/main/java/com/etogruppe/examples/VerifyPin10.java"
    ```

=== "Swift"

    ```swift
    --8<-- "bindings/swift/examples/Sources/10_verify_pin/main.swift"
    ```

=== "JS/TS"

    ```typescript
    --8<-- "bindings/wasm/examples/10-verify_pin.ts"
    ```

## 11. Reset Pin

=== "Rust"

    ```rust
    --8<-- "examples/11_reset_pin.rs"
    ```

=== "Java"

    ```java
    --8<-- "bindings/android/tests/src/main/java/com/etogruppe/examples/ResetPin11.java"
    ```

=== "Swift"

    ```swift
    --8<-- "bindings/swift/examples/Sources/11_reset_pin/main.swift"
    ```

=== "JS/TS"

    ```typescript
    --8<-- "bindings/wasm/examples/11-reset_pin.ts"
    ```

## 12. Change Password

=== "Rust"

    ```rust
    --8<-- "examples/12_change_password.rs"
    ```

=== "Java"

    ```java
    --8<-- "bindings/android/tests/src/main/java/com/etogruppe/examples/ChangePassword12.java"
    ```

=== "Swift"

    ```swift
    --8<-- "bindings/swift/examples/Sources/12_change_password/main.swift"
    ```

=== "JS/TS"

    ```typescript
    --8<-- "bindings/wasm/examples/12-change_password.ts"
    ```

## 13. Send Amount

=== "Rust"

    ```rust
    --8<-- "examples/13_send_amount.rs"
    ```

=== "Java"

    ```java
    --8<-- "bindings/android/tests/src/main/java/com/etogruppe/examples/SendAmount13.java"
    ```

=== "Swift"

    ```swift
    --8<-- "bindings/swift/examples/Sources/13_send_amount/main.swift"
    ```

=== "JS/TS"

    ```typescript
    --8<-- "bindings/wasm/examples/13-send_amount.ts"
    ```

## 14. Get Exchange Rate

=== "Rust"

    ```rust
    --8<-- "examples/14_get_exchange_rate.rs"
    ```

=== "Java"

    ```java
    --8<-- "bindings/android/tests/src/main/java/com/etogruppe/examples/GetExchangeRate14.java"
    ```

=== "Swift"

    ```swift
    --8<-- "bindings/swift/examples/Sources/14_get_exchange_rate/main.swift"
    ```

=== "JS/TS"

    ```typescript
    --8<-- "bindings/wasm/examples/14-get_exchange_rate.ts"
    ```

## 16. Get Purchase List

=== "Rust"

    ```rust
    --8<-- "examples/16_get_tx_list.rs"
    ```

=== "Java"

    ```java
    --8<-- "bindings/android/tests/src/main/java/com/etogruppe/examples/GetTxList16.java"
    ```

=== "Swift"

    ```swift
    --8<-- "bindings/swift/examples/Sources/16_get_tx_list/main.swift"
    ```

=== "JS/TS"

    ```typescript
    --8<-- "bindings/wasm/examples/16-get_tx_list.ts"
    ```

## 18. Delete User

=== "Rust"

    ```rust
    --8<-- "examples/18_delete_user.rs"
    ```

=== "Java"

    ```java
    --8<-- "bindings/android/tests/src/main/java/com/etogruppe/examples/DeleteUser18.java"
    ```

=== "Swift"

    ```swift
    --8<-- "bindings/swift/examples/Sources/18_delete_user/main.swift"
    ```

=== "JS/TS"

    ```typescript
    --8<-- "bindings/wasm/examples/18-delete_user.ts"
    ```

## 19. Get Wallet Transaction List

=== "Rust"

    ```rust
    --8<-- "examples/19_get_wallet_tx_list.rs"
    ```

=== "Java"

    ```java
    --8<-- "bindings/android/tests/src/main/java/com/etogruppe/examples/GetWalletTxList19.java"
    ```

=== "Swift"

    ```swift
    --8<-- "bindings/swift/examples/Sources/19_get_wallet_tx_list/main.swift"
    ```

=== "JS/TS"

    ```typescript
    --8<-- "bindings/wasm/examples/19-get_wallet_tx_list.ts"
    ```

## 20. Send Compliment

=== "Rust"

    ```rust
    --8<-- "examples/20_send_compliment.rs"
    ```

=== "Java"

    ```java
    --8<-- "bindings/android/tests/src/main/java/com/etogruppe/examples/SendCompliment20.java"
    ```

=== "Swift"

    ```swift
    --8<-- "bindings/swift/examples/Sources/20_send_compliment/main.swift"
    ```

=== "JS/TS"

    ```typescript
    --8<-- "bindings/wasm/examples/20-send_compliment.ts"
    ```

## 22. Initialize Wallet from Shares
 
=== "Rust"

    ```rust
    --8<-- "examples/22_init_wallet_from_shares.rs"

    ```
