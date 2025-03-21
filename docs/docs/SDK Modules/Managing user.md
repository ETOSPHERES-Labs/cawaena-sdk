# User management

The SDK is designed to allow multiple users working with their own wallets on the same end devices sharing the same storage space. This makes it easy for a single person to have multiple alias users for different purposes and use different wallets for each of them to have a clear separation of risks.

The user initialization is done by two main operations in the SDK.

*Creating a new user* : This creates a new user in the in-memory database. All the properties of the user, like his selected KYC process, his KYC status, his access token for the backend, pin, encrypted password, etc... are set with the default values. A salt is generated for the user, which will be used later for encrypting the password.

*Initializing a user* : This function initializes the user for a new session. It also checks that a valid access token has been provided by updating the KYC status of the user from the backend in the SDK internal state.

## Creating a new user

User creation in the SDK is compulsory. This user is only a local user which might be already existing in the identity management provider.  

???+ note

    The user might already exist in the OAuth system, as well as every where else, including ETOPay backend. However, the SDK associates the local user to the system user only when an access token is provided.

The SDK supports multi-user environments and switching between users is fairly simple. Creating a user in the SDK informs the SDK about the user and allows the SDK to manage the user's state locally, whilst syncing it with the backend periodically.

This allows the SDK to be used across multiple devices, and ideally on the same device, on multiple storage path prefixes. This means, that changing the storage path prefix would result in the SDK unknowing the existence of the user and would require to create the user once again.

Creating a new user can be done using the `create_new_user` function which takes the `username` input parameter. Before creating a user, it is important that at least the storage path is set in the SDK.

The `username` should always match the `preferred_username` claim on the JWT `access_token`, otherwise the SDK would not be able to access the backend services for that user. Through this, the newly created SDK local user gets recognized in the system as a valid user.

???+ tip end

    The application can extract the `preferred_username` information automatically from the JWT claim and set the username directly, instead of asking the user to enter the input. A user might mistype or misunderstand and enter a username which might later not work. This would lead to a bad end-user experience and should be avoided.

!!! Note

    The code snippets provided are intended as pseudo-code to demonstrate logic and workflows. They are not guaranteed to compile, execute, or function as-is. Users should adapt and validate them according to their specific requirements and development environment.

=== "Rust"

    ```rust linenums="1"
    async fn main() {
        let mut sdk = Sdk::default();
        sdk.set_config(...).unwrap();

        sdk.create_new_user("username").await.unwrap();
    }
    ```

=== "Java"

    ```java linenums="1"
    package org.example.app;
    import com.etopay.Wallet;

    public class app {
        public static void main(){
            Wallet sdk = new Wallet();

            try {
                sdk.setConfig("...");
                sdk.createNewUser("username");
            } catch (Exception e) {
                e.printStackTrace();
            }
        }
    }
    ```

=== "Swift"

    ```swift linenums="1"
    import Foundation
    import ETOPaySdk

    let sdk = ETOPaySdk()
    try await sdk.setConfig(config: "...")

    do {
        try await sdk.createNewUser(username: "username")
    } catch {
        print(error.localizedDescription)
    }
    ```

=== "Typescript"

    ```typescript linenums="1"
    import * as wasm from "../pkg/etopay_sdk_wasm";

    const sdk = await new ETOPaySdk();
    await sdk.setConfig("...")

    await sdk.createNewUser("username");
    ```

## Initializing a user and access token refresh

The user is created and needs to be initialized before any state updates or wallet-related operations can be performed for this user. This allows the SDK to create multiple users and by using the initializing function, only the selected user is activated for the session. Without initializing a user, all operations related to the user would fail or conversely the previously initialized user's session will be used and might corrupt the state! To protect this from happening, before initializing the user, a corresponding access token is required. An invalid access token would result in failure of the initialization.

The access token brings the following safe operations for the SDK:

1. Only the correct user with the username would be initialized. Mismatch would cause an error.
2. The application can only initialize a user, only after the authorization of the actual person, since they would need to share their credentials for creating an access token.
3. Any user whose rights have been revoked, due to misuse reports, would not be able to use the system as the access token would be invalid and generating a new one would not also work.

!!! warning

    The user management is local to the end devices and deleting the application data, cache, temporary data files, etc... or changing the storage path prefix in the configuration would result in a loss of state and that would require the application to re-create and re-initialize user.

=== "Rust"

    ```rust linenums="1"
    async fn main() {
        let mut sdk = Sdk::default();
        sdk.set_config(...).unwrap();
        
        sdk.create_new_user("username").await.unwrap();

        sdk.refresh_access_token("access_token").await.unwrap();
        sdk.init_user("username").await.unwrap();

        // other SDK functions now use the initialized user
    }
    ```

=== "Java"

    ```java linenums="1"
    package org.example.app;
    import com.etopay.Wallet;

    public class app {
        public static void main(){
            Wallet sdk = new Wallet();

            try {
                sdk.setConfig("...");
                sdk.createNewUser("username");
                sdk.refreshAccessToken("accessToken");
                sdk.initializeUser("username");
                // other SDK functions now use the initialized user
            } catch (Exception e) {
                e.printStackTrace();
            }
        }
    }
    ```

=== "Swift"

    ```swift linenums="1"
    import Foundation
    import ETOPaySdk

    let sdk = ETOPaySdk()
    try await sdk.setConfig(config: "...")

    do {
        try await sdk.createNewUser(username: "username")
        try await sdk.refreshAccessToken(access_token: "access_token")
        try await sdk.initUser(username: "username")
        // other SDK functions now use the initialized user
    } catch {
        print(error.localizedDescription)
    }
    ```

=== "Typescript"

    ```typescript linenums="1"
    import * as wasm from "../pkg/etopay_sdk_wasm";

    const sdk = await new ETOPaySdk();
    await sdk.setConfig("...")

    await sdk.createNewUser("username");
    await sdk.refreshAccessToken("access_token");
    await sdk.initializeUser("username");
    // other SDK functions now use the initialized user
    ```

## Deleting a user

Deleting the user is simply deleting the user entity from the local database, while maintaining entries for other users. The delete user also calls the backend API to trigger an archiving action for the user. Deleting the user also deletes all the local data files for the user, which in this case are files related to the wallet. Since, this is a one-way operation a user is required to enter the pin, that they have set for the wallet. If there is no wallet setup, the pin can be skipped and the user is simply deleted locally and archived in the backend.

!!! danger

    Deleting a user not only deletes the user in the system but also deletes all local files and information from the device. This means, that the wallet is also deleted. Hence, a pin is used to verify if the user wishes to delete all this information. Deletion of a wallet without having a backup file or without the mnemonic is extremely dangerous as it can potentially lead to permanent loss of funds.

=== "Rust"

    ```rust linenums="1"
    async fn main() {
        let mut sdk = Sdk::default();
        sdk.set_config(...).unwrap();

        sdk.create_new_user("username").await.unwrap();
        sdk.refresh_access_token("access_token").await.unwrap();
        sdk.init_user("username").await.unwrap();

        let pin = "1234"; 
        // only if wallet was created by the user, a pin value is required.
        // otherwise, it is None.
        sdk.delete_user(Some(pin)).await.unwrap();
    }
    ```

=== "Java"

    ```java linenums="1"
    package org.example.app;
    import com.etopay.Wallet;

    public class app {
        public static void main(){
            Wallet sdk = new Wallet();

            try {
                sdk.setConfig("...");
                sdk.createNewUser("username");

                sdk.refreshAccessToken("accessToken");
                sdk.initializeUser("username");
                
                String pin = "1234"; 
                // only if wallet was created by the user, a pin value is required.
                // otherwise, it is null.
                sdk.deleteUser(pin)
            } catch (Exception e) {
                e.printStackTrace();
            }
        }
    }
    ```

=== "Swift"

    ```swift linenums="1"
    import Foundation
    import ETOPaySdk

    let sdk = ETOPaySdk()
    try await sdk.setConfig(config: "...")
    
    do {
        try await sdk.createNewUser(username: "username")

        try await sdk.refreshAccessToken(access_token: "access_token")
        try await sdk.initUser(username: "username")
        
        let pin = "1234"; 
        // only if wallet was created by the user, a pin value is required.
        // otherwise, it is nil.
        try await sdk.deleteUser(pin: pin)
    } catch {
        print(error.localizedDescription)
    }
    ```

=== "Typescript"

    ```typescript linenums="1"
    import * as wasm from "../pkg/etopay_sdk_wasm";

    const sdk = await new ETOPaySdk();
    await sdk.setConfig("...")

    await sdk.createNewUser("username");
    await sdk.refreshAccessToken("access_token");
    await sdk.initializeUser("username");

    let pin = "1234"; 
    // only if wallet was created by the user, a pin value is required.
    // otherwise, it is null.
    await sdk.deleteUser(pin);
    ```

## User lifecycle overview

          Username    Refresh access   Username              Pin              
             |            token          |                      |             
             |             |             |                      |             
             |             |             |                      |             
             |             |             |                      |             
        +----v---------+   |     +-------v------+        +------v---------+   
        |              |   |     |              |        |                |   
        |  Create      |   |     | Initialize   |        |     Delete     |   
        |  new         +---v-----> User         +-------->     User       |   
        |  User        |         |              |        |                |   
        +--------------+         +-----+--------+        +----------------+   
    Once                                |
    xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
    Multiple                            |
    Times                   +-----------v-----------------+
                            |  User           Wallet      |
                            |  State          Operations  |
                            |  Change                     |
                            +-----------------------------+
