# Wallet management

The SDK provides users with the opportunity to host their own wallets on their personal end-devices in a safe and easy manner. Before discussing wallet management, some information on wallets and what they are is needed to understand how to manage non-custodial hot wallets.

## Creating the wallet

The stronghold secret manager requires a file path for the wallet and a password to unlock this file. This password disables other applications from interpreting the files created by the stronghold engine and needs to come from the user.

The IOTA SDK offers an account manager structure which comprises of various fields to work with the wallet and the internal wallet accounts. The SDK creates a standard account `standalone` for its usage. There might be other accounts that could exist and are not operated upon by the SDK. The following ways can be used to create a wallet in the SDK:

### Create a new wallet

This does not require any user input except `username`,  `password` and `pin`. But, this should be a multi-step process. The created wallet returns a mnemonic. The app should immediately delete the wallet. In the second step the migration of the wallet with the mnemonic should be carried out and the wallet is only loaded with the mnemonic entered by the user. This approach protects the user against creating a wallet without never confirming the mnemonic back to the SDK and also by deleting a wallet, the SDK can ensure that there is actually no wallet created whose mnemonic was never entered from outside the application. This forces applications to have their end-users the mnemonic either memorized or input from a copy.

???+ info

    A fresh wallet can be created by a random seed, using the stronghold secret manager. It needs the password and username. The username is part of the file path and helps distinguish across different user wallets on the same end device. It returns the mnemonic, and this needs to be securely stored by the user, otherwise access to the funds on the wallet addresses would get limited. A node url for the DLT network can also be selected. Currently, the PoW is set to local, however it might change based on the used node url and its support for PoW.

=== "Rust"

    ```rust linenums="1"
    async fn main() {
        dotenvy::dotenv().ok();
        let path = "/tmp/Cawaena";

        // ensure a clean start
        tokio::fs::create_dir_all(path).await.unwrap();

        let password = std::env::var("SATOSHI_PASSWORD").unwrap();
        let pin = std::env::var("SATOSHI_PIN").unwrap();

        // Create and initialize the user
        //...

        let mnemonic = sdk.create_new_wallet(&pin, &password).await.unwrap();
        sdk.verify_mnemonic(&pin, &password, &mnemonic).await.unwrap();

        // If no exception is thrown, the mnemonic was verified...
        // now the wallet can be initialized and used...

    }
    ```

=== "Java"

    ```java linenums="1"

    package org.example.app;

    import com.cawaena.Wallet;
    import java.nio.file.Files;
    import java.nio.file.Paths;
    import java.io.IOException;
    import java.util.Map;

    public class app {
        private Wallet sdk;
        public static void main(){
            // Create and initialize the user
            //...

            try {
                String password = env.get("SATOSHI_PASSWORD");
                String pin = env.get("SATOSHI_PIN");
                
                String mnemonic = "";
                mnemonic = sdk.createNewWallet(pin, password);
                // return the mnemonic to the user to verify it

                sdk.verMnemonic(pin, password, mnemonic); 
                // If no exception is thrown, the mnemonic was verified...
                // now the wallet can be initialized and used...

            } catch (Exception e) {
                e.printStackTrace();
            }
        }
    }


    ```

=== "Swift"

    ```swift linenums="1"
    // Create and initialize the user
    //...
    do {
        let password = ProcessInfo.processInfo.environment["SATOSHI_PASSWORD"] ?? ""
        let pin = ProcessInfo.processInfo.environment["SATOSHI_PIN"] ?? ""

        var mnemonic = ""
        mnemonic = try sdk.createNewWallet(pin: pin, password: password)
        try sdk.verifyMnemonic(pin: pin, password: password, mnemonic: mnemonic)
        // If no exception is thrown, the mnemonic was verified...
        // now the wallet can be initialized and used..

    } catch {
        print(error.localizedDescription)
    }

    ```

### Migrate an existing wallet

This just performs the second step of the create fresh wallet process and needs in addition to the `mnemonic` also the `username`, `password` and `pin`.

=== "Rust"

    ```rust linenums="1"
    async fn main() {
        dotenvy::dotenv().ok();
        let path = "/tmp/Cawaena";

        // ensure a clean start
        tokio::fs::create_dir_all(path).await.unwrap();

        let password = std::env::var("SATOSHI_PASSWORD").unwrap();
        let pin = std::env::var("SATOSHI_PIN").unwrap();
        let mnemonic = ""; // User should enter their mnemonic

        // Create and initialize the user
        //...

        sdk.create_wallet_from_mnemonic(&pin, &password,&mnemonic).await.unwrap();

        // If no exception is thrown, the mnemonic was verified...
        // now the wallet can be initialized and used...

    }
    ```

=== "Java"

    ```java linenums="1"

    package org.example.app;

    import com.cawaena.Wallet;
    import java.nio.file.Files;
    import java.nio.file.Paths;
    import java.io.IOException;
    import java.util.Map;

    public class app {
        private Wallet sdk;
        public static void main(){
            // Create and initialize the user
            //...

            try {
                String password = env.get("SATOSHI_PASSWORD");
                String pin = env.get("SATOSHI_PIN");
                
                String mnemonic = ""; // User should enter their mnemonic
                sdk.createWalletFromMnemonic(pin, password, mnemonic);
                
                // If no exception is thrown, the mnemonic was verified...
                // now the wallet can be initialized and used...

            } catch (Exception e) {
                e.printStackTrace();
            }
        }
    }


    ```

=== "Swift"

    ```swift linenums="1"
    // Create and initialize the user
    //...
    do {
        let password = ProcessInfo.processInfo.environment["SATOSHI_PASSWORD"] ?? ""
        let pin = ProcessInfo.processInfo.environment["SATOSHI_PIN"] ?? ""

        let mnemonic = "" // User should enter their mnemonic
        try sdk.createWalletFromMnemonic(pin: pin, password: password, mnemonic: mnemonic)
        // If no exception is thrown, the mnemonic was verified...
        // now the wallet can be initialized and used..

    } catch {
        print(error.localizedDescription)
    }

    ```

### From a wallet backup file

If the user has created a backup from any other devices or wallet applications using the stronghold file format, this file can be used to restore the wallet by creating a new wallet from the backup file. All existing accounts as well as mnemonic information are restored.

=== "Rust"

    ```rust linenums="1"
    async fn main() {
        dotenvy::dotenv().ok();
        let path = "/tmp/Cawaena";

        // ensure a clean start
        tokio::fs::create_dir_all(path).await.unwrap();

        let password = std::env::var("SATOSHI_PASSWORD").unwrap(); // This is the password used while creating backup!
        let pin = std::env::var("SATOSHI_PIN").unwrap();
        let backup_path = ""; // The path to the backup file

        // Create and initialize the user
        //...

        sdk.create_wallet_from_backup(&pin, &password,&backup_path).await.unwrap();

        // If no exception is thrown, the backup was successfully restored...
        // now the wallet can be initialized and used...

    }
    ```

=== "Java"

    ```java linenums="1"

    package org.example.app;

    import com.cawaena.Wallet;
    import java.nio.file.Files;
    import java.nio.file.Paths;
    import java.io.IOException;
    import java.util.Map;

    public class app {
        private Wallet sdk;
        public static void main(){
            // Create and initialize the user
            //...

            try {
                String password = env.get("SATOSHI_PASSWORD"); // This is the password used while creating backup!
                String pin = env.get("SATOSHI_PIN");
                
                String backup_path = ""; // The path to the backup file
                sdk.createWalletFromBackup(pin, password, backup_path);
                
                // If no exception is thrown, the backup was successfully restored...
                // now the wallet can be initialized and used...

            } catch (Exception e) {
                e.printStackTrace();
            }
        }
    }


    ```

=== "Swift"

    ```swift linenums="1"
    // Create and initialize the user
    //...
    do {
        let password = ProcessInfo.processInfo.environment["SATOSHI_PASSWORD"] ?? "" // This is the password used while creating backup!
        let pin = ProcessInfo.processInfo.environment["SATOSHI_PIN"] ?? ""

        let backup_path = "" // The path to the backup file
        try sdk.createWalletFromBackup(pin: pin, password: password, backup_path: backup_path)
        // If no exception is thrown, the backup was successfully restored...
        // now the wallet can be initialized and used...

    } catch {
        print(error.localizedDescription)
    }

    ```

This restores an existing wallet from a backup file. It requires the `backup path` of the file as well the `backup password` in addition to the `password` of the backup and the new `pin` to be used.

## Creating a wallet backup

The SDK also provides function to create a stronghold backup file. This file is generated and stored in the path defined as `{path_prefix}/backups/{coin_type}/{username}/{Unix date time in seconds}.stronghold`. This file can be given to the user to download the backup and securely store it outside the application. The same backup file can be used to restore the wallet on other devices as well as in other application supporting stronghold files.

A `password` is also required to create the backup and the same `password` is required for restoring the backup. The backup can be created only if a wallet exists an it is successfully initialized for use. Uninitialized wallets need to be initialized before creating their backups.

=== "Rust"

    ```rust linenums="1"
    async fn main() {
        dotenvy::dotenv().ok();
        let path = "/tmp/Cawaena";

        // ensure a clean start
        tokio::fs::create_dir_all(path).await.unwrap();

        let backup_password = std::env::var("SATOSHI_BACKUP_PASSWORD").unwrap(); // This is the password used while creating backup!
        // Create and initialize the user
        // Create the wallet
        // Initialize the wallet
        let path = sdk.create_wallet_backup(&backup_password).await.unwrap();

        // the function also returns the path to the backup file

    }
    ```

=== "Java"

    ```java linenums="1"

    package org.example.app;

    import com.cawaena.Wallet;
    import java.nio.file.Files;
    import java.nio.file.Paths;
    import java.io.IOException;
    import java.util.Map;

    public class app {
        private Wallet sdk;
        public static void main(){
            // Create and initialize the user
            // Create the wallet
            // Initialize the wallet

            try {
                String backup_password = env.get("BACKUP_PASSWORD"); // This is the password used while creating backup!
                
                String backup_path = sdk.createWalletBackup(backup_password);
                
                // the function also returns the path to the backup file

            } catch (Exception e) {
                e.printStackTrace();
            }
        }
    }


    ```

=== "Swift"

    ```swift linenums="1"
    // Create and initialize the user
    // Create the wallet
    // Initialize the wallet
    do {
        let backup_password = ProcessInfo.processInfo.environment["SATOSHI_BACKUP_PASSWORD"] ?? "" // This is the password used while creating backup!

        let backup_path = try sdk.createWalletBackup(password: backup_password)
        // the function also returns the path to the backup file

    } catch {
        print(error.localizedDescription)
    }

    ```

=== "Rust"

    ```rust linenums="1"
    async fn main() {
        dotenvy::dotenv().ok();
        let path = "/tmp/Cawaena";

        // ensure a clean start
        tokio::fs::create_dir_all(path).await.unwrap();

        let backup_password = std::env::var("SATOSHI_BACKUP_PASSWORD").unwrap(); // This is the password used while creating backup!
        // Create and initialize the user
        // Create the wallet
        // Initialize the wallet
        let path = sdk.create_wallet_backup(&backup_password).await.unwrap();

        // the function also returns the path to the backup file

    }
    ```

=== "Java"

    ```java linenums="1"

    package org.example.app;

    import com.cawaena.Wallet;
    import java.nio.file.Files;
    import java.nio.file.Paths;
    import java.io.IOException;
    import java.util.Map;

    public class app {
        private Wallet sdk;
        public static void main(){
            // Create and initialize the user
            // Create the wallet
            // Initialize the wallet

            try {
                String backup_password = env.get("BACKUP_PASSWORD"); // This is the password used while creating backup!
                
                String backup_path = sdk.createWalletBackup(backup_password);
                
                // the function also returns the path to the backup file

            } catch (Exception e) {
                e.printStackTrace();
            }
        }
    }


    ```

=== "Swift"

    ```swift linenums="1"
    // Create and initialize the user
    // Create the wallet
    // Initialize the wallet
    do {
        let backup_password = ProcessInfo.processInfo.environment["SATOSHI_BACKUP_PASSWORD"] ?? "" // This is the password used while creating backup!

        let backup_path = try sdk.createWalletBackup(password: backup_password)
        // the function also returns the path to the backup file

    } catch {
        print(error.localizedDescription)
    }

    ```

## Initializing the wallet

For performing any operations involving the wallet, like fetching wallet balance, or send a transfer or generating a new address, the wallet needs to be initialized. The initialization also requires a `pin` entry from the application user, guaranteeing that the wallet cannot be misused.

=== "Rust"

    ```rust linenums="1"
    async fn main() {
        dotenvy::dotenv().ok();
        let path = "/tmp/Cawaena";

        // ensure a clean start
        tokio::fs::create_dir_all(path).await.unwrap();

        // Initialize the wallet
        let pin = "1234"; // User enters the pin
        sdk.init_wallet(&pin).await.unwrap();

        // Now all wallet functions can be called

    }
    ```

=== "Java"

    ```java linenums="1"

    package org.example.app;

    import com.cawaena.Wallet;
    import java.nio.file.Files;
    import java.nio.file.Paths;
    import java.io.IOException;
    import java.util.Map;

    public class app {
        private Wallet sdk;
        public static void main(){
            // Initialize the wallet

            try {
                String pin = "1234"; // User enters the pin
                
                sdk.initializeWallet(pin);
                
                // Now all wallet functions can be called

            } catch (Exception e) {
                e.printStackTrace();
            }
        }
    }


    ```

=== "Swift"

    ```swift linenums="1"
    // Initialize the wallet
    do {
        
        let pin = "1234"; // User enters the pin
        try sdk.initWallet(pin: pin)

        // Now all wallet functions can be called


    } catch {
        print(error.localizedDescription)
    }

    ```

## Deleting the wallet

This function just deletes the wallet files and is a one-way function, to be used under extreme caution, as it could result in permanent loss of funds. Note that, similar to any other wallet operation, deleting the wallet is also a wallet operation and requires the wallet to be correctly initialized. Without initialization, the deletion of wallet would fail.

=== "Rust"

    ```rust linenums="1"
    async fn main() {
        dotenvy::dotenv().ok();
        let path = "/tmp/Cawaena";

        // ensure a clean start
        tokio::fs::create_dir_all(path).await.unwrap();

        // Initialize the wallet
        let pin = "1234"; // User enters the pin
        sdk.init_wallet(&pin).await.unwrap();

        // Only initialized wallets can be deleted
        sdk.delete_wallet();

    }
    ```

=== "Java"

    ```java linenums="1"

    package org.example.app;

    import com.cawaena.Wallet;
    import java.nio.file.Files;
    import java.nio.file.Paths;
    import java.io.IOException;
    import java.util.Map;

    public class app {
        private Wallet sdk;
        public static void main(){
            // Initialize the wallet

            try {
                String pin = "1234"; // User enters the pin
                
                sdk.initializeWallet(pin);
                
                // Only initialized wallets can be deleted
                sdk.deleteWallet();

            } catch (Exception e) {
                e.printStackTrace();
            }
        }
    }


    ```

=== "Swift"

    ```swift linenums="1"
    // Initialize the wallet
    do {
        
        let pin = "1234"; // User enters the pin
        try sdk.initWallet(pin: pin)

        // Only initialized wallets can be deleted
        try sdk.deleteWallet()


    } catch {
        print(error.localizedDescription)
    }

    ```

## Password and pin utilities

In addition to creating, migrating, backups and initialization, the wallet module also performs auxiliary operations for pin and password management. It supports function to reset the pin using password, verify the pin, or change the wallet password using the current password and pin. Wallet initialization is again a pre-requisite, since the pin and password operations are related to the wallet and can only be performed once a wallet is initialized successfully.

=== "Rust"

    ```rust linenums="1"
    async fn main() {
        dotenvy::dotenv().ok();
        let path = "/tmp/Cawaena";

        // ensure a clean start
        tokio::fs::create_dir_all(path).await.unwrap();

        // Initialize the wallet
        let pin = "1234"; // User enters the pin
        sdk.init_wallet(&pin).await.unwrap();

        // Try to verify the pin
        sdk.verify_pin(&pin);
        let new_pin = "1235";
        let password = "StrongP@55w0rd";
        // or reset the pin to a new one
        sdk.reset_pin(&password,&new_pin)
        // or change the password
        let new_password = "StrongP@55W0rd";
        sdk.change_password(&pin,&password,&new_password);
    }
    ```

=== "Java"

    ```java linenums="1"

    package org.example.app;

    import com.cawaena.Wallet;
    import java.nio.file.Files;
    import java.nio.file.Paths;
    import java.io.IOException;
    import java.util.Map;

    public class app {
        private Wallet sdk;
        public static void main(){
            // Initialize the wallet

            try {
                String pin = "1234"; // User enters the pin
                
                sdk.initializeWallet(pin);
                
                // Try to verify the pin
                sdk.pinVerify(pin);
                String new_pin = "1235";
                String password = "StrongP@55w0rd";
                // or reset the pin to a new one
                sdk.pinReset(password, new_pin);
                // or change the password
                String new_password = "StrongP@55W0rd";
                sdk.passwordChange(pin, password, new_password);

            } catch (Exception e) {
                e.printStackTrace();
            }
        }
    }


    ```

=== "Swift"

    ```swift linenums="1"
    // Initialize the wallet
    do {
        
        let pin = "1234"; // User enters the pin
        try sdk.initWallet(pin: pin)

        // Try to verify the pin
        try sdk.verifyPin(pin: pin)
        let new_pin = "1235";
        let password = "StrongP@55w0rd";
        // or reset the pin to a new one
        try sdk.resetPin(password: password, new_pin: new_pin)
        // or change the password
        let new_password = "StrongP@55W0rd";
        try sdk.changePassword(pin: pin, current_password: password, new_password: new_password)

    } catch {
        print(error.localizedDescription)
    }

    ```

## Wallet flows

```
                Mnemonic  Username  Password  Pin  backup password         Pin                            Pin        
                   | ^         |      |       |      |                     |                              |          
                   | |         |      |       |      |                     |                              |          
                   | |         |      |       |      |                     |                              |          
                   | |         |      |       |      |                     |                              |          
                   | |         |      |       |      |                     |                              |          
                   | |         |      |       |      |                     |                              |          
                   | |         |      |       |      |                     |                              |          
    Inputs         v |         v      v       v      v                     v                              v          
-------------------------------------------------------------------------------------------------------------------  
                                                                |                           |                        
                                           +-------------+      |                           |                        
                                           |             |      |                           |                        
                                           | Create      |      |      +-------------+      |                        
                                           | New         +---+  |      |             |      |        +--------------+
                                           | Wallet      |   |  |      |  Initialize |      |        |              |
                                           |             |   |  |      |  Wallet     +------+-------->   Delete     |
                                           +-------------+   |  +------>             |      |        |   wallet     |
                                                             |  |      |             |      |        |   (external) |
                                                             |  |      +------+------+      |        |              |
                         +----------+      +-------------+   |  |             |             |        +--------------+
                         |          |      |             |   |  |             |             |                        
                         | Mnemonic |      | Migrate     |   |  |             |             |                        
                 +------->          +------> Existing    |   |  |             |             |                        
                 |       |          |      | Wallet      |   |  |      +------v------+      |                        
                 |       |          |      |             |   |  |      |             |      |                        
                 |       +----------+      +-------------+   |  |      |  User       |      |                        
                 |                                           |  |      |  Wallet     |      |                        
                 |                                           |  |      |  Functions  |      |                        
                 |           +------+      +-------------+   |  |      |             |      |                        
                 |           |      |      |             |   |  |      +-------------+      |                        
                 |           |Backup|      | Create      |   |  |                           |                        
                 |           | File +------> Wallet      |   |  |                           |                        
                 |           |      |      | From        |   |  |                           |                        
                 |           |      |      | Backup      |   |  |                           |                        
                 |           +------+      +-------------+   |  |                           |                        
                 |                                           |  |                           |                        
                 |                                           |  |                           |                        
                 |    +-------------+      +-------------+   |  |                           |                        
                 |    |             |      |             |   |  |                           |                        
                 |    | Verify      |      |  Delete     |   |  |                           |                        
                 +----+ Mnemonic    <------+  wallet     <---+  |                           |                        
                      |             |      | (internal)  |      |                           |                        
                      |             |      |             |      |                           |                        
                      +-------------+      +-------------+      |                           |                        
                                                                |                           |                        
                                                                |                           |                        
                                             Once(setup)        |        Multiple times     |       Once (tear down) 
                                                                |                           |                        
```
