# Configuring the SDK

The Cawaena SDK needs to be configured correctly for secure and functional usage. Misconfiguration might lead to potential information leaks as well as bad end-user experience.

## Configuring authentication provider

The authentication provider is a setting which is configured in the beginning in conjunction with the Cawaena development team. The Cawaena SDK and backend support Oauth2.0/OpenID Connect[^1] provider and can work with external authentication providers. The following information is needed by the Cawaena team to configure the backend to accept requests from the SDK:

1. **ISSUER** - The issuer, which is part of the JWT claim in the `access_token`, created by the OAuth2.0 server. The issuer is generally the URL to the realm, but could also be different based on different settings.
2. **AUTHORITY** - The authority is where the public keys/certificates are hosted, which are used by the OAuth2.0 server to sign the JWT `access_tokens`. This is mostly a URL of the following type: `{base_url}/auth/realms/{realm_name}/protocol/openid-connect/certs`
3. **AZP** - The authorized party, which is part of the JWT claim in the `access_token`, created by the OAuth2.0 server. The authorized party is typically the name of the 3rd-party client, which has requested the JWT Token for the user using various flows listed in the standard.
4. **NAME** - A unique name to assign and identify this particular authentication provider settings in the backend as well as in the SDK.

???+ info

    The control of the client credentials, the flows used to fetch the JWT as well as the entire user management including user registration, email verification and user settings is out of scope for Cawaena backend and SDK. This should be managed by applications using the SDK themselves.

Once this information is provided and configured correctly in the backend by the Cawaena team, the SDK function `set_auth_provider` can be used as shown below to configure the correct authentication provider.

=== "Rust"

    ```rust linenums="1" hl_lines="12 13"
    async fn main() {
        dotenvy::dotenv().ok();

        // ensure a clean start
        tokio::fs::create_dir_all(path).await.unwrap();

        let username = std::env::var("SATOSHI_USERNAME").unwrap();

        let mut sdk = Sdk::default(); // (1)! 

        // Set the auth provider
        let auth_provider = std::env::var("AUTH_PROVIDER").unwrap();
        sdk.set_auth_provider(&auth_provider);
        sdk.validate_config().unwrap();

        // other SDK functions

    }
    ```

    1. This internally sets the path prefix or the storage path to the current working directory.
=== "Java"
    ```java linenums="1" hl_lines="25 29"

     package org.example.app;

    import com.etogruppe.CryptpaySdk;
    import java.nio.file.Files;
    import java.nio.file.Paths;
    import java.io.IOException;
    import java.util.Map;

    public class app {
        private CryptpaySdk sdk;
        public static void main(){
            sdk = new CryptpaySdk();
            String path = "/tmp/Cawaena";

            // ensure a clean start
            try {
                Files.createDirectories(Paths.get(path));
            } catch (IOException e) {
                e.printStackTrace();
            }

            Map<String, String> env = System.getenv();
            String username = env.get("SATOSHI_USERNAME");

            String authProvider = env.get("AUTH_PROVIDER");

            try {
                sdk.setStoragePath(path);
                sdk.authProvider(authProvider);
                sdk.logLevel("info");
                sdk.initLogger();
                
                sdk.checkConfig();
                // other SDK functions
            } catch (Exception e) {
                e.printStackTrace();
            }
        }
    }

    ```

=== "Swift"
    ```swift linenums="1" hl_lines="14 19"

    import Foundation

    let path = "/tmp/Cawaena"

    // ensure a clean start
    do {
        try FileManager.default.createDirectory(atPath: path, withIntermediateDirectories: true, attributes: nil)
    } catch {
        print(error.localizedDescription)
    }

    let username = ProcessInfo.processInfo.environment["SATOSHI_USERNAME"] ?? ""

    let auth_provider = ProcessInfo.processInfo.environment["AUTH_PROVIDER"] ?? ""

    let sdk = Sdk()
    sdk.setPathPrefix(path_prefix: path)
    sdk.setLogLevel(log_level: "info")
    sdk.setAuthProvider(auth_provider: auth_provider)
    do {
        try sdk.initLogger()
        try sdk.checkConfig() 
        // other SDK functions
    } catch {
        print(error.localizedDescription)
    }

    ```

Every time, the OAuth client refreshes or fetches a new access token for the user, the access token can be updated in the SDK using the `refresh_access_token`function.

=== "Rust"

    ```rust linenums="1" hl_lines="16-19"
    async fn main() {
        dotenvy::dotenv().ok();

        // ensure a clean start
        tokio::fs::create_dir_all(path).await.unwrap();

        let username = std::env::var("SATOSHI_USERNAME").unwrap();

        let mut sdk = Sdk::default();
        let auth_provider = std::env::var("AUTH_PROVIDER").unwrap();
        sdk.set_auth_provider(&auth_provider);
        sdk.validate_config().unwrap();

        // other SDK functions

        // Refreshing access token from environment
        // But it can also be directly given from the OAuth client
        let new_token = std::env::var("NEW_TOKEN").unwrap();
        sdk.refresh_access_token(&new_token);
    }
    ```

=== "Java"
    ```java linenums="1" hl_lines="36-39"

    package org.example.app;

    import com.etogruppe.CryptpaySdk;
    import java.nio.file.Files;
    import java.nio.file.Paths;
    import java.io.IOException;
    import java.util.Map;

    public class app {
        private CryptpaySdk sdk;
        public static void main(){
            sdk = new CryptpaySdk();
            String path = "/tmp/Cawaena";

            // ensure a clean start
            try {
                Files.createDirectories(Paths.get(path));
            } catch (IOException e) {
                e.printStackTrace();
            }

            Map<String, String> env = System.getenv();
            String username = env.get("SATOSHI_USERNAME");

            String authProvider = env.get("AUTH_PROVIDER");

            try {
                sdk.setStoragePath(path);
                sdk.authProvider(authProvider);
                sdk.logLevel("info");
                sdk.initLogger();
                
                sdk.checkConfig();
                // other SDK functions

                // Refreshing access token from environment
                // But it can also be directly given from the OAuth client
                let newToken = env.get("NEW_TOKEN").unwrap();
                sdk.refreshAccessToken(newToken);
            } catch (Exception e) {
                e.printStackTrace();
            }
        }
    }

    ```

=== "Swift"
    ```swift linenums="1" hl_lines="24-27"

    import Foundation

    let path = "/tmp/Cawaena"

    // ensure a clean start
    do {
        try FileManager.default.createDirectory(atPath: path, withIntermediateDirectories: true, attributes: nil)
    } catch {
        print(error.localizedDescription)
    }

    let username = ProcessInfo.processInfo.environment["SATOSHI_USERNAME"] ?? ""

    let auth_provider = ProcessInfo.processInfo.environment["AUTH_PROVIDER"] ?? ""

    let sdk = Sdk()
    sdk.setPathPrefix(path_prefix: path)
    sdk.setLogLevel(log_level: "info")
    sdk.setAuthProvider(auth_provider: auth_provider)
    do {
        try sdk.initLogger()
        try sdk.checkConfig() 
        // other SDK functions
        // Refreshing access token from environment
        // But it can also be directly given from the OAuth client
        let access_token = ProcessInfo.processInfo.environment["NEW_TOKEN"] ?? ""
        try sdk.refreshAccessToken(access_token: accessToken)
    } catch {
        print(error.localizedDescription)
    }

    ```

!!! warning

    The SDK is not responsible for refreshing the access token. Neither does it have the credentials, nor a way to obtain credentials for refreshing an access token for the user. This is the responsibility of the client application integrating the SDK. The <B>refresh_access_token</B> function should not be confused with <B>refresh_token</B> and should not be passed the value of <B>refresh_token</B>. The function needs the value of a valid <B>access_token</B> as a string.

## Configuring backend

The Cawaena team provides the URL for the backend, which can directly set using the `set_backend_url` function. This information is part of the initial setup and is important before starting the SDK usage.

=== "Rust"
    ```rust linenums="1" hl_lines="12-13"

    async fn main() {
        dotenvy::dotenv().ok();

        // ensure a clean start
        tokio::fs::create_dir_all(path).await.unwrap();

        let username = std::env::var("SATOSHI_USERNAME").unwrap();

        let mut sdk = Sdk::default();

        // Set the backend URL
        let url = std::env::var("URL").unwrap();
        sdk.set_backend_url(&url);
        sdk.validate_config().unwrap();

        // other SDK functions

    }

    ```

=== "Java"
    ```java linenums="1" hl_lines="25 30"

    package org.example.app;

    import com.etogruppe.CryptpaySdk;
    import java.nio.file.Files;
    import java.nio.file.Paths;
    import java.io.IOException;
    import java.util.Map;

    public class app {
        private CryptpaySdk sdk;
        public static void main(){
            sdk = new CryptpaySdk();
            String path = "/tmp/Cawaena";

            // ensure a clean start
            try {
                Files.createDirectories(Paths.get(path));
            } catch (IOException e) {
                e.printStackTrace();
            }

            Map<String, String> env = System.getenv();
            String username = env.get("SATOSHI_USERNAME");

            String url = env.get("URL");

            try {
                sdk.setStoragePath(path);
                sdk.logLevel("info");
                sdk.backendUrl(url);
                sdk.initLogger();
                sdk.checkConfig();
                // other SDK functions

            } catch (Exception e) {
                e.printStackTrace();
            }
        }
    }

    ```

=== "Swift"
    ```swift linenums="1" hl_lines="14 19"

    import Foundation

    let path = "/tmp/Cawaena"

    // ensure a clean start
    do {
        try FileManager.default.createDirectory(atPath: path, withIntermediateDirectories: true, attributes: nil)
    } catch {
        print(error.localizedDescription)
    }

    let username = ProcessInfo.processInfo.environment["SATOSHI_USERNAME"] ?? ""

    let url = ProcessInfo.processInfo.environment["URL"] ?? ""

    let sdk = Sdk()
    sdk.setPathPrefix(path_prefix: path)
    sdk.setLogLevel(log_level: "info")
    sdk.setBackendUrl(backend_url: url)
    do {
        try sdk.initLogger()
        try sdk.checkConfig() 
        // other SDK functions
    } catch {
        print(error.localizedDescription)
    }

    ```

## Configuring the storage path prefix

Depending on how the application is built, it is mandatory that the application has access to a certain file system, where it is allowed read and write files and directories and sub-directories. The storage path prefix can then be configured using the `set_path_prefix` function, which requires the top-level directory path to where the application can store data specific to the SDK. Both relative and absolute paths work, however, absolute paths are preferred.

This function is mandatory to be called, at least once, to enable creation of user and wallet, since it requires the SDK to create some files and fill them with relevant data for later usage. The default path prefix, if not set, is always the current working directory, from where the application is launched.

???+ tip

    It is recommended to use a path where only the application has file system rights. Allowing access to other applications is a potential security risk and may incur loss of funds for end-users.

=== "Rust"
    ```rust linenums="1" hl_lines="3 12"

    async fn main() {
        dotenvy::dotenv().ok();
        let path = "/tmp/Cawaena";

        // ensure a clean start
        tokio::fs::create_dir_all(path).await.unwrap();

        let username = std::env::var("SATOSHI_USERNAME").unwrap();

        let mut sdk = Sdk::default();
        sdk.set_env(Environment::Development);
        sdk.set_path_prefix(path);
        sdk.validate_config().unwrap();
    }

    ```
=== "Java"
    ```java linenums="1" hl_lines="13 26"

    package org.example.app;

    import com.etogruppe.CryptpaySdk;
    import java.nio.file.Files;
    import java.nio.file.Paths;
    import java.io.IOException;
    import java.util.Map;

    public class app {
        private CryptpaySdk sdk;
        public static void main(){
            sdk = new CryptpaySdk();
            String path = "/tmp/Cawaena"; // (1)!

            // ensure a clean start
            try {
                Files.createDirectories(Paths.get(path));
            } catch (IOException e) {
                e.printStackTrace();
            }

            Map<String, String> env = System.getenv();
            String username = env.get("SATOSHI_USERNAME");

            try {
                sdk.setStoragePath(path);
                sdk.logLevel("info");
                sdk.initLogger();
                sdk.checkConfig();
                // other SDK functions

            } catch (Exception e) {
                e.printStackTrace();
            }
        }
    }

    ```

    1. For use in Android applications, it is important to extract the path where the app has permissions to create files and directories and use it as the storage path. This is generally something like `\data\data\org.example.app\` if the application package is org.example.app

=== "Swift"
    ```swift linenums="1" hl_lines="3 15"

    import Foundation

    let path = "/tmp/Cawaena" // (1)!

    // ensure a clean start
    do {
        try FileManager.default.createDirectory(atPath: path, withIntermediateDirectories: true, attributes: nil)
    } catch {
        print(error.localizedDescription)
    }

    let username = ProcessInfo.processInfo.environment["SATOSHI_USERNAME"] ?? ""

    let sdk = Sdk()
    sdk.setPathPrefix(path_prefix: path)
    sdk.setLogLevel(log_level: "info")
    do {
        try sdk.initLogger()
        try sdk.checkConfig() 
        // other SDK functions
    } catch {
        print(error.localizedDescription)
    }

    ```

    1. For use in iOS applications, it is important to extract the path where the app has permissions to create files and directories and use it as the storage path.

## Configuring currency

The SDK requires a correct currency to be configured. The currently supported currencies are `smr` for Shimmer network and `iota` for the IOTA network. Configuration of the currency is required to correctly set-up the wallet.

Currently, the configuration of the coin type in [BIP-0032](https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki) derivation path and connecting to a custom node using a different URL is not supported in the SDK. However, these features will be added in the future to customize the SDK for any test tokens or customer specific blockchain nodes.

The currency configuration is linked with the environment configuration. By selecting the environment and the currency, the SDK internally connects automatically to the correct network. By default, on any environment except development, the SDK connects to the main network of the given currency.

The development environment is used only internally by the Cawaena development team for testing and should be avoided by application integrating the SDK. Pre-configured environments can be requested to the Cawaena development team, however, the function for setting the environment will be deprecated in the future releases.

=== "Rust"

    ```rust linenums="1" hl_lines="12"
    async fn main() {
        dotenvy::dotenv().ok();
        let path = "/tmp/Cawaena";

        // ensure a clean start
        tokio::fs::create_dir_all(path).await.unwrap();

        let username = std::env::var("SATOSHI_USERNAME").unwrap();

        sdk.set_path_prefix(path);
        sdk.set_env(Environment::Development);
        sdk.set_currency(Currency::Iota);
        sdk.validate_config().unwrap();

        // other SDK functions

    }
    ```

=== "Java"

    ```java linenums="1" hl_lines="30"

    package org.example.app;

    import com.etogruppe.CryptpaySdk;
    import java.nio.file.Files;
    import java.nio.file.Paths;
    import java.io.IOException;
    import java.util.Map;

    public class app {
        private CryptpaySdk sdk;
        public static void main(){
            sdk = new CryptpaySdk();
            String path = "/tmp/Cawaena"; 

            // ensure a clean start
            try {
                Files.createDirectories(Paths.get(path));
            } catch (IOException e) {
                e.printStackTrace();
            }

            Map<String, String> env = System.getenv();
            String username = env.get("SATOSHI_USERNAME");

            try {
                sdk.setStoragePath(path);
                sdk.logLevel("info");
                sdk.initLogger();
                sdk.sdkEnv(env.get("ENVIRONMENT"));
                sdk.setCurrency("SMR");
                sdk.checkConfig();
                // other SDK functions

            } catch (Exception e) {
                e.printStackTrace();
            }
        }
    }


    ```

=== "Swift"

    ```swift linenums="1" hl_lines="21"
    
    import Foundation

    let path = "/tmp/Cawaena"

    // ensure a clean start
    do {
        try FileManager.default.createDirectory(atPath: path, withIntermediateDirectories: true, attributes: nil)
    } catch {
        print(error.localizedDescription)
    }

    let username = ProcessInfo.processInfo.environment["SATOSHI_USERNAME"] ?? ""
    let environment = ProcessInfo.processInfo.environment["ENVIRONMENT"] ?? ""

    let sdk = Sdk()
    sdk.setPathPrefix(path_prefix: path)
    sdk.setLogLevel(log_level: "info")
    do {
        try sdk.initLogger()
        try sdk.setSdkEnv(sdk_env: environment)
        sdk.setCurrency("SMR");
        try sdk.checkConfig() 
        // other SDK functions
    } catch {
        print(error.localizedDescription)
    }

    ```

## Logging in the SDK and validating configuration

It is important and recommended to initialize the logger provided by the SDK. The SDK creates a log file internally which can be exported and analyzed during testing and integration. The path prefix should be set and the log level should be defined before the logger can be successfully initialized.

The log file with the name `cryptpay_sdk.log` is created by the SDK at the root location specified by the set path prefix. The logger will append the logs to the same file, if the file exists and will create a new file if it does not exist.

The different log levels that can be set for the logger are: `debug`, `info`, `warn`, `error`.

The SDK also provides a shorthand utility to validate the configuration, any time a configuration is set or modified. This utility does not guarantee that the configuration will always work, but rather provides certain kind of semantic validation and checks if certain preset conditions are met before the SDK module functions can be used.

???+ tip

    Use the SDK configuration validation function every time a new configuration is done in the SDK before calling any other SDK functions. This helps catch errors due to configuration during the validation and not while executing the module functions.

=== "Rust"
    ```rust linenums="1" hl_lines="11-13"

    async fn main() {
        dotenvy::dotenv().ok();
        let path = "/tmp/Cawaena";

        // ensure a clean start
        tokio::fs::create_dir_all(path).await.unwrap();


        let mut sdk = Sdk::default();
        sdk.set_path_prefix(path);
        sdk.set_log_level("info");
        sdk.init_logger().unwrap();
        sdk.validate_config().unwrap();
    }

    ```
=== "Java"
    ```java linenums="1" hl_lines="26-28"

    package org.example.app;

    import com.etogruppe.CryptpaySdk;
    import java.nio.file.Files;
    import java.nio.file.Paths;
    import java.io.IOException;
    import java.util.Map;

    public class app {
        private CryptpaySdk sdk;
        public static void main(){
            sdk = new CryptpaySdk();
            String path = "/tmp/Cawaena;

            // ensure a clean start
            try {
                Files.createDirectories(Paths.get(path));
            } catch (IOException e) {
                e.printStackTrace();
            }

            Map<String, String> env = System.getenv();

            try {
                sdk.setStoragePath(path);
                sdk.logLevel("info");
                sdk.initLogger();
                sdk.checkConfig();
                // other SDK functions

            } catch (Exception e) {
                e.printStackTrace();
            }
        }
    }

    ```

=== "Swift"
    ```swift linenums="1" hl_lines="14 16 17"

    import Foundation

    let path = "/tmp/Cawaena" 

    // ensure a clean start
    do {
        try FileManager.default.createDirectory(atPath: path, withIntermediateDirectories: true, attributes: nil)
    } catch {
        print(error.localizedDescription)
    }

    let sdk = Sdk()
    sdk.setPathPrefix(path_prefix: path)
    sdk.setLogLevel(log_level: "info")
    do {
        try sdk.initLogger()
        try sdk.checkConfig() 
        // other SDK functions
    } catch {
        print(error.localizedDescription)
    }

    ```

## Complete example

The following code snippet shows a complete example for correctly configuring the SDK before using any of its module functions. Exceptions from the SDK with misconfiguration can be caught early to avoid unncessary delays while integrating other module functions.

=== "Rust"
    ```rust linenums="1"

    async fn main() {
        dotenvy::dotenv().ok();
        let path = "/tmp/Cawaena";

        // ensure a clean start
        tokio::fs::create_dir_all(path).await.unwrap();

        let mut sdk = Sdk::default();

        // Set the auth provider
        let auth_provider = std::env::var("AUTH_PROVIDER").unwrap();
        sdk.set_auth_provider(&auth_provider);

        // Set the backend URL
        let url = std::env::var("URL").unwrap();
        sdk.set_backend_url(&url);

        // Set path prefix
        sdk.set_path_prefix(path);

        // Set environment and currency
        sdk.set_env(Environment::Development);
        sdk.set_currency(Currency::Iota);

        // Set log level and initialize logger
        sdk.set_log_level("info");
        sdk.init_logger().unwrap();

        // Validate configuration
        sdk.validate_config().unwrap();

    }
    ```
=== "Java"
    ```Java linenums="1"

    package org.example.app;

    import com.etogruppe.CryptpaySdk;
    import java.nio.file.Files;
    import java.nio.file.Paths;
    import java.io.IOException;
    import java.util.Map;

    public class app {
        private CryptpaySdk sdk;
        public static void main(){
            sdk = new CryptpaySdk();
            String path = "/tmp/Cawaena";

            // ensure a clean start
            try {
                Files.createDirectories(Paths.get(path));
            } catch (IOException e) {
                e.printStackTrace();
            }

            Map<String, String> env = System.getenv();

            try {
                // Set auth provider
                sdk.authProvider(env.get("AUTH_PROVIDER"););

                // Set backend URL
                sdk.backendUrl(env.get("URL"));

                // Set storage path (path prefix)
                sdk.setStoragePath(path);

                // Set environment and currency
                sdk.sdkEnv(env.get("ENVIRONMENT"));
                sdk.setCurrency("SMR");

                // Set log level and initialize logger
                sdk.logLevel("info");
                sdk.initLogger();
                
                // Validate configuration
                sdk.checkConfig();

            } catch (Exception e) {
                e.printStackTrace();
            }
        }
    }
    
    ```
=== "Swift"
    ```Swift linenums="1"

    import Foundation

    let path = "/tmp/Cawaena"

    // ensure a clean start
    do {
        try FileManager.default.createDirectory(atPath: path, withIntermediateDirectories: true, attributes: nil)
    } catch {
        print(error.localizedDescription)
    }

    let auth_provider = ProcessInfo.processInfo.environment["AUTH_PROVIDER"] ?? ""
    let url = ProcessInfo.processInfo.environment["URL"] ?? ""
    let environment = ProcessInfo.processInfo.environment["ENVIRONMENT"] ?? ""

    let sdk = Sdk()
    
    // Set auth provider
    sdk.setAuthProvider(auth_provider: auth_provider)

    // Set backend url
    sdk.setBackendUrl(backend_url: url)

    // Set path prefix
    sdk.setPathPrefix(path_prefix: path)
   
    do {
        // Set environment and currency
        try sdk.setSdkEnv(sdk_env: environment)
        try sdk.setCurrency(currency: "SMR") 

        // Set log level and initialize logger
        sdk.setLogLevel(log_level: "info")
        try sdk.initLogger()
        
        // validate configuration
        try sdk.checkConfig() 
    } catch {
        print(error.localizedDescription)
    }

    ```

[^1]:
    The following information links could be used as reference for OAuth2.0 and OpenID Connect

    [OAuth 2.0 Official Website](https://oauth.net/2/)

    [OAuth 2.0 RFC](https://datatracker.ietf.org/doc/html/rfc6749)

    [OAuth 2.0 Simplified](https://aaronparecki.com/oauth-2-simplified/)

    [OAuth 2.0 Playground](https://developers.google.com/oauthplayground)

    [OpenID Connect Official Website](https://openid.net/connect/)

    [OpenID Connect RFC](https://openid.net/specs/openid-connect-core-1_0.html)

    [OpenID Connect Playground](https://www.openidconnect.net/)

    [OpenID Connect Introduction](https://connect2id.com/learn/openid-connect)
